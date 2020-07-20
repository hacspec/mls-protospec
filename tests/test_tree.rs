use mls_protospec::tree::Tree;

#[test]
fn test_root() {
    let mut tree = Tree::new();
    for i in 65..77 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    assert_eq!(Ok(tree.get_root()), tree.get_node(15));
}
