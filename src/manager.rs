use crate::graph::{self, Graph, Node};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::error::Error;

pub struct Manager {
    pub sessions: HashMap<String, Box<dyn Graph>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            sessions: HashMap::new(),
        }
    }

    pub fn new_session(&mut self, g: G) -> String {
        let id = assign_id();
        self.sessions.insert(id.clone(), Box::new(g));
        id
    }

    pub fn create_simple_from_nodes(&mut self, data: &[u8]) -> Result<String, Box<dyn Error>> {
        let node_list: Vec<Node> = serde_json::from_slice(data)?;
        let g = graph::SimpleGraph::from_node_list(node_list);
        Ok(self.new_session(g))
    }
}

fn assign_id() -> String {
    let id: String = nanoid!(10);
    id
}
