mod data_processing;
mod data_analysis;
mod test;
use data_processing::AmazonDataCleaner;
use data_analysis::{calculate_average_degree_centrality, AmazonDataAnalysis};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize AmazonDataCleaner with the dataset
    let mut cleaner = AmazonDataCleaner::new("amazon-meta.txt");
    cleaner.load_data()?; // Load the data from the file
    cleaner.clean_data(); // Clean the data (e.g., removing duplicates or invalid entries)

    // Generate a random sample 
    let sample_size = 100000;
    let random_sample = cleaner.random_sample(sample_size);

    // Create a new instance of AmazonDataCleaner for the random sample
    let sample_cleaner = AmazonDataCleaner {
        filepath: String::new(),
        data: random_sample, // Use the random sample as the dataset
    };

    // Summarize the top categories in the random sample
    let top_categories = sample_cleaner.summarize_top_categories();
    println!("Top Categories in Random Sample:");
    for (category, count, avg_sales_rank, avg_rating) in &top_categories {
        println!("Category: {}", category);
        println!("  Number of Products: {}", count);
        println!("  Average Sales Rank: {:.2}", avg_sales_rank);
        match avg_rating {
            Some(rating) => println!("  Average Review Rating: {:.2}", rating),
            None => println!("  Average Review Rating: No reviews available"),
        }
    }

    // Create graphs for the top categories in the random sample
    let category_graphs = sample_cleaner.create_graphs_for_top_categories(top_categories);

    // Calculate and display average degree centrality for each category's graph
    for (category, graph) in &category_graphs {
        let avg_degree_centrality = calculate_average_degree_centrality(graph);
        println!(
            "Average Degree Centrality for Category {}: {:.2}",
            category, avg_degree_centrality
        );
    } // products in the graph only contain one category

    // Create a global graph from the random sample
    let global_graph = sample_cleaner.create_global_graph();
    println!(
        "Global Graph created with {} nodes and {} edges.",
        global_graph.node_count(),
        global_graph.edge_count()
    );

    // Step 2: Calculate Co-Purchase Ratios
    let co_purchase_ratios = AmazonDataAnalysis::calculate_co_purchase_ratios(&global_graph);
    println!("\nCo-Purchase Ratios:");
    for (category, (in_category_ratio, cross_category_ratio)) in &co_purchase_ratios {
        println!(
            "Category: {} - In-Category Ratio: {:.2}, Cross-Category Ratio: {:.2}",
            category, in_category_ratio, cross_category_ratio
        );
    }

    Ok(())
}
