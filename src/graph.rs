use std::collections::VecDeque;
use crate::parser::ListOfEdges;  
#[derive(Debug)]
pub struct Graph {
    pub n: usize,  
    pub outedges: Vec<Vec<usize>>,  
}
#[allow(dead_code)]
impl Graph {
     fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph { n, outedges: vec![Vec::new(); n] };
        for &(u, v) in edges {
            if u < n && v < n {
                g.outedges[u].push(v);
            }
        }
        g.sort_graph_lists();
        g
    }

    pub fn create_undirected_optimized(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph { n, outedges: vec![Vec::new(); n] };
        for &(u, v) in edges {
            if u < n && v < n {
                g.outedges[u].push(v);
                g.outedges[v].push(u);  
            }
        }
        g.sort_graph_lists();
        g
    }

    fn sort_graph_lists(&mut self) {
        for list in &mut self.outedges {
            list.sort_unstable();  
            list.dedup();  
        }
    }

    
    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.n];  
        let mut queue = VecDeque::new();

        distances[start] = 0;
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[current];
            
            for &neighbor in &self.outedges[current] {
                if distances[neighbor] == usize::MAX {  
                    queue.push_back(neighbor);
                    distances[neighbor] = current_distance + 1;
                }
            }
        }

        distances
    }
}
