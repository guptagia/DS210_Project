#[cfg(test)]
mod tests {
    use super::*;  // Access all functions and types in the current module
    use crate::parser::read_file;  // Functions for reading files
    use crate::graph::{Graph, Vertex};  // Import Graph structure
    use crate::analysis::{bfs, degree_distribution};  // BFS and degree distribution
    use crate::clusters::connected_components;  // Import connected components
    use std::collections::HashMap;  // For BFS results

    // Test the `read_file` function from the `parser` module
    #[test]
    fn test_read_file() {
        // Create a simple dataset
        let data = "0 1\n1 2\n2 3\n";

        // Use a temporary file to test the file reading
        use std::io::Write;
        let temp_file_path = "temp_test_data.txt";  // Temporary file path
        let mut temp_file = std::fs::File::create(temp_file_path).expect("Failed to create temp file");
        temp_file.write_all(data.as_bytes()).expect("Failed to write to temp file");

        let edges = read_file(temp_file_path);  // Read the dataset

        // Validate the expected number of edges and specific values
        assert_eq!(edges.len(), 3, "Should parse 3 edges");
        assert_eq!(edges[0], (0, 1), "First edge should be (0, 1)");
        assert_eq!(edges[1], (1, 2), "Second edge should be (1, 2)");
        assert_eq!(edges[2], (2, 3), "Third edge should be (2, 3)");
    }

    // Test the `create_directed` function from the `graph` module
    #[test]
    fn test_create_directed_graph() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];  // Simple directed graph
        let graph = Graph::create_directed(4, &edges);  // Directed graph with 4 nodes

        assert_eq!(graph.outedges.len(), 4, "Graph should have 4 nodes");
        assert_eq!(graph.outedges[0], vec![1], "Node 0 should have an edge to 1");
        assert_eq!(graph.outedges[1], vec![2], "Node 1 should have an edge to 2");
        assert_eq!(graph.outedges[2], vec![3], "Node 2 should have an edge to 3");
    }

    // Test the BFS function from the `analysis` module
    #[test]
    fn test_bfs() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];  // Simple directed graph
        let graph = Graph::create_directed(4, &edges);  // Directed graph with 4 nodes

        let distances = bfs(&graph, 0);  // Perform BFS from node 0

        // Validate BFS results for expected distances
        assert_eq!(distances.get(&1), Some(&1), "Distance from 0 to 1 should be 1");
        assert_eq!(distances.get(&2), Some(&2), "Distance from 0 to 2 should be 2");
        assert_eq!(distances.get(&3), Some(&3), "Distance from 0 to 3 should be 3");
    }

    // Test the degree distribution from the `analysis` module
    #[test]
    fn test_degree_distribution() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];  // Simple directed graph
        let graph = Graph::create_directed(4, &edges);  // Directed graph with 4 nodes

        let degree_dist = degree_distribution(&graph);  // Get degree distribution

        // Validate the expected degree distribution
        assert!(degree_dist.get(&1).is_some(), "Degree 1 should be present");
        assert_eq!(degree_dist[&1], 3, "There should be 3 nodes with degree 1");
    }

    // Test connected components from the `clusters` module
    #[test]
    fn test_connected_components() {
        // Small sample dataset for testing
        let edges = vec![(0, 1), (1, 2), (3, 4)];  // Two connected components
        let graph = Graph::create_undirected(5, &edges);

        // Call the `connected_components` function
        let components = connected_components(&graph);

        // Use the result to ensure it's actually tested
        assert_eq!(components.len(), 2, "There should be two connected components");
        assert!(components[0].contains(&0), "First component should contain node 0");
        assert!(components[1].contains(&3), "Second component should contain node 3");
    }
}
