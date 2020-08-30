mod algorithms;
mod graph;
use algorithms::bfs_by;
use graph::{Node, RcNode, SimpleGraph};
use std::rc::Rc;

fn main() {
    let edges = vec![
        (Node::new("1"), Node::new("5")),
        (Node::new("1"), Node::new("2")),
        (Node::new("2"), Node::new("3")),
        (Node::new("2"), Node::new("4")),
        (Node::new("1"), Node::new("6")),
        (Node::new("6"), Node::new("7")),
        (Node::new("6"), Node::new("8")),
        (Node::new("8"), Node::new("10")),
        (Node::new("8"), Node::new("9")),
        (Node::new("11"), Node::new("12")),
    ];
    let G = SimpleGraph::from_edge_list(edges);
    let f = |n: &RcNode| n.Id == "36";
    print!("{:?}\n", bfs_by(&G, Rc::new(Node::new("1")), f));
    let f = |n: &RcNode| n.Id == "1";
    print!("{:?}", bfs_by(&G, Rc::new(Node::new("1")), f));
}
