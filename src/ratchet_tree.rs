use crate::tree::{self, Tree, NodeContent};

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Node {
    private_key: Vec<u8>,
    public_key: Vec<u8>,
    unmurged_leafs: Vec<u8>, // ordered
    credential: Vec<u8>, // Leaf nodes only
    parent_hash: Vec<u8>,
    blank: bool, // if true, nothing else is set
}

impl ToString for Node {
    fn to_string(&self) -> String {
        "Ratchet Tree Node".to_owned()
    }
}

impl NodeContent for Node {}
