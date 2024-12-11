mod graph;
mod clustering;

use graph::GraphBuilder;
use clustering::Clusterer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "used_car_dataset.csv";

    // Step 1: Load graph
    let mut graph = GraphBuilder::load_from_csv(file_path)?;

    // Step 2: Add edges based on similarity
    graph.add_edges(0.5); // Threshold similarity

    // Step 3: Perform clustering
    let clusters = Clusterer::k_means(&graph, 3); // Example: 3 clusters

    // Step 4: Display clusters
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}: {:?}", i + 1, cluster);
    }

    Ok(())
}
