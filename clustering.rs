use petgraph::graph::{NodeIndex};
use petgraph::Graph;

use crate::graph::CarNode;

pub struct Clusterer;

impl Clusterer {
    pub fn k_means(graph: &Graph<CarNode, f64>, k: usize) -> Vec<Vec<NodeIndex>> {
        let mut clusters: Vec<Vec<NodeIndex>> = vec![vec![]; k];
        for (i, node) in graph.node_indices().enumerate() {
            clusters[i % k].push(node);
        }
        clusters
    }
}
