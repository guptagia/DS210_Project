use std::collections::{HashSet, VecDeque};  
use crate::graph::Graph;  

// Function to find connected components in an undirected graph
pub fn connected_components(graph: &Graph) -> Vec<HashSet<usize>> {
    let mut visited = HashSet::new();  
    let mut components = Vec::new();  
    
    for node in 0..graph.n {
        if !visited.contains(&node) {  
            let mut component = HashSet::new(); 
            let mut queue = VecDeque::new(); 
            
            queue.push_back(node);  
            visited.insert(node);  
            
          
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