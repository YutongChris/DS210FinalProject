mod data_processing;
mod data_analysis;

use data_processing::{AmazonDataCleaner,create_graph_from_data, Category, summarize_by_category};
use data_analysis::{calculate_neighbors, calculate_average_degree_centrality};
use std::fs;
use serde_json;
use std::error::Error;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cleaner = AmazonDataCleaner::new("amazon-meta.txt");
    cleaner.load_data()?;
    cleaner.clean_data();


    cleaner.summarize_by_category(50);

    

    Ok(())
}
