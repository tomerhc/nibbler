use std::cell::RefCell;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node {
    pub Id: &'static str,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id
    }
}
impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.Id.cmp(&other.Id)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.Id.hash(state);
    }
}

impl Node {
    pub fn new(id: &'static str) -> Self {
        Node { Id: id }
    }
}

pub type RcNode = Rc<Node>;

#[derive(Clone, Debug)]
pub struct SimpleGraph {
    pub data: HashMap<RcNode, Vec<RcNode>>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        SimpleGraph {
            data: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let rcnode: RcNode = Rc::new(node);
        self.data.entry(rcnode).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, source: Node, target: Node) {
        let rcsource = Rc::new(source);
        let rctarget = Rc::new(target);

        let entry_source = self.data.entry(Rc::clone(&rcsource)).or_insert(Vec::new());
        match entry_source.binary_search(&rctarget) {
            Ok(_) => (),
            Err(ind) => entry_source.insert(ind, Rc::clone(&rctarget)),
        }

        let entry_target = self.data.entry(Rc::clone(&rctarget)).or_insert(Vec::new());
        match entry_target.binary_search(&rcsource) {
            Ok(_) => (),
            Err(ind) => entry_target.insert(ind, Rc::clone(&rcsource)),
        }
    }

    pub fn from_edge_list(edge_list: Vec<(Node, Node)>) -> Self {
        let mut G = Self::new();
        for (source, target) in edge_list.into_iter() {
            G.add_edge(source, target);
        }
        G
    }

    pub fn from_node_list(mut node_list: Vec<Node>) -> Self {
        let mut G = SimpleGraph::new();
        node_list.sort_unstable();
        node_list.dedup();
        for n in node_list.into_iter() {
            G.add_node(n);
        }
        G
    }

    pub fn neighbors(&self, node: RcNode) -> Option<Vec<RcNode>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    pub fn has_node(&self, node: RcNode) -> bool {
        self.data.contains_key(&node)
    }

    //pub fn has_edge(&self, source: RcNode, target: RcNode) -> bool {
    //    match self.data.get(&source) {
    //        Some(v) =>
    //    }
    //}
}
