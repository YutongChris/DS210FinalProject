use petgraph::graph::Graph;
use crate::data_processing::Category;

pub fn calculate_neighbors(graph: &Graph<Category, ()>) {
    for node_index in graph.node_indices() {
        let neighbors_count = graph.neighbors(node_index).count();
        let category_name = &graph[node_index].name;
        println!("Category: {}, Number of Neighbors: {}", category_name, neighbors_count);
    }
}

pub fn calculate_average_degree_centrality(graph: &Graph<Category, ()>) {
    let total_nodes = graph.node_count();
    let mut total_degree = 0;

    for node_index in graph.node_indices() {
        let degree = graph.neighbors(node_index).count();
        total_degree += degree;
    }

    let average_degree_centrality = total_degree as f64 / total_nodes as f64;
    println!("Average Degree Centrality: {:.2}", average_degree_centrality);
}
