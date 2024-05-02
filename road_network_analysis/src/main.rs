mod parser;
mod graph;
mod analysis;
mod clusters;
fn main() {
    let edges = parser::read_file("roadNet-CA.txt"); 

    // Creating an undirected graph from the list of edges
    let graph = graph::Graph::create_undirected(1965206, &edges); 

    // Calculating average distance between pairs of vertices
    let avg_distance = analysis::average_distance(&graph);
    println!("Average distance between pairs of vertices: {}", avg_distance);

    // Analyzing the degree distribution
    let degree_dist = analysis::degree_distribution(&graph);
    println!("Degree distribution:");
    for (degree, count) in degree_dist {
        println!("Degree {}: {} vertices", degree, count);
    }

    // Identify connected components in the graph
    let components = analysis::connected_components(&graph);
    println!("Number of connected components: {}", components.len());
}

