// Import modules for data processing, graph construction, and centrality computations
mod data_processing;
mod graph;
mod centrality;

// Bring functions into scope from modules
use data_processing::{process_dataset};
use graph::{build_graph, build_subgraph};
use centrality::{calculate_closeness_centrality, calculate_betweenness_centrality, get_highest_centrality_indices};

fn main() {
    println!("Centrality Measures Analysis Project");

    // Load and process dataset from the used_car_dataset csv file
    let processed_data = process_dataset("used_car_dataset.csv");

    // Build the main graph
    let graph = build_graph(&processed_data);

    // Compute centrality measures for the full graph
    let closeness = calculate_closeness_centrality(&graph);
    let betweenness = calculate_betweenness_centrality(&graph);

    println!("Closeness Centrality: {:#?}", closeness);
    println!("Betweenness Centrality: {:#?}", betweenness);
    
    // Get the indices of the nodes with the highest closeness and betweenness centrality (in centrality.rs)
    let (max_closeness_idx, max_betweenness_idx) = get_highest_centrality_indices(&graph, &closeness, &betweenness);

    if let Some(idx) = max_closeness_idx {
        println!(
            "Node with highest closeness centrality: {} (Value: {:.4})",
            graph.nodes()[idx], closeness[idx]
        );
    }
    if let Some(idx) = max_betweenness_idx {
        println!(
            "Node with highest betweenness centrality: {} (Value: {:.4})",
            graph.nodes()[idx], betweenness[idx]
        );
    }

    // Build subgraph for nodes related to Chevrolet
    let chevrolet_subgraph = build_subgraph(&graph, "Chevrolet");
    println!("Subgraph for Chevrolet: {:?}", chevrolet_subgraph);

    // Calculate centrality measures for the subgraph
    let closeness_subgraph = calculate_closeness_centrality(&chevrolet_subgraph);
    let betweenness_subgraph = calculate_betweenness_centrality(&chevrolet_subgraph);

    println!("Closeness Centrality in subgraph: {:?}", closeness_subgraph);
    println!("Betweenness Centrality in subgraph: {:?}", betweenness_subgraph);
}
