mod data_processing;
mod graph;
mod centrality;

use data_processing::{process_dataset, filter_by_attribute};
use graph::{build_tree_graph, Graph};
use centrality::{calculate_closeness_centrality, calculate_betweenness_centrality};

fn main() {
    println!("Centrality Measures Analysis Project");

    // Load and preprocess the dataset
    let processed_data = process_dataset("used_car_dataset.csv");

    // Build the main graph
    let graph = build_tree_graph(&processed_data);

    // Compute centrality measures for the full graph
    let closeness = calculate_closeness_centrality(&graph);
    let betweenness = calculate_betweenness_centrality(&graph);

    println!("Closeness Centrality: {:#?}", closeness);
    println!("Betweenness Centrality: {:#?}", betweenness);
    
    // Find the highest centrality nodes
    display_highest_centrality(&graph, &closeness, &betweenness);
}

// Display highest centrality nodes
fn display_highest_centrality(graph: &Graph, closeness: &[f64], betweenness: &[f64]) {
    let max_closeness = closeness
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx);

    let max_betweenness = betweenness
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(idx, _)| idx);

    if let Some(idx) = max_closeness {
        println!(
            "Node with highest closeness centrality: {} (Value: {:.4})",
            graph.nodes()[idx], closeness[idx]
        );
    }
    if let Some(idx) = max_betweenness {
        println!(
            "Node with highest betweenness centrality: {} (Value: {:.4})",
            graph.nodes()[idx], betweenness[idx]
        );
    }
}
#[cfg(test)]
// Unit tests for the main modules to ensure correctness.
mod tests {
    use super::*;

    #[test]
    fn test_dataset_processing() {
        let processed_data = process_dataset("used_car_dataset.csv");
        // Ensuring the dataset is non-empty is critical because subsequent steps like graph construction
        // and centrality calculations depend on having valid data to process.
        assert!(!processed_data.is_empty(), "Processed dataset should not be empty.");
    }

    #[test]
    fn test_filter_by_attribute() {
        let data = vec![{
            let mut map = std::collections::HashMap::new();
            map.insert("Brand".to_string(), "Toyota".to_string());
            map
        }];

        let filtered = filter_by_attribute(&data, "Brand", "Toyota");
        assert_eq!(filtered.len(), 1, "Filtered data should contain one entry.");
    }

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

    #[test]
    fn test_centrality_measures() {
        let graph = Graph {
            nodes: vec!["Node1".to_string(), "Node2".to_string()],
            edges: vec![(0, 1)],
        };

        let closeness = calculate_closeness_centrality(&graph);
        assert_eq!(closeness.len(), 2, "Closeness centrality should have two values.");
    }
}