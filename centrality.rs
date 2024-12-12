use crate::graph::Graph;

// Calculates the closeness centrality for each node in the graph.
pub fn calculate_closeness_centrality(graph: &Graph) -> Vec<f64> {
    let mut centrality = vec![0.0; graph.nodes().len()];

    for (i, _node) in graph.nodes().iter().enumerate() {
        let mut total_distance = 0;
        for (j, _other_node) in graph.nodes().iter().enumerate() {
            if i != j {
                total_distance += 1; // Dummy distance calculation
            }
        }
        centrality[i] = if total_distance > 0 {
            1.0 / total_distance as f64
        } else {
            0.0
        };
    }

    centrality
}

// Calculates the betweenness centrality for each node in the graph.
pub fn calculate_betweenness_centrality(graph: &Graph) -> Vec<f64> {
    let mut centrality = vec![0.0; graph.nodes().len()];

    for (index, _node) in graph.nodes().iter().enumerate() {
        for &(start, end) in graph.edges() {
            if start == index || end == index {
                centrality[index] += 1.0; // Dummy edge involvement calculation
            }
        }
    }

    centrality
}

#[cfg(test)]
// Unit tests for the main modules to ensure correctness.
mod tests {
    use super::*;

    #[test]
    fn test_dataset_processing() {
        let processed_data = process_dataset("data/used_car_dataset.csv");
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
