#![allow(dead_code)]
//! A binary tree structure

use crate::ciphersuites::*;
use crate::{util, traits::Encode};
use evercrypt::prelude::*;

mod tree_hash;
use tree_hash::*;

mod node;
use node::*;

mod pretty_print;

use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    InvalidNodeId,
    NodeIdTooFarTooTheRight,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    id_ctr: u32,
    leaf_id_ctr: u32,
    ciphersuite: Ciphersuite,
}

impl Tree {
    pub fn new(csuite: Name) -> Self {
        Self {
            nodes: Vec::new(),
            id_ctr: 0,
            leaf_id_ctr: 0,
            ciphersuite: Ciphersuite::from(csuite),
        }
    }
    pub fn get_height(&self) -> u32 {
        util::log2(self.id_ctr)
    }
    pub fn num_nodes(&self) -> u32 {
        util::num_nodes(self.leaf_id_ctr)
    }
    pub fn get_node(&self, id: u32) -> Result<&Node, Error> {
        if id > self.id_ctr {
            return Err(Error::InvalidNodeId);
        }
        match self.nodes.get(id as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_leaf_node(&self, leaf_id: u32) -> Result<&Node, Error> {
        self.get_node(2*leaf_id)
    }
    pub fn get_parent(&self, node_id: u32) -> Result<&Node, Error> {
        let node = match self.nodes.get(node_id as usize) {
            Some(n) => n,
            None => return Err(Error::InvalidNodeId),
        };
        self.get_parent_from_node(node)
    }
    pub fn get_parent_from_node(&self, node: &Node) -> Result<&Node, Error> {
        match self.nodes.get(node.get_parent_id() as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_sibling(&self, node_id: u32) -> Result<&Node, Error> {
        let node = match self.nodes.get(node_id as usize) {
            Some(n) => n,
            None => return Err(Error::InvalidNodeId),
        };
        self.get_sibling_from_node(node)
    }
    pub fn get_sibling_from_node(&self, node: &Node) -> Result<&Node, Error> {
        match self.nodes.get(node.get_sibling_id() as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_left_child(&self, node_id: u32) -> Result<&Node, Error> {
        let node = match self.nodes.get(node_id as usize) {
            Some(n) => n,
            None => return Err(Error::InvalidNodeId),
        };
        self.get_left_child_from_node(node)
    }
    pub fn get_left_child_from_node(&self, node: &Node) -> Result<&Node, Error> {
        match self.nodes.get(node.get_left_id() as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_right_child(&self, node_id: u32) -> Result<&Node, Error> {
        let node = match self.nodes.get(node_id as usize) {
            Some(n) => n,
            None => return Err(Error::InvalidNodeId),
        };
        self.get_right_child_from_node(node)
    }
    pub fn get_right_child_from_node(&self, node: &Node) -> Result<&Node, Error> {
        match self.nodes.get(node.get_right_id() as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_direct_path(&self, node_id: u32) -> Result<Vec<&Node>, Error> {
        let node = self.get_node(node_id)?;
        self.get_direct_path_from_node(node)
    }
    pub fn get_direct_path_from_node(&self, node: &Node) -> Result<Vec<&Node>, Error> {
        let path_ids = node.get_direct_path_ids(self.get_root());
        let mut nodes = Vec::new();
        for id in path_ids {
            let node = self.get_node(id)?;
            nodes.push(node);
        }
        Ok(nodes)
    }
    pub fn get_root(&self) -> &Node {
        let id = (1 << util::log2(self.num_nodes())) - 1;
        let out = self
            .nodes
            .get(id)
            .expect("Couldn't get the node with ID I was looking for.");
        debug_assert_eq!(id as u32, out.id);
        out
    }
    pub fn add_leaf(&mut self) {
        if self.id_ctr % 2 != 0 {
            // We add an intermediate node before we add the leaf.
            self.nodes
                .push(Node::new(NodeType::Parent, self.id_ctr, None));
            self.id_ctr += 1;
        }

        // Now add the leaf.
        self.nodes.push(Node::new(
            NodeType::Leaf,
            self.id_ctr,
            Some(self.leaf_id_ctr),
        ));
        self.leaf_id_ctr += 1;
        self.id_ctr += 1;
    }

    fn get_level(&self, level: u32) -> Vec<&Node> {
        let mut out = Vec::new();

        for node in &self.nodes {
            if node.get_level() == level {
                out.push(node);
            }
        }

        out
    }

    // Section 5.2
    // The resolution of a node is an ordered list of non-blank nodes that
    // collectively cover all non-blank descendants of the node.
    fn resolution(&self, node_id: u32) -> Result<Vec<&Node>, Error> {
        let node = self.get_node(node_id)?;

        if node.blank {
            if node.node_type == NodeType::Leaf {
                // The resolution of a blank leaf node is the empty list
                return Ok(Vec::new());
            }
            // The resolution of a blank intermediate node
            let mut left = self.resolution(self.get_left_child_from_node(node)?.id)?;
            let mut right = self.resolution(self.get_right_child_from_node(node)?.id)?;
            left.append(&mut right);
            return Ok(left);
        }

        // The resolution of a non-blank node
        debug_assert!(!node.blank);
        let mut resolution = vec![node];

        // Add all unmerged leaves
        for &unmerged in node.unmerged_leaves.iter() {
            resolution.push(self.get_node(unmerged)?);
        }

        Ok(resolution)
    }

    fn hash_parent(&self, node: &Node) -> Result<Vec<u8>, Error> {
        let left_hash = self.hash(node.get_left_id())?;
        let right_hash = self.hash(node.get_right_id())?;
        let parent_node = if self.get_root().id == node.id {
            None
        } else {
            Some(ParentNode::new(
                &node.public_key,
                &node.unmerged_leaves,
                &node.parent_hash,
            ))
        };
        let input = ParentNodeHashInput::new(node.id, parent_node, left_hash, right_hash);
        Ok(hash(self.ciphersuite.hash, &input.encode()))
    }
    fn hash_leaf(&self, node: &Node) -> Result<Vec<u8>, Error> {
        let input = LeafNodeHashInput::new(node.id, node.key_package.as_ref());
        Ok(hash(self.ciphersuite.hash, &input.encode()))
    }

    // Section 7.5 Tree Hash
    pub fn hash(&self, node_id: u32) -> Result<Vec<u8>, Error> {
        let node = self.get_node(node_id)?;
        self.hash_node(node)
    }
    pub fn hash_node(&self, node: &Node) -> Result<Vec<u8>, Error> {
        match node.node_type {
            NodeType::Parent => self.hash_parent(node),
            NodeType::Leaf => self.hash_leaf(node),
            _ => panic!("This shouldn't happen ..."),
        }
    }
}
