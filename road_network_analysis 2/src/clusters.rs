// Function to find connected components in an undirected graph
fn connected_components(graph: &Graph) -> Vec<HashSet<Node>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for node in graph.adjacency_list.keys() {
        if !visited.contains(node) {
            let mut component = HashSet::new();
            let mut queue = VecDeque::new();

            queue.push_back(node.clone());
            visited.insert(node.clone());

            while let Some(current) = queue.pop_front() {
                component.insert(current.clone());

                for neighbor in graph.adjacency_list.get(&current).unwrap() {
                    if !visited.contains(neighbor) {
                        queue.push_back(neighbor.clone());
                        visited.insert(neighbor.clone());
                    }
                }
            }

            components.push(component);
        }
    }

    components
}
