use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        let index1 = self.nodes.iter().position(|n| n == node1).unwrap();
        let index2 = self.nodes.iter().position(|n| n == node2).unwrap();
        self.edges.push((index1, index2));
    }

    pub fn nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}

// Builds a tree structure
pub fn build_graph(data: &[HashMap<String, String>]) -> Graph {
    let mut graph = Graph::new();

    // Add nodes to the graph
    for entry in data {
        if let Some(brand) = entry.get("Brand") {
            graph.add_node(brand.clone());
        }
    }

    // Create a sparse tree-like structure by connecting nodes sequentially
    let nodes = graph.nodes.clone();
    let node_count = nodes.len();

    for i in 0..node_count {
        if i + 1 < node_count {
            graph.add_edge(&nodes[i], &nodes[i + 1]); // Create a chain of nodes
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        let data = vec![{
            let mut map = std::collections::HashMap::new();
            map.insert("Brand".to_string(), "Toyota".to_string());
            map
        }];

        let graph = build_graph(&data);
        assert_eq!(graph.nodes.len(), 1, "Graph should have one node.");
    }
}
