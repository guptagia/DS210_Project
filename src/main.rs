mod parser;
mod graph;
mod analysis;

use crate::parser::{read_file, find_max_index_parallel};
use crate::graph::Graph;
use crate::analysis::{average_distance_sampled, degree_distribution};

fn main() {
    let file_path = "roadNet-CA.txt";
    println!("Attempting to find the maximum index.");
    let n = match find_max_index_parallel(file_path) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Error determining the maximum index: {:?}", e);
            return;
        },
    };

    println!("Reading file to get edges");
    let edges = match read_file(file_path) {
       Ok(edges) => edges,
       Err(e) => {
        eprintln!("Error reading file: {:?}", e);
           return;
       },
};

   println!("Creating an undirected graph.");
   let graph = Graph::create_undirected_optimized(n, &edges);

    println!("Calculating average distance sampled.");
    let avg_distance = average_distance_sampled(&graph, 1000);
    println!("Average distance between pairs of vertices: {}", avg_distance);

    println!("Calculating degree distribution.");
    let degree_dist = degree_distribution(&graph);
    let mut degree_vec: Vec<_> = degree_dist.iter().collect();
    degree_vec.sort_by_key(|&(degree, _)| *degree);

    println!("Degree distribution:");
    for &(degree, count) in degree_vec.iter() {
        println!("Degree {}: {} vertices", degree, count);
    }
    println!("Completed degree distribution.");
}