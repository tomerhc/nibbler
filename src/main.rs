mod algorithms;
mod graph;
use graph::{Node, SimpleGraph};
use std::rc::Rc;

fn main() {
    let edges = vec![
        (Node::new("0"), Node::new("1")),
        (Node::new("1"), Node::new("2")),
        (Node::new("2"), Node::new("3")),
        (Node::new("4"), Node::new("5")),
        (Node::new("5"), Node::new("1")),
    ];
    let G = SimpleGraph::from_edge_list(edges);

    println!("{:?}", G.neighbors(Rc::new(Node::new("1"))));
}
