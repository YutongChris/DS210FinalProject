use petgraph::graph::Graph;

pub fn calculate_average_degree_centrality(graph: &Graph<u32, ()>) -> f64 {
    // Total degree is the sum of degrees for all nodes
    let total_degree: usize = graph.node_indices().map(|node| graph.neighbors(node).count()).sum();

    // Total number of nodes
    let num_nodes = graph.node_count();

    // Calculate average degree centrality
    if num_nodes == 0 {
        0.0 // Avoid division by zero
    } else {
        total_degree as f64 / num_nodes as f64
    }
}
