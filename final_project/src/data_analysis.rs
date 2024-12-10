use petgraph::graph::Graph;
use crate::AmazonDataCleaner;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

pub fn calculate_average_degree_centrality(graph: &Graph<(u32, String), ()>) -> f64 {
    let total_nodes = graph.node_count();
    if total_nodes == 0 {
        return 0.0; 
    }

    let total_degree: usize = graph.node_indices()
        .map(|node| graph.neighbors(node).count())
        .sum();

    total_degree as f64 / total_nodes as f64
}

pub struct AmazonDataAnalysis;

impl AmazonDataAnalysis {
    /// Calculates the average category co-purchases for each category
    pub fn calculate_co_purchase_ratios(
        global_graph: &Graph<(u32, String), ()>,
    ) -> HashMap<String, (f64, f64)> {
        let mut category_ratios = HashMap::new();
    
        // To keep track of edges
        let mut category_edge_counts = HashMap::new();
    
        // Iterate over all edges in the graph
        for edge in global_graph.edge_indices() {
            // Get source and target nodes for the edge
            if let Some((source_node, target_node)) = global_graph.edge_endpoints(edge) {
                let source_category = global_graph.node_weight(source_node).map(|(_, cat)| cat);
                let target_category = global_graph.node_weight(target_node).map(|(_, cat)| cat);
    
                if let (Some(source_cat), Some(target_cat)) = (source_category, target_category) {
                    // Debug: Print each edge and its categories
                    println!(
                        "Debug - Edge: Source Category: {}, Target Category: {}",
                        source_cat, target_cat
                    );
    
                    // Increment cross-category or in-category counters
                    let counts = category_edge_counts
                        .entry(source_cat.clone())
                        .or_insert((0, 0)); // Initialize tuple (in_category_edges, cross_category_edges)
    
                    if source_cat == target_cat {
                        counts.0 += 1; // Increment in-category edges
                        println!(
                            "Debug - In-Category Edge: Source: {}, Target: {}",
                            source_cat, target_cat
                        );
                    } else {
                        counts.1 += 1; // Increment cross-category edges
                        println!(
                            "Debug - Cross-Category Edge: Source: {}, Target: {}",
                            source_cat, target_cat
                        );
                    }
                }
            }
        }
    
        // Calculate ratios for each category
        for (category, (in_category_edges, cross_category_edges)) in &category_edge_counts {
            let total_edges = in_category_edges + cross_category_edges;
    
            // Debug: Print total edges for the category
            println!(
                "Debug - Category: {}, In-Category Edges: {}, Cross-Category Edges: {}, Total Edges: {}",
                category, in_category_edges, cross_category_edges, total_edges
            );
    
            let in_category_ratio = if total_edges > 0 {
                *in_category_edges as f64 / total_edges as f64
            } else {
                0.0
            };
    
            let cross_category_ratio = if total_edges > 0 {
                *cross_category_edges as f64 / total_edges as f64
            } else {
                0.0
            };
    
            // Debug: Print ratios
            println!(
                "Debug - Category: {}, In-Category Ratio: {:.2}, Cross-Category Ratio: {:.2}",
                category, in_category_ratio, cross_category_ratio
            );
    
            category_ratios.insert(category.clone(), (in_category_ratio, cross_category_ratio));
        }
    
        category_ratios
    }
    
}

    
