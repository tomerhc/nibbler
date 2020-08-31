use std::cell::RefCell;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct Node {
    pub id: &'static str,
    pub data: HashMap<&'static str, &'static str>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
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
        self.id.cmp(&other.id)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Node {
    pub fn new(id: &'static str) -> Self {
        Node {
            id: id,
            data: HashMap::new(),
        }
    }

    pub fn with_data(id: &'static str, data: HashMap<&'static str, &'static str>) -> Self {
        Node { id, data }
    }
}

pub type RcNode = Rc<Node>;

pub trait Graph {
    fn new() -> Self;
    fn has_node(&self, node: RcNode) -> bool;
    fn neighbors(&self, node: RcNode) -> Option<Vec<RcNode>>;
}

#[derive(Clone, Debug)]
pub struct SimpleGraph {
    pub data: HashMap<RcNode, Vec<RcNode>>,
}

impl Graph for SimpleGraph {
    fn new() -> Self {
        SimpleGraph {
            data: HashMap::new(),
        }
    }

    fn neighbors(&self, node: RcNode) -> Option<Vec<RcNode>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    fn has_node(&self, node: RcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl SimpleGraph {
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
}

#[derive(Clone, Debug)]
pub struct DiGraph {
    pub data: HashMap<RcNode, Vec<RcNode>>,
}

impl Graph for DiGraph {
    fn new() -> Self {
        DiGraph {
            data: HashMap::new(),
        }
    }

    fn neighbors(&self, node: RcNode) -> Option<Vec<RcNode>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    fn has_node(&self, node: RcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl DiGraph {
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
        self.data.entry(rctarget).or_insert(Vec::new());
    }

    pub fn from_edge_list(edge_list: Vec<(Node, Node)>) -> Self {
        let mut G = Self::new();
        for (source, target) in edge_list.into_iter() {
            G.add_edge(source, target);
        }
        G
    }

    pub fn from_node_list(mut node_list: Vec<Node>) -> Self {
        let mut G = DiGraph::new();
        node_list.sort_unstable();
        node_list.dedup();
        for n in node_list.into_iter() {
            G.add_node(n);
        }
        G
    }
}

#[derive(Clone, Debug)]
pub struct WeightedGraph {
    pub data: HashMap<RcNode, HashMap<RcNode, f32>>,
}

impl Graph for WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            data: HashMap::new(),
        }
    }

    fn neighbors(&self, node: RcNode) -> Option<Vec<RcNode>> {
        match self.data.get(&node) {
            Some(v) => {
                let nodes = v.iter().map(|(node, _)| Rc::clone(node)).collect();
                Some(nodes)
            }
            None => None,
        }
    }

    fn has_node(&self, node: RcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl WeightedGraph {
    pub fn add_node(&mut self, node: Node) {
        let rcnode: RcNode = Rc::new(node);
        self.data.entry(rcnode).or_insert(HashMap::new());
    }

    pub fn add_edge(&mut self, source: Node, target: Node, weight: f32) {
        let rcsource = Rc::new(source);
        let rctarget = Rc::new(target);

        let entry_source = self
            .data
            .entry(Rc::clone(&rcsource))
            .or_insert(HashMap::new());
        match entry_source.get_mut(&rctarget) {
            Some(w) => {
                *w = weight;
            }
            None => {
                entry_source.insert(Rc::clone(&rctarget), weight);
            }
        }
        self.data.entry(rctarget).or_insert(HashMap::new());
    }

    pub fn from_edge_list(edge_list: Vec<(Node, Node, f32)>) -> Self {
        let mut G = Self::new();
        for (source, target, weight) in edge_list.into_iter() {
            G.add_edge(source, target, weight);
        }
        G
    }

    pub fn from_node_list(mut node_list: Vec<Node>) -> Self {
        let mut G = WeightedGraph::new();
        node_list.sort_unstable();
        node_list.dedup();
        for n in node_list.into_iter() {
            G.add_node(n);
        }
        G
    }

    pub fn get_edge_weight(&self, source: RcNode, target: RcNode) -> Option<&f32> {
        let neigbours = self.data.get(&source)?;
        neigbours.get(&target)
    }

    pub fn neighbor_weights(&self, node: RcNode) -> Option<HashMap<RcNode, f32>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }
}
