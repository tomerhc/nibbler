mod algorithms;
mod graph;
mod manager;
use algorithms::{bfs_by, dijkstra};
use graph::{Node, RcNode, SimpleGraph, WeightedGraph};
use std::rc::Rc;

fn main() {
    let edges = vec![
        (Node::new("1"), Node::new("5"), 3.0),
        (Node::new("1"), Node::new("2"), 3.0),
        (Node::new("2"), Node::new("3"), 3.0),
        (Node::new("2"), Node::new("4"), 1.0),
        (Node::new("4"), Node::new("9"), 2.0),
        (Node::new("9"), Node::new("8"), 1.0),
        (Node::new("1"), Node::new("6"), 2.0),
        (Node::new("6"), Node::new("7"), 3.0),
        (Node::new("6"), Node::new("8"), 7.0),
        (Node::new("8"), Node::new("10"), 1.0),
        (Node::new("11"), Node::new("12"), 3.0),
    ];
    let G = WeightedGraph::from_edge_list(edges);
    print!(
        "{:?}\n",
        dijkstra(&G, Rc::new(Node::new("1")), Rc::new(Node::new("10")))
    );
}
