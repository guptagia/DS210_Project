// src/graph.rs
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::parser::ListOfEdges;

pub type Vertex = usize;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // number of vertices
    pub outedges: AdjacencyLists, // adjacency lists representing out-edges
}

impl Graph {
    // Create a directed graph from a list of edges
    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph { n, outedges: vec![vec![]; n] };
        for (u, v) in edges {
            g.outedges[*u].push(*v);
        }
        g.sort_graph_lists();
        g
    }

    // Create an undirected graph from a list of edges
    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        for (u, v) in edges {
            g.outedges[*u].push(*v);
            g.outedges[*v].push(*u); // Add the reverse edge for undirected graphs
        }
        g.sort_graph_lists();
        g
    }

    // Sort the adjacency lists to maintain order
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    // Breadth-First Search to calculate distances from a starting node
    pub fn bfs(&self, start: Vertex) -> HashMap<Vertex, usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();

        queue.push_back(start);
        distances.insert(start, 0);

        while let Some(current) = queue.pop_front() {
            let current_distance = *distances.get(&current).unwrap();

            for &neighbor in &self.outedges[current] {
                if !distances.contains_key(&neighbor) {
                    queue.push_back(neighbor);
                    distances.insert(neighbor, current_distance + 1);
                }
            }
        }

        distances
    }
}
