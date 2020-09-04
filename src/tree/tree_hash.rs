use crate::encode_util::*;
use crate::{key_package::KeyPackage, traits::Encode};

// From Section 7.5.
// Only used for encoding
pub(crate) struct ParentNode<'a> {
    hpke_public_key: &'a [u8],
    unmerged_leaves: &'a [u32],
    parent_hash: &'a [u8],
}

impl<'a> ParentNode<'a> {
    pub(crate) fn new(
        hpke_public_key: &'a [u8],
        unmerged_leaves: &'a [u32],
        parent_hash: &'a [u8],
    ) -> Self {
        Self {
            hpke_public_key,
            unmerged_leaves,
            parent_hash,
        }
    }
}

impl<'a> Encode for ParentNode<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        encode_slice(LenType::L16, &self.hpke_public_key, &mut out);
        encode_u32_slice(LenType::L32, &self.unmerged_leaves, &mut out);
        out
    }
}

// From Section 7.5.
// Only used for encoding
pub(crate) struct ParentNodeHashInput<'a> {
    node_index: u32,
    parent_node: Option<ParentNode<'a>>,
    left_hash: Vec<u8>,
    right_hash: Vec<u8>,
}

impl<'a> ParentNodeHashInput<'a> {
    pub(crate) fn new(
        node_index: u32,
        parent_node: Option<ParentNode<'a>>,
        left_hash: Vec<u8>,
        right_hash: Vec<u8>,
    ) -> Self {
        Self {
            node_index,
            parent_node,
            left_hash,
            right_hash,
        }
    }
}

impl<'a> Encode for ParentNodeHashInput<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(&self.node_index.to_be_bytes());
        out.extend(self.parent_node.encode());
        out
    }
}

pub(crate) struct LeafNodeHashInput<'a> {
    node_index: u32,
    key_package: Option<&'a KeyPackage>,
}

impl<'a> LeafNodeHashInput<'a> {
    pub(crate) fn new(node_index: u32, key_package: Option<&'a KeyPackage>) -> Self {
        Self {
            node_index,
            key_package,
        }
    }
}

impl<'a> Encode for LeafNodeHashInput<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(&self.node_index.to_be_bytes());
        out.extend(self.key_package.encode());
        out
    }
}
