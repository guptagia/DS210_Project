mod parser;
mod analysis;
mod graph;
mod clusters; 

fn main() {
 
    match parser::read_file() {
        Ok(edges) => {
          
            let graph = graph::Graph::create_undirected(1965206, &edges);

            let avg_distance = analysis::average_distance(&graph);
            println!("Average distance between pairs of vertices: {}", avg_distance);

            

            let degree_dist = analysis::degree_distribution(&graph);
            println!("Degree distribution:");
            for (degree, count) in degree_dist {
                println!("Degree {}: {} vertices", degree, count);
            }

            let components = clusters::connected_components(&graph);
            println!("Number of connected components: {}", components.len());

            for (i, component) in components.iter().enumerate() {
                println!("Component {}: {:?}", i + 1, component);
            }
        }
        Err(e) => {
            // If read_file encounters an error, print the error
            println!("Error reading file: {:?}", e);
            return; // Exit the program if there's an error
        }
    }
}
