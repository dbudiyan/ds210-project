use crate::graph::Graph;
use std::collections::{VecDeque};

/// Calculates the shortest path distance between two nodes using BFS.
fn bfs_shortest_path(graph: &Graph, start: usize) -> Vec<Option<usize>> {
    let mut distances = vec![None; graph.nodes().len()];
    let mut queue = VecDeque::new();
    distances[start] = Some(0);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let current_distance = distances[node].unwrap();
        
        // Look for neighbors of the current node
        for &(a, b) in graph.edges() {
            let neighbor = if a == node { b } else if b == node { a } else { continue };

            if distances[neighbor].is_none() {
                distances[neighbor] = Some(current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }
    
    distances
}

/// Calculates the closeness centrality for each node in the graph.
pub fn calculate_closeness_centrality(graph: &Graph) -> Vec<f64> {
    let mut centrality = vec![0.0; graph.nodes().len()];

    for i in 0..graph.nodes().len() {
        // Calculate the shortest path distances from node `i` to all other nodes
        let distances = bfs_shortest_path(graph, i);
        
        // Sum the distances (ignore unreachable nodes)
        let reachable_distances: Vec<usize> = distances.iter().filter_map(|&d| d).collect();
        let total_distance: usize = reachable_distances.iter().sum();
        let reachable_count = reachable_distances.len();

        // Closeness centrality is normalized by the number of reachable nodes
        if total_distance > 0 && reachable_count > 1 {
            centrality[i] = (reachable_count as f64 - 1.0) / total_distance as f64;
        } else {
            centrality[i] = 0.0; // If the node is isolated, its centrality is 0
        }
    }

    // Normalize centrality values to be in the range [0, 1]
    let max_centrality = centrality.iter().cloned().fold(f64::MIN, f64::max);
    if max_centrality > 0.0 {
        centrality.iter_mut().for_each(|c| *c /= max_centrality);
    }

    centrality
}

// Calculates the betweenness centrality for each node in the graph.
pub fn calculate_betweenness_centrality(graph: &Graph) -> Vec<f64> {
    let mut centrality = vec![0.0; graph.nodes().len()];
    let node_count = graph.nodes().len();

    if node_count < 2 {
        return centrality; // No centrality for graphs with fewer than 2 nodes
    }

    for s in 0..node_count {
        let mut shortest_paths = vec![0.0; node_count];
        let mut dependencies = vec![0.0; node_count];
        let mut stack = Vec::new();
        let mut queue = std::collections::VecDeque::new();
        let mut predecessors = vec![Vec::new(); node_count];

        shortest_paths[s] = 1.0;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for &(start, end) in graph.edges() {
                let w = if start == v { end } else if end == v { start } else { continue };

                if shortest_paths[w] == 0.0 {
                    queue.push_back(w);
                }

                if !predecessors[w].contains(&v) {
                    predecessors[w].push(v);
                }

                shortest_paths[w] += shortest_paths[v];
            }
        }

        while let Some(w) = stack.pop() {
            for &v in &predecessors[w] {
                dependencies[v] += (shortest_paths[v] / shortest_paths[w]) * (1.0 + dependencies[w]);
            }
            if w != s {
                centrality[w] += dependencies[w];
            }
        }
    }

    // Normalize the betweenness centrality values
    let max_possible = ((node_count - 1) * (node_count - 2)) as f64;

    if max_possible > 0.0 {
        for value in &mut centrality {
            *value /= max_possible;
        }
    }

    centrality
}

/// Returns the indices of the nodes with the highest centrality values.
pub fn get_highest_centrality_indices(_graph: &Graph, closeness: &[f64], betweenness: &[f64]) -> (Option<usize>, Option<usize>) {
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

    (max_closeness, max_betweenness)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

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
