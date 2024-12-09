mod data_processing;
mod data_analysis;
mod test;
use petgraph::Graph;
use data_processing::{AmazonDataCleaner, Category};
use data_analysis::{calculate_average_degree_centrality};
use std::fs;
use serde_json;
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cleaner = AmazonDataCleaner::new("amazon-meta.txt");
    cleaner.load_data()?;
    cleaner.clean_data();
    let top_categories = cleaner.summarize_top_categories();
    println!("Top 3 Categories:");
    for (category, count, avg_sales_rank, avg_rating) in &top_categories {
        println!("Category: {}", category);
        println!("  Number of Products: {}", count);
        println!("  Average Sales Rank: {:.2}", avg_sales_rank);
        match avg_rating {
            Some(rating) => println!(" Average Review Rating: {:.2}", rating),
            None => println!("  Average Review Rating: No reviews available"),
        }
    }

    let category_graphs = cleaner.create_graphs_for_top_categories(top_categories);

    for (category, graph) in &category_graphs {
        let avg_degree_centrality = calculate_average_degree_centrality(graph);
        println!(
            "Category: {}, Average Degree Centrality: {:.2}",
            category, avg_degree_centrality
        );
    }

    Ok(())
}