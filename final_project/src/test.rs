use crate::calculate_average_degree_centrality;
use std::fs;
use petgraph::Graph;
#[test]
fn test_calculate_average_degree_centrality() {
    // Create a small graph manually
    let mut graph = Graph::<u32, ()>::new();

    // Add nodes (products)
    let node_a = graph.add_node(1); // Product 1
    let node_b = graph.add_node(2); // Product 2
    let node_c = graph.add_node(3); // Product 3
    let node_d = graph.add_node(4); // Product 4

    // Add edges (connections between similar products)
    graph.add_edge(node_a, node_b, ()); // Product 1 -> Product 2
    graph.add_edge(node_b, node_a, ()); // Product 2 -> Product 1
    graph.add_edge(node_a, node_c, ()); // Product 1 -> Product 3
    graph.add_edge(node_c, node_a, ()); // Product 3 -> Product 1
    graph.add_edge(node_b, node_d, ()); // Product 2 -> Product 4
    graph.add_edge(node_d, node_b, ()); // Product 4 -> Product 2

    // Debug: Print adjacency list
    for node in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();
        println!("Node {:?} -> {:?}", node.index(), neighbors);
    }

    // Calculate the average degree centrality
    let avg_degree_centrality = calculate_average_degree_centrality(&graph);

    // Expected average degree centrality:
    // Node degrees: [2 (A), 2 (B), 1 (C), 1 (D)]
    // Sum of degrees = 2 + 2 + 1 + 1 = 6
    // Average = 6 / 4 = 1.5
    let expected_avg_degree_centrality = 1.5;

    // Assert the result
    assert!(
        (avg_degree_centrality - expected_avg_degree_centrality).abs() < f64::EPSILON,
        "Expected: {:.2}, Got: {:.2}",
        expected_avg_degree_centrality,
        avg_degree_centrality
    );
}
