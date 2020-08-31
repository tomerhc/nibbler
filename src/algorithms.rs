use crate::graph::{Graph, Node, RcNode, SimpleGraph, WeightedGraph};
use std::cmp::Ordering::Equal;
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

pub fn dijkstra(g: &WeightedGraph, root: RcNode, target: RcNode) -> Option<(Vec<RcNode>, f32)> {
    let mut costs: HashMap<RcNode, f32> = g
        .neighbor_weights(Rc::clone(&root))?
        .iter()
        .map(|(node, w)| (Rc::clone(node), *w))
        .collect();

    let mut parents: HashMap<RcNode, Option<RcNode>> = g
        .neighbors(Rc::clone(&root))?
        .iter()
        .map(|node| (Rc::clone(node), Some(Rc::clone(&root))))
        .collect();

    for n in g.data.keys() {
        costs.entry(Rc::clone(n)).or_insert(f32::INFINITY);
        parents.entry(Rc::clone(n)).or_insert(None);
    }

    let mut processed_nodes: Vec<RcNode> = vec![Rc::clone(&root)];

    let mut current_node = Some(Rc::clone(&root));
    while let Some(node) = current_node {
        if node == target {
            return Some((
                make_route(parents, Rc::clone(&root), Rc::clone(&node)),
                *costs.get(&node).unwrap(),
            ));
        } else {
            let cost = *costs.get(&node).unwrap();
            let neighbors = g.neighbor_weights(Rc::clone(&node)).unwrap();
            for (n, c) in neighbors.iter() {
                let new_cost = c + cost;
                if costs.get(n).unwrap() > &new_cost {
                    *costs.get_mut(n).unwrap() = new_cost;
                    *parents.get_mut(n).unwrap() = Some(Rc::clone(&node))
                }
            }
            processed_nodes.push(Rc::clone(&node));
            current_node = lowest_cost_node(&costs, &processed_nodes);
        }
    }
    None
}

fn make_route(
    parents: HashMap<RcNode, Option<RcNode>>,
    root: RcNode,
    target: RcNode,
) -> Vec<RcNode> {
    let mut route = Vec::new();
    let mut current = target;
    while current != root {
        route.push(Rc::clone(&current));
        current = Rc::clone(parents.get(&current).unwrap().as_ref().unwrap());
    }
    route.push(root);
    route.into_iter().rev().collect()
}

fn lowest_cost_node<'a>(
    costs: &'a HashMap<RcNode, f32>,
    processed: &'a Vec<RcNode>,
) -> Option<RcNode> {
    let not_proc: HashMap<&RcNode, f32> = costs
        .into_iter()
        .filter(|(n, _)| !processed.contains(n))
        .map(|(n, w)| (n, *w))
        .collect();
    let min_node = not_proc
        .into_iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Equal))?;
    if min_node.1 == f32::INFINITY {
        return None;
    }
    Some(Rc::clone(min_node.0))
}
