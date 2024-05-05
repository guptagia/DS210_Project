use std::collections::HashMap;
use crate::graph::Graph;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;

pub fn average_distance_sampled(graph: &Graph, samples: usize) -> f64 {
    let mut rng = thread_rng();
    let node_indices: Vec<usize> = (0..graph.n).collect();
    let sample_nodes: Vec<_> = node_indices.as_slice().choose_multiple(&mut rng, samples).cloned().collect();

    let total_distance: f64 = sample_nodes.par_iter().map(|&start| {
        let distances = graph.bfs(start);
        distances.iter().filter(|&&d| d != usize::MAX).map(|&d| d as f64).sum::<f64>()
    }).sum();

    let count = samples * (graph.n - 1);
    total_distance / count as f64
}

pub fn degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new();
    for list in &graph.outedges {
        let degree = list.len();
        *degree_count.entry(degree).or_insert(0) += 1;
    }
    degree_count
}

