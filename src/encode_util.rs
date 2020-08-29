use crate::key_package::KeyPackage;

// From Section 7.5.
// Only used for encoding
pub(crate) struct ParentNode {
    hpke_public_key: Vec<u8>,
    unmerged_leaves: Vec<u32>,
    parent_hash: Vec<u8>,
}

// From Section 7.5.
// Only used for encoding
pub(crate) struct ParentNodeHashInput {
    node_index: u32,
    parent_node: Option<ParentNode>,
    left_hash: Vec<u8>,
    right_hash: Vec<u8>,
}

pub(crate) struct LeafNodeHashInput {
    node_index: u32,
    key_package: Option<KeyPackage>
}

// struct {
//     uint32 node_index;
//     optional<KeyPackage> key_package;
// } LeafNodeHashInput;

pub(crate) fn encode_parent_node(parent_node_in: &ParentNodeHashInput) {
    unimplemented!();
}
