use mls_protospec::prelude::*;

const DEFAULT_CIPHERSUITE: CiphersuiteName =
    CiphersuiteName::MLS10_128_DHKEMX25519_AES128GCM_SHA256_Ed25519;

#[test]
fn test_tree_hash() {
    let mut tree = Tree::new(DEFAULT_CIPHERSUITE);

    // Add 5 leaves to the tree.
    for _ in 0..5 {
        tree.add_leaf();
    }
    println!("Tree:\n{}", tree);

    // Check some tree properties.
    assert_eq!(tree.get_height(), 3);
    assert_eq!(tree.num_nodes(), 9);

    // Compute hash for all leaves first
    for i in 0..5 {
        let leaf_i = tree.get_leaf_node(i).unwrap();
        let hash = tree.hash_node(leaf_i).unwrap();
        println!("Hash of {}: {:?}", leaf_i, hash);
    }

    // Compute tree hash (hash of root node)
    let root = tree.get_root();
    let tree_hash = tree.hash_node(root);
    println!("Tree hash: {:?}", tree_hash);
}
