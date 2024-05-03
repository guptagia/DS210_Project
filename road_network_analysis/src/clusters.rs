// src/clusters.rs
use std::collections::{HashSet, VecDeque};  // Necessary collections
use crate::graph::Graph;  // Import the Graph structure

// Function to find connected components in an undirected graph
pub fn connected_components(graph: &Graph) -> Vec<HashSet<usize>> {
    let mut visited = HashSet::new();  // Set to track visited nodes
    let mut components = Vec::new();  // Stores the connected components

    // Iterate through all nodes in the graph
    for node in 0..graph.n {
        if !visited.contains(&node) {  // If the node hasn't been visited
            let mut component = HashSet::new();  // Create a new component
            let mut queue = VecDeque::new();  // BFS queue

            queue.push_back(node);  // Start BFS from the current node
            visited.insert(node);  // Mark it as visited

            // Perform BFS to find all nodes in the connected component
            while let Some(current) = queue.pop_front() {
                component.insert(current);  // Add to the component

                // Add unvisited neighbors to the queue
                for &neighbor in &graph.outedges[current] {
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                        visited.insert(neighbor);  // Mark as visited
                    }
                }
            }

            components.push(component);  // Add the component to the list
        }
    }

    components  // Return the list of connected components
}

