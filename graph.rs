use std::collections::HashMap;
use rand::random; 

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    // Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // Adds a node to the graph if it does not already exist.
    pub fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    // Adds an edge between two nodes. Assumes both nodes already exist in the graph.
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        let index1 = self.nodes.iter().position(|n| n == node1).unwrap();
        let index2 = self.nodes.iter().position(|n| n == node2).unwrap();
        self.edges.push((index1, index2));
    }

    // Getter for nodes
    pub fn nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    // Getter for edges
    pub fn edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}

// Construct a graph with some realistic edges (not fully connected).
pub fn build_graph(data: &[HashMap<String, String>]) -> Graph {
    let mut graph = Graph::new();

    // Add nodes to the graph
    for entry in data {
        if let Some(brand) = entry.get("Brand") {
            graph.add_node(brand.clone());
        }
    }

    // Add some edges between random nodes (avoid complete graph)
    let nodes = graph.nodes.clone();
    let node_count = nodes.len();

    for i in 0..node_count {
        for j in i + 1..node_count {
            if rand::random::<f64>() < 0.5 {
            graph.add_edge(&nodes[i], &nodes[j]);
            }
        }
    }

    graph
}
