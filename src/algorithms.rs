use crate::graph::{ArcNode, Graph, Node, SimpleGraph, WeightedGraph};
use std::cmp::Ordering::Equal;
use std::collections::{HashMap, VecDeque};
//use std::rc::Arc;
use std::sync::{Arc, Weak};
pub fn bfs_by<F, G>(g: &G, root: ArcNode, goal_fn: F) -> Option<ArcNode>
where
    F: Fn(&ArcNode) -> bool,
    G: Graph,
{
    let mut queue = VecDeque::from(vec![root]);
    let mut visited = Vec::new();
    while !queue.is_empty() {
        let current_node = queue.pop_back().unwrap();
        if !visited.contains(&current_node) {
            if goal_fn(&current_node) {
                return Some(Arc::clone(&current_node));
            } else {
                let ns = g.neighbors(Arc::clone(&current_node)).unwrap_or(vec![]);
                let _: Vec<_> = ns.iter().map(|n| queue.push_front(Arc::clone(n))).collect();
                visited.push(Arc::clone(&current_node));
            }
        }
    }
    None
}

pub fn dijkstra(g: &WeightedGraph, root: ArcNode, target: ArcNode) -> Option<(Vec<ArcNode>, f32)> {
    let mut costs: HashMap<ArcNode, f32> = g
        .neighbor_weights(Arc::clone(&root))?
        .iter()
        .map(|(node, w)| (Arc::clone(node), *w))
        .collect();

    let mut parents: HashMap<ArcNode, Option<ArcNode>> = g
        .neighbors(Arc::clone(&root))?
        .iter()
        .map(|node| (Arc::clone(node), Some(Arc::clone(&root))))
        .collect();

    for n in g.data.keys() {
        costs.entry(Arc::clone(n)).or_insert(f32::INFINITY);
        parents.entry(Arc::clone(n)).or_insert(None);
    }

    let mut processed_nodes: Vec<ArcNode> = vec![Arc::clone(&root)];

    let mut current_node = Some(Arc::clone(&root));
    while let Some(node) = current_node {
        if node == target {
            return Some((
                make_route(parents, Arc::clone(&root), Arc::clone(&node)),
                *costs.get(&node).unwrap(),
            ));
        } else {
            let cost = *costs.get(&node).unwrap();
            let neighbors = g.neighbor_weights(Arc::clone(&node)).unwrap();
            for (n, c) in neighbors.iter() {
                let new_cost = c + cost;
                if costs.get(n).unwrap() > &new_cost {
                    *costs.get_mut(n).unwrap() = new_cost;
                    *parents.get_mut(n).unwrap() = Some(Arc::clone(&node))
                }
            }
            processed_nodes.push(Arc::clone(&node));
            current_node = lowest_cost_node(&costs, &processed_nodes);
        }
    }
    None
}

fn make_route(
    parents: HashMap<ArcNode, Option<ArcNode>>,
    root: ArcNode,
    target: ArcNode,
) -> Vec<ArcNode> {
    let mut route = Vec::new();
    let mut current = target;
    while current != root {
        route.push(Arc::clone(&current));
        current = Arc::clone(parents.get(&current).unwrap().as_ref().unwrap());
    }
    route.push(root);
    route.into_iter().rev().collect()
}

fn lowest_cost_node<'a>(
    costs: &'a HashMap<ArcNode, f32>,
    processed: &'a Vec<ArcNode>,
) -> Option<ArcNode> {
    let not_proc: HashMap<&ArcNode, f32> = costs
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
    Some(Arc::clone(min_node.0))
}
