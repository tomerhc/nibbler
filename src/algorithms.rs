use crate::graph::{Graph, Node, RcNode, Simplegraph, Weightedgraph};
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub fn bfs_by<F, G>(g: &G, root: RcNode, goal_fn: F) -> Option<RcNode>
where
    F: Fn(&RcNode) -> bool,
    G: Graph,
{
    let mut queue = VecDeque::from(vec![root]);
    let mut visited = Vec::new();
    while !queue.is_empty() {
        let current_node = queue.pop_back().unwrap();
        if !visited.contains(&current_node) {
            if goal_fn(&current_node) {
                return Some(Rc::clone(&current_node));
            } else {
                let ns = g.neighbors(Rc::clone(&current_node)).unwrap_or(vec![]);
                let _: Vec<_> = ns.iter().map(|n| queue.push_front(Rc::clone(n))).collect();
                visited.push(Rc::clone(&current_node));
            }
        }
    }
    None
}

pub fn djikastra(g: &Weightedgraph, root: RcNode, target: RcNode) -> Option<Vec<RcNode>> {
    None
}
