use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
//use std::rc::{Arc, Weak};
use std::sync::{Arc, Weak};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub data: Option<HashMap<String, String>>,
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
    pub fn new(id: String) -> Self {
        Node {
            id: id,
            data: Some(HashMap::new()),
        }
    }

    pub fn with_data(id: String, data: HashMap<String, String>) -> Self {
        Node {
            id: id,
            data: Some(data),
        }
    }
}

pub type ArcNode = Arc<Node>;

pub trait Graph {
    fn has_node(&self, node: ArcNode) -> bool;
    fn neighbors(&self, node: ArcNode) -> Option<Vec<ArcNode>>;
}

#[derive(Clone, Debug)]
pub struct SimpleGraph {
    pub data: HashMap<ArcNode, Vec<ArcNode>>,
}

impl Graph for SimpleGraph {
    fn neighbors(&self, node: ArcNode) -> Option<Vec<ArcNode>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    fn has_node(&self, node: ArcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl SimpleGraph {
    fn new() -> Self {
        SimpleGraph {
            data: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let rcnode: ArcNode = Arc::new(node);
        self.data.entry(rcnode).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, source: Node, target: Node) {
        let rcsource = Arc::new(source);
        let rctarget = Arc::new(target);

        let entry_source = self.data.entry(Arc::clone(&rcsource)).or_insert(Vec::new());
        match entry_source.binary_search(&rctarget) {
            Ok(_) => (),
            Err(ind) => entry_source.insert(ind, Arc::clone(&rctarget)),
        }

        let entry_target = self.data.entry(Arc::clone(&rctarget)).or_insert(Vec::new());
        match entry_target.binary_search(&rcsource) {
            Ok(_) => (),
            Err(ind) => entry_target.insert(ind, Arc::clone(&rcsource)),
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
    pub data: HashMap<ArcNode, Vec<ArcNode>>,
}

impl Graph for DiGraph {
    fn neighbors(&self, node: ArcNode) -> Option<Vec<ArcNode>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }

    fn has_node(&self, node: ArcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl DiGraph {
    fn new() -> Self {
        DiGraph {
            data: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) {
        let rcnode: ArcNode = Arc::new(node);
        self.data.entry(rcnode).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, source: Node, target: Node) {
        let rcsource = Arc::new(source);
        let rctarget = Arc::new(target);

        let entry_source = self.data.entry(Arc::clone(&rcsource)).or_insert(Vec::new());
        match entry_source.binary_search(&rctarget) {
            Ok(_) => (),
            Err(ind) => entry_source.insert(ind, Arc::clone(&rctarget)),
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
    pub data: HashMap<ArcNode, HashMap<ArcNode, f32>>,
}

impl Graph for WeightedGraph {
    fn neighbors(&self, node: ArcNode) -> Option<Vec<ArcNode>> {
        match self.data.get(&node) {
            Some(v) => {
                let nodes = v.iter().map(|(node, _)| Arc::clone(node)).collect();
                Some(nodes)
            }
            None => None,
        }
    }

    fn has_node(&self, node: ArcNode) -> bool {
        self.data.contains_key(&node)
    }
}

impl WeightedGraph {
    fn new() -> Self {
        WeightedGraph {
            data: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, node: Node) {
        let rcnode: ArcNode = Arc::new(node);
        self.data.entry(rcnode).or_insert(HashMap::new());
    }

    pub fn add_edge(&mut self, source: Node, target: Node, weight: f32) {
        let rcsource = Arc::new(source);
        let rctarget = Arc::new(target);

        let entry_source = self
            .data
            .entry(Arc::clone(&rcsource))
            .or_insert(HashMap::new());
        match entry_source.get_mut(&rctarget) {
            Some(w) => {
                *w = weight;
            }
            None => {
                entry_source.insert(Arc::clone(&rctarget), weight);
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

    pub fn get_edge_weight(&self, source: ArcNode, target: ArcNode) -> Option<&f32> {
        let neigbours = self.data.get(&source)?;
        neigbours.get(&target)
    }

    pub fn neighbor_weights(&self, node: ArcNode) -> Option<HashMap<ArcNode, f32>> {
        match self.data.get(&node) {
            Some(v) => Some(v.to_owned()),
            None => None,
        }
    }
}
