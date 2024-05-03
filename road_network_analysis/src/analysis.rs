// src/analysis.rs
use std::collections::HashMap;  // Importing multiple collections
use crate::graph::Graph;  // Importing Graph structure
 


// Calculate the average distance between pairs of vertices
pub fn average_distance(graph: &Graph) -> f64 {
    let mut total_distance = 0.0;
    let mut count = 0;

    // Perform BFS from each vertex to calculate distances
    for i in 0..graph.n {
        let distances = graph.bfs(i);

        for (_, distance) in distances {
            total_distance += distance as f64;
            count += 1;
        }
    }

    total_distance / count as f64
}

// Degree distribution analysis
pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new();

    for l in &graph.outedges {
        let degree = l.len();
        *degree_count.entry(degree).or_insert(0) += 1;
    }

    degree_count
}

