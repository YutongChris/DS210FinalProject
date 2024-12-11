use crate::calculate_average_degree_centrality;
use crate::AmazonDataAnalysis;
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::Graph;

    #[test]
fn test_adjacency_list() {
    let mut graph = Graph::<u32, ()>::new();

    let node_a = graph.add_node(1); // Product 1
    let node_b = graph.add_node(2); // Product 2
    let node_c = graph.add_node(3); // Product 3

    graph.add_edge(node_a, node_b, ()); // Product 1 -> Product 2
    graph.add_edge(node_b, node_c, ()); // Product 2 -> Product 3

    let expected_adjacency = vec![
        (1, vec![2]),
        (2, vec![3]),
        (3, vec![]),
    ];

    for (expected_product, expected_neighbors) in expected_adjacency {
        let node_index = graph
            .node_indices()
            .find(|&n| graph[n] == expected_product)
            .expect("Node not found");
        let neighbors: Vec<u32> = graph.neighbors(node_index).map(|n| graph[n]).collect();

        assert_eq!(
            neighbors, expected_neighbors,
            "Mismatch for Product {}: Expected {:?}, Got {:?}",
            expected_product, expected_neighbors, neighbors
        );
    }
}


    #[test]
    fn test_calculate_average_degree_centrality() {
        let mut graph = Graph::<(u32, String), ()>::new();

        let node_a = graph.add_node((1, "Books".to_string())); // Product 1 in Books
        let node_b = graph.add_node((2, "Books".to_string())); // Product 2 in Books
        let node_c = graph.add_node((3, "Music".to_string())); // Product 3 in Music
        let node_d = graph.add_node((4, "Books".to_string())); // Product 4 in Books


        graph.add_edge(node_a, node_b, ()); // Product 1 -> Product 2
        graph.add_edge(node_b, node_a, ()); // Product 2 -> Product 1
        graph.add_edge(node_a, node_c, ()); // Product 1 -> Product 3
        graph.add_edge(node_c, node_a, ()); // Product 3 -> Product 1
        graph.add_edge(node_b, node_d, ()); // Product 2 -> Product 4
        graph.add_edge(node_d, node_b, ()); // Product 4 -> Product 2


        let avg_degree_centrality = calculate_average_degree_centrality(&graph);

        let expected_avg_degree_centrality = 1.5;

        assert!(
            (avg_degree_centrality - expected_avg_degree_centrality).abs() < f64::EPSILON,
            "Expected: {:.2}, Got: {:.2}",
            expected_avg_degree_centrality,
            avg_degree_centrality
        );
    }

    
    #[test]
    fn test_calculate_co_purchase_ratios() {
        let mut global_graph = Graph::<(u32, String), ()>::new();

        let book_node_1 = global_graph.add_node((1, "Book".to_string()));
        let book_node_2 = global_graph.add_node((2, "Book".to_string()));
        let music_node_1 = global_graph.add_node((3, "Music".to_string()));
        let dvd_node_1 = global_graph.add_node((4, "DVD".to_string()));

        global_graph.add_edge(book_node_1, book_node_2, ()); // Book -> Book
        global_graph.add_edge(book_node_1, music_node_1, ()); // Book -> Music
        global_graph.add_edge(dvd_node_1, book_node_1, ());  // DVD -> Book
        global_graph.add_edge(music_node_1, music_node_1, ()); // Music -> Music (self-loop)

        let co_purchase_ratios = AmazonDataAnalysis::calculate_co_purchase_ratios(&global_graph);

        println!("Debug - Available Categories in Ratios: {:?}", co_purchase_ratios.keys());

        assert!(co_purchase_ratios.contains_key("Book"), "Book category not found in ratios");
        assert!(co_purchase_ratios.contains_key("Music"), "Music category not found in ratios");
        assert!(co_purchase_ratios.contains_key("DVD"), "DVD category not found in ratios");

        assert_eq!(
            co_purchase_ratios.get("Book").unwrap(),
            &(0.5, 0.5),
            "Book category ratios mismatch"
        );
        assert_eq!(
            co_purchase_ratios.get("Music").unwrap(),
            &(1.0, 0.0),
            "Music category ratios mismatch"
        );
        assert_eq!(
            co_purchase_ratios.get("DVD").unwrap(),
            &(0.0, 1.0),
            "DVD category ratios mismatch"
        );
    }
}
