///! A binary tree structure
use crate::util;

#[derive(Clone, Copy, Debug, PartialEq)]
enum NodeType {
    Root,
    Intermediate, // XXX: Also used for root atm.
    Leaf,
    Unspecified,
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::Unspecified
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    InvalidNodeId,
    NodeIdTooFarTooTheRight,
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
    id_ctr: u128,
    leaf_id_ctr: u128,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Node {
    val: String, // make generic
    id: u128,
    leaf_id: u128, // Only used if node_type is Leaf
    node_type: NodeType,
}

impl Node {
    pub fn new(v: &String) -> Self {
        Self {
            val: v.clone(),
            id: u128::MAX,
            leaf_id: u128::MAX,
            node_type: NodeType::Leaf,
        }
    }
    pub fn get_val(&self) -> &String {
        &self.val
    }
    fn is_leaf(&self) -> bool {
        self.node_type == NodeType::Leaf
    }
    fn get_level(&self) -> u128 {
        util::level(self.id)
    }
    fn get_left_id(&self) -> u128 {
        debug_assert!(self.node_type != NodeType::Leaf);
        util::left(self.id)
    }
    fn get_right_id(&self) -> u128 {
        debug_assert!(self.node_type != NodeType::Leaf);
        util::right(self.id)
    }
    fn get_parent_id(&self) -> u128 {
        util::parent(self.id)
    }
    fn get_sibling_id(&self) -> u128 {
        util::sibling(self.id)
    }
    fn get_direct_path_ids(&self, root: &Self) -> Vec<u128> {
        util::direct_path(self.id, root.id)
    }
    fn get_co_path_ids(&self, root: &Self) -> Vec<u128> {
        let mut direct_path = vec![self.id];
        direct_path.extend(self.get_direct_path_ids(root));
        direct_path.pop();
        direct_path.iter().map(|&v| util::sibling(v)).collect()
    }
}

impl Tree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            id_ctr: 0,
            leaf_id_ctr: 0,
        }
    }
    pub fn get_height(&self) -> u128 {
        util::log2(self.id_ctr)
    }
    pub fn num_nodes(&self) -> u128 {
        util::num_nodes(self.leaf_id_ctr)
    }
    pub fn get_node(&self, id: u128) -> Result<&Node, Error> {
        if id > self.id_ctr {
            return Err(Error::InvalidNodeId);
        }
        match self.nodes.get(id as usize) {
            Some(n) => Ok(n),
            None => Err(Error::InvalidNodeId),
        }
    }
    pub fn get_parent(&self, node_id: u128) -> Result<&Node, Error> {
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
    pub fn get_sibling(&self, node_id: u128) -> Result<&Node, Error> {
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
    pub fn get_direct_path(&self, node_id: u128) -> Result<Vec<&Node>, Error> {
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
        debug_assert_eq!(id as u128, out.id);
        out
    }
    pub fn add(&mut self, val: &String) {
        if self.id_ctr % 2 != 0 {
            // We have to add an intermediate node before we can add the leaf.
            self.nodes.push(Node {
                val: "I".to_owned(),
                id: self.id_ctr,
                leaf_id: u128::MAX,
                node_type: NodeType::Intermediate,
            });
            self.id_ctr += 1;
        }

        // Now add the leaf.
        self.nodes.push(Node {
            val: val.clone(),
            id: self.id_ctr,
            leaf_id: self.leaf_id_ctr,
            node_type: NodeType::Leaf,
        });
        self.leaf_id_ctr += 1;
        self.id_ctr += 1;
    }

    fn get_level(&self, level: u128) -> Vec<&Node> {
        let mut out = Vec::new();

        for node in &self.nodes {
            if node.get_level() == level {
                out.push(node);
            }
        }

        out
    }

    pub fn to_str(&self) -> String {
        let mut s = "".to_owned();
        fn space(n: usize) -> String {
            let mut s = "".to_owned();
            for _ in 0..n {
                s += " ";
            }
            s
        }

        let n_leafs = self.id_ctr / 2;
        let height = self.get_height();
        let spacer = 7;
        for i in (0..height + 1).rev() {
            let n_half = if height - i != 0 {
                (n_leafs / (2 * (height - i))) as usize
            } else {
                (n_leafs / 2) as usize
            };
            let factor = if n_half * 2 < i as usize && n_half != 0 {
                n_half + 1
            } else {
                n_half
            };
            let pad = if i == 0 {
                0
            } else {
                spacer / 2 + 2 + factor * (3 + spacer)
            };
            s += &(i.to_string() + "|" + &space(pad));
            for (j, node) in self.get_level(i).iter().enumerate() {
                let id = if node.is_leaf() {
                    node.id.to_string()
                } else {
                    node.id.to_string()
                };
                s += &(node.get_val().to_owned() + "_" + &id + &space(spacer));
            }
            s += "\n";
        }

        s
    }
}

#[test]
fn test_get_node() {
    let mut tree = Tree::new();
    for i in 65..76 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    for i in 0..21 {
        assert_eq!(i, tree.get_node(i).unwrap().id);
        assert_eq!(i, tree.nodes.get(i as usize).unwrap().id);
    }
    assert_eq!(Err(Error::InvalidNodeId), tree.get_node(21));
}

#[test]
fn test_left() {
    let mut tree = Tree::new();
    for i in 65..77 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    assert_eq!(7, tree.get_root().get_left_id());
    assert_eq!(
        3,
        tree.get_node(tree.get_root().get_left_id())
            .unwrap()
            .get_left_id()
    );
    assert_eq!(1, tree.get_node(3).unwrap().get_left_id());
    assert_eq!(0, tree.get_node(1).unwrap().get_left_id());
    assert_eq!(16, tree.get_node(17).unwrap().get_left_id());
}

#[test]
fn test_right() {
    let mut tree = Tree::new();
    for i in 65..76 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    // Note that this node doesn't exist!
    assert_eq!(23, tree.get_root().get_right_id());
    assert_eq!(11, tree.get_node(7).unwrap().get_right_id());
    assert_eq!(18, tree.get_node(17).unwrap().get_right_id());
    assert_eq!(14, tree.get_node(13).unwrap().get_right_id());
    assert_eq!(5, tree.get_node(3).unwrap().get_right_id());
}

#[test]
fn test_parent() {
    let mut tree = Tree::new();
    for i in 65..79 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    // Note that this node doesn't exist!
    assert_eq!(31, tree.get_root().get_parent_id());
    assert_eq!(15, tree.get_node(7).unwrap().get_parent_id());
    assert_eq!(7, tree.get_node(3).unwrap().get_parent_id());
    assert_eq!(5, tree.get_node(4).unwrap().get_parent_id());
    assert_eq!(15, tree.get_node(23).unwrap().get_parent_id());
}

#[test]
fn test_direct_path() {
    let mut tree = Tree::new();
    for i in 65..79 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
    }
    fn check(id: u128, expected: &Vec<u128>, tree: &Tree) {
        let path = tree.get_direct_path(id).unwrap();
        let path_ids: Vec<u128> = path.iter().map(|n| n.id).collect();
        assert_eq!(expected, &path_ids);
    }
    check(0, &vec![1, 3, 7, 15], &tree);
    check(2, &vec![1, 3, 7, 15], &tree);
    check(4, &vec![5, 3, 7, 15], &tree);
    check(6, &vec![5, 3, 7, 15], &tree);
    check(8, &vec![9, 11, 7, 15], &tree);

    // Querrying a path that doesn't exist throws an error.
    match tree.get_direct_path(26) {
        Ok(_) => panic!("There should be no valid path here!"),
        Err(e) => assert_eq!(e, Error::InvalidNodeId),
    }
}

#[test]
fn simple_tree() {
    let mut tree = Tree::new();
    for i in 65..79 {
        tree.add(&String::from_utf8(vec![i]).unwrap());
        // println!("{:?}", &tree);
        println!("\npretty print tree:\n{}", tree.to_str());
    }

    // let path = tree.find_empty_path();
    // println!("path: {:?}", path);

    // tree.add(Node::new("B"));
    // tree.add(Node::new("C"));

    // println!("\n\n{:?}", &tree);

    // tree.add(Node::new("D"));

    // println!("\n\n{:?}", &tree);
    // println!("\npretty print tree:\n {}", tree.to_str());
}
