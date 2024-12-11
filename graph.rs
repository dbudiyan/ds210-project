use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

#[derive(Debug)]
pub struct CarNode {
    pub id: usize,
    pub price: f64,
    pub mileage: f64,
}

pub struct GraphBuilder {
    graph: Graph<CarNode, f64, Undirected>,
}

impl GraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: Graph::new_undirected(),
        }
    }

    pub fn load_from_csv(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let mut rdr = Reader::from_reader(BufReader::new(file));
        let mut builder = Self::new();

        for (idx, result) in rdr.records().enumerate() {
            let record = result?;
            let price: f64 = record.get(1).unwrap_or("0").parse()?;
            let mileage: f64 = record.get(2).unwrap_or("0").parse()?;
            builder.graph.add_node(CarNode { id: idx, price, mileage });
        }

        Ok(builder)
    }

    pub fn add_edges(&mut self, threshold: f64) {
        let nodes: Vec<_> = self.graph.node_references().collect();
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let similarity = Self::calculate_similarity(
                    &nodes[i].1,
                    &nodes[j].1,
                );
                if similarity > threshold {
                    self.graph.add_edge(nodes[i].0, nodes[j].0, similarity);
                }
            }
        }
    }

    fn calculate_similarity(node1: &CarNode, node2: &CarNode) -> f64 {
        let distance = ((node1.price - node2.price).powi(2) + (node1.mileage - node2.mileage).powi(2)).sqrt();
        1.0 / (1.0 + distance)
    }

    pub fn get_graph(&self) -> &Graph<CarNode, f64, Undirected> {
        &self.graph
    }
}
