use crate::{key_package::KeyPackage, util};
use std::fmt::Debug;

#[derive(Default, Debug)]
pub struct Node {
    pub(crate) id: u32,
    pub(crate) leaf_id: Option<u32>, // Only used if node_type is Leaf
    pub(crate) node_type: NodeType,
    pub(crate) private_key: Vec<u8>,
    pub(crate) public_key: Vec<u8>,
    pub(crate) key_package: Option<KeyPackage>,
    pub(crate) unmerged_leaves: Vec<u32>, // ordered | TODO: maybe add reference to nodes, is faster but also needs lifetimes
    pub(crate) credential: Option<Vec<u8>>, // Leaf nodes only
    pub(crate) parent_hash: Vec<u8>,
    pub(crate) blank: bool, // if true, nothing else is set
    pub(crate) subtree_hash: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum NodeType {
    Parent,
    Leaf,
    Unspecified,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Unspecified
    }
}

impl Node {
    pub(crate) fn new(node_type: NodeType, id: u32, leaf_id: Option<u32>) -> Self {
        Self {
            id,
            leaf_id,
            node_type,
            private_key: Vec::new(),
            public_key: Vec::new(),
            key_package: None,
            unmerged_leaves: Vec::new(),
            credential: None,
            parent_hash: Vec::new(),
            blank: true,
            subtree_hash: Vec::new(),
        }
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }
    pub(crate) fn get_level(&self) -> u32 {
        util::level(self.id)
    }
    pub(crate) fn get_left_id(&self) -> u32 {
        debug_assert!(self.node_type != NodeType::Leaf);
        util::left(self.id)
    }
    pub(crate) fn get_right_id(&self) -> u32 {
        debug_assert!(self.node_type != NodeType::Leaf);
        util::right(self.id)
    }
    pub(crate) fn get_parent_id(&self) -> u32 {
        util::parent(self.id)
    }
    pub(crate) fn get_sibling_id(&self) -> u32 {
        util::sibling(self.id)
    }
    pub(crate) fn get_direct_path_ids(&self, root: &Self) -> Vec<u32> {
        util::direct_path(self.id, root.id)
    }
    pub(crate) fn get_co_path_ids(&self, root: &Self) -> Vec<u32> {
        let mut direct_path = vec![self.id];
        direct_path.extend(self.get_direct_path_ids(root));
        direct_path.pop();
        direct_path.iter().map(|&v| util::sibling(v)).collect()
    }
}
