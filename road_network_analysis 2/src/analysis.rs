// src/analysis.rs
use std::collections::HashMap;
use super::graph::{Graph, Vertex};
use std::collections::HashSet;
use std::collections::VecDeque;  


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

// Identify connected components in the graph
pub fn connected_components(graph: &Graph) -> Vec<HashSet<Vertex>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for i in 0..graph.n {
        if !visited.contains(&i) {
            let mut component = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back(i);
            visited.insert(i);

            while let Some(current) = queue.pop_front() {
                component.insert(current);

                for &neighbor in &graph.outedges[current] {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);
                    }
                }
            }

            components.push(component);
        }
    }

    components
}
