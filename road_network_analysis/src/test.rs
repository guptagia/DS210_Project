#[cfg(test)]
mod tests {
    use crate::parser::read_file;
    use crate::graph::Graph;
    use crate::analysis::degree_distribution;
    use crate::clusters::connected_components;

    #[test]
    fn test_read_file() {
        let data = "0 1\n1 2\n2 3\n";

        use std::io::Write;
        let temp_file_path = "temp_test_data.txt";
        let mut temp_file = std::fs::File::create(temp_file_path).expect("Failed to create temp file");
        temp_file.write_all(data.as_bytes()).expect("Failed to write to temp file");

        let edges = read_file(); // Removed the argument here

        assert_eq!(edges.len(), 3, "Should parse 3 edges");
        assert_eq!(edges[0], (0, 1), "First edge should be (0, 1)");
        assert_eq!(edges[1], (1, 2), "Second edge should be (1, 2)");
        assert_eq!(edges[2], (2, 3), "Third edge should be (2, 3)");
    }

    #[test]
    fn test_create_directed_graph() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);

        assert_eq!(graph.outedges.len(), 4, "Graph should have 4 nodes");
        assert_eq!(graph.outedges[0], vec![1], "Node 0 should have an edge to 1");
        assert_eq!(graph.outedges[1], vec![2], "Node 1 should have an edge to 2");
        assert_eq!(graph.outedges[2], vec![3], "Node 2 should have an edge to 3");
    }

    #[test]
    fn test_bfs() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        let distances = (&graph).bfs(0); 

        assert_eq!(distances.get(&1), Some(&1), "Distance from 0 to 1 should be 1");
        assert_eq!(distances.get(&2), Some(&2), "Distance from 0 to 2 should be 2");
        assert_eq!(distances.get(&3), Some(&3), "Distance from 0 to 3 should be 3");
    }

    #[test]
    fn test_degree_distribution() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        let degree_dist = degree_distribution(&graph);

        assert!(degree_dist.get(&1).is_some(), "Degree 1 should be present");
        assert_eq!(degree_dist[&1], 3, "There should be 3 nodes with degree 1");
    }

    #[test]
    fn test_connected_components() {
        let edges = vec![(0, 1), (1, 2), (3, 4)];
        let graph = Graph::create_undirected(5, &edges);

        let components = connected_components(&graph);

        assert_eq!(components.len(), 2, "There should be two connected components");
        assert!(components[0].contains(&0), "First component should contain node 0");
        assert!(components[1].contains(&3), "Second component should contain node 3");
    }
}

