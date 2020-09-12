use crate::graph::{self, Graph, Node};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;

pub struct Manager {
    pub sessions: HashMap<String, Box<dyn Graph + Send>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            sessions: HashMap::new(),
        }
    }

    pub fn new_session<G: Graph + Send + 'static>(&mut self, g: G) -> String {
        let id = assign_id();
        self.sessions.insert(id.clone(), Box::new(g));
        id
    }

    pub fn create_simple_from_nodes(&mut self, data: Vec<u8>) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<Node> = serde_json::from_slice(&data)?;
        let g = graph::SimpleGraph::from_node_list(node_list);
        Ok(self.new_session(g))
    }

    pub fn create_simple_from_edges(
        &mut self,
        data: &'static [u8],
    ) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<(Node, Node)> = serde_json::from_slice(data)?;
        let g = graph::SimpleGraph::from_edge_list(node_list);
        Ok(self.new_session(g))
    }

    pub fn create_di_from_nodes(&mut self, data: &'static [u8]) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<Node> = serde_json::from_slice(data)?;
        let g = graph::DiGraph::from_node_list(node_list);
        Ok(self.new_session(g))
    }

    pub fn create_di_from_edges(&mut self, data: &'static [u8]) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<(Node, Node)> = serde_json::from_slice(data)?;
        let g = graph::DiGraph::from_edge_list(node_list);
        Ok(self.new_session(g))
    }
    pub fn create_weighted_from_nodes(
        &mut self,
        data: &'static [u8],
    ) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<Node> = serde_json::from_slice(data)?;
        let g = graph::WeightedGraph::from_node_list(node_list);
        Ok(self.new_session(g))
    }

    pub fn create_weighted_from_edges(
        &mut self,
        data: &'static [u8],
    ) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<(Node, Node, f32)> = serde_json::from_slice(data)?;
        let g = graph::WeightedGraph::from_edge_list(node_list);
        Ok(self.new_session(g))
    }
}

fn assign_id() -> String {
    let id: String = nanoid!(10);
    id
}
