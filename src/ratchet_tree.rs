use crate::ciphersuites::Ciphersuite;
use crate::tree::{self, NodeContent, Tree};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Node {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    unmerged_leaves: Vec<u128>, // ordered | TODO: maybe add reference to nodes, is faster but also needs lifetimes
    credential: Option<Vec<u8>>, // Leaf nodes only
    parent_hash: Vec<u8>,
    blank: bool, // if true, nothing else is set
    subtree_hash: Vec<u8>,
}

impl ToString for Node {
    fn to_string(&self) -> String {
        "Ratchet Tree Node".to_owned()
    }
}

impl NodeContent for Node {
    fn is_blank(&self) -> bool {
        self.blank
    }
    fn get_unmerged_leaves(&self) -> &[u128] {
        &self.unmerged_leaves
    }
}

// impl Node {
//     // The resolution of a node is an ordered list of non-blank nodes that
//     // collectively cover all non-blank descendants of the node.
//     fn resolution<'a>(&'a self, tree: &'a RatchetTree, node: &TreeNode) -> Result<Vec<&'a Node>, String> {
//         // The resolution of a blank leaf node is the empty list
//         if self.blank && self.leaf {
//             return Ok(Vec::new());
//         }

//         // The resolution of a blank intermediate node
//         if self.blank {
//             return vec![self.left(tree).resolution(tree), self.right(tree).resolution(tree)];
//         }

//         // The resolution of a non-blank node
//         debug_assert!(!self.blank);
//         let mut resolution = vec![self];

//         // Add all unmerged leaves
//         for &unmerged in self.unmerged_leaves.iter() {
//             resolution.push(tree.get_node(unmerged)?);
//         }

//         Ok(resolution)
//     }
// }

#[derive(Debug, Default)]
pub struct TreeConfig {
    cipher_suite: Ciphersuite,
}

pub struct RatchetTree(Tree<Node, TreeConfig>);

// impl RatchetTree {
//     fn get_node(&self, id: NodeId) -> Result<&Node, String> {
//         for node in self.nodes.iter() {
//             if node.id == id {
//                 return Ok(node);
//             }
//         }
//         Err(format!("Didn't find node with id {:?}", id))
//     }
// }
