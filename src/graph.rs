use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<T: PartialEq + Eq + Hash + Clone, W: Clone> {
    pub connections: HashMap<T, Vec<(T, W)>>,
    pub node_weights: HashMap<T, W>,
}

impl<T: PartialEq + Eq + Hash + Clone, W: Clone> Default for Graph<T, W> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq + Eq + Hash + Clone, W: Clone> Graph<T, W> {
    pub fn new() -> Self {
        Graph {
            connections: HashMap::new(),
            node_weights: HashMap::new(),
        }
    }

    pub fn add_unidirectional_edge(&mut self, from: T, to: T, edge_weight: W) {
        self.connections
            .entry(from)
            .or_default()
            .push((to, edge_weight));
    }

    pub fn add_bidirectional_edge(&mut self, from: T, to: T, edge_weight: W) {
        self.connections
            .entry(from.clone())
            .or_default()
            .push((to.clone(), edge_weight.clone()));
        self.connections
            .entry(to)
            .or_default()
            .push((from, edge_weight));
    }

    pub fn add_node_weight(&mut self, node: T, node_weight: W) {
        self.node_weights.entry(node).or_insert(node_weight);
    }
}
