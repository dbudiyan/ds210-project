mod data_processing;
mod graph;
mod centrality;

use data_processing::process_dataset;
use graph::build_graph;
use centrality::{calculate_closeness_centrality, calculate_betweenness_centrality};

fn main() {
    // Entry point of the program. Provides an overview of what the program does.
    println!("Centrality Measures Analysis Project");

    let processed_data = process_dataset("data/used_car_dataset.csv");
    let graph = build_graph(&processed_data);

    let closeness = calculate_closeness_centrality(&graph);
    let betweenness = calculate_betweenness_centrality(&graph);

    println!("Closeness Centrality: {:#?}", closeness);
    println!("Betweenness Centrality: {:#?}", betweenness);
}
