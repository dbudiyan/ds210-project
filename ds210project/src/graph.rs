use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<String>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    // Creates a new, empty graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    // Adds a node to the graph if it doesn't already exist.
    pub fn add_node(&mut self, node: String) {
        if !self.nodes.contains(&node) {
            self.nodes.push(node);
        }
    }

    // Adds an edge between two nodes in the graph.
    // Panics if either of the nodes does not exist in the graph.
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        let index1 = self.nodes.iter().position(|n| n == node1).unwrap();
        let index2 = self.nodes.iter().position(|n| n == node2).unwrap();
        self.edges.push((index1, index2));
    }

    // Returns a reference to the list of nodes in the graph.
    pub fn nodes(&self) -> &Vec<String> {
        &self.nodes
    }

    // Returns a reference to the list of edges in the graph.
    pub fn edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
}

// Builds a tree structure based on the dataset.
// Each node represents a unique "Brand" from the dataset; nodes are connected sequentially to form a sparse tree.
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

// Builds a subgraph centered on a target node, includes the target node and its direct neighbors.
pub fn build_subgraph(graph: &Graph, target_node: &str) -> Graph {
    let mut subgraph = Graph::new();
    let node_index = graph.nodes.iter().position(|n| n == target_node);

    if let Some(index) = node_index {
        subgraph.add_node(graph.nodes[index].clone());

        for &(start, end) in &graph.edges {
            if start == index || end == index {
                let neighbor_index = if start == index { end } else { start };
                subgraph.add_node(graph.nodes[neighbor_index].clone());
                subgraph.add_edge(
                    &graph.nodes[start],
                    &graph.nodes[end],
                );
            }
        }
    }

    subgraph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Tests the construction of a graph from a dataset; ensures nodes are added correctly and edges are empty for a single-node graph.
    fn test_graph_construction() {
        let data = vec![{
            let mut map = std::collections::HashMap::new();
            map.insert("Brand".to_string(), "Toyota".to_string());
            map
        }];

        let graph = build_graph(&data);
        assert_eq!(graph.nodes.len(), 1, "Graph should have one node.");
        assert!(graph.edges.is_empty(), "Graph should have no edges.");
    }

    #[test]
    // Tests adding a single node to the graph; verifies that the node is added correctly and that no duplicate nodes exist.
    fn test_add_node() {
        let mut graph = Graph::new();
        graph.add_node("Toyota".to_string());
        assert_eq!(graph.nodes.len(), 1, "Graph should have one node after adding.");
        assert_eq!(graph.nodes[0], "Toyota", "Node should be 'Toyota'.");
    }

    #[test]
    // Tests adding an edge between two nodes in the graph; verifies the edge list is updated correctly and references the correct nodes.
    fn test_add_edge() {
        let mut graph = Graph::new();
        graph.add_node("Toyota".to_string());
        graph.add_node("Honda".to_string());
        graph.add_edge("Toyota", "Honda");

        assert_eq!(graph.edges.len(), 1, "Graph should have one edge.");
        assert_eq!(graph.edges[0], (0, 1), "Edge should connect 'Toyota' and 'Honda'.");
    }

    #[test]
    // Tests building a subgraph centered on a specific node; ensures the subgraph contains the target node and its neighbors.
    fn test_build_subgraph() {
        let mut graph = Graph::new();
        graph.add_node("Chevrolet".to_string());
        graph.add_node("Jeep".to_string());
        graph.add_node("Jaguar".to_string());
        graph.add_edge("Chevrolet", "Jeep");
        graph.add_edge("Chevrolet", "Jaguar");

        let subgraph = build_subgraph(&graph, "Chevrolet");

        assert_eq!(subgraph.nodes.len(), 3, "Subgraph should have three nodes.");
        assert_eq!(subgraph.edges.len(), 2, "Subgraph should have two edges.");
    }
}
