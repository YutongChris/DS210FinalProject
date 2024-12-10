use plotters::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader}; 
use regex::Regex;
use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs;
//use serde_json;
use petgraph::Graph;
use petgraph::adj::NodeIndex;
use std::collections::HashMap;
use rand::seq::SliceRandom;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: u32,
    pub asin: Option<String>,
    pub title: Option<String>,
    pub group: Option<String>,
    pub salesrank: Option<u32>,
    pub similar: Vec<String>,
    pub categories: Option<u32>,
    pub category_list: Vec<String>,
    pub total_reviews: Option<u32>,
    pub downloaded_reviews: Option<u32>,
    pub avg_rating: Option<f32>,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Review {
    pub date: String,
    pub customer: String,
    pub rating: u32,
    pub votes: u32,
    pub helpful: u32,
}

pub struct AmazonDataCleaner {
    pub filepath: String,
    pub data: Vec<Product>, // Make the data field public
    
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub name: String,
}

impl AmazonDataCleaner {
    pub fn new(filepath: &str) -> Self {
        AmazonDataCleaner {
            filepath: filepath.to_string(),
            data: Vec::new(),
        }
    }

    pub fn get_data(&self) -> &Vec<Product> {
        &self.data
    }

    pub fn load_data(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::open(&self.filepath)?;
        let reader = BufReader::new(file);
        let mut products = Vec::new();
        let mut product = Product {
            id: 0,
            asin: None,
            title: None,
            group: None,
            salesrank: None,
            similar: Vec::new(),
            categories: None,
            category_list: Vec::new(),
            total_reviews: None,
            downloaded_reviews: None,
            avg_rating: None,
            reviews: Vec::new(),
        };
    
        // Updated Regex to match date, customer ID, rating, votes, and helpful counts
        let review_regex = Regex::new(r"(\d{4})-(\d{1,2})-(\d{1,2})\s+(?:customer|cutomer):\s+(\S+)\s+rating:\s+(\d+)\s+votes:\s+(\d+)\s+helpful:\s+(\d+)")?;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            //println!("Processing line: {}", line); // Debug: Print each line being processed
    
            if line.starts_with("Id: ") {
                if product.asin.is_some() {
                    //println!("Parsed Product: {:?}", product); // Debug statement to check parsed product
                    products.push(product);
                }
                product = Product {
                    id: line[4..].trim().parse().unwrap_or_default(),
                    asin: None,
                    title: None,
                    group: None,
                    salesrank: None,
                    similar: Vec::new(),
                    categories: None,
                    category_list: Vec::new(),
                    total_reviews: None,
                    downloaded_reviews: None,
                    avg_rating: None,
                    reviews: Vec::new(),
                };
            } else if line.starts_with("ASIN: ") {
                product.asin = Some(line[6..].trim().to_string());
            } else if line.starts_with("title: ") {
                product.title = Some(line[7..].trim().to_string());
            } else if line.starts_with("group: ") {
                product.group = Some(line[7..].trim().to_string());
            } else if line.starts_with("salesrank: ") {
                product.salesrank = Some(line[10..].trim().parse().unwrap_or_default());
            } else if line.starts_with("similar: ") {
                product.similar = line[9..].trim().split_whitespace().skip(1).map(|s| s.to_string()).collect();
            } else if line.starts_with("categories: ") {
                product.categories = Some(line[12..].trim().parse().unwrap_or_default());
            } else if line.starts_with("|") {
                product.category_list.push(line.to_string());
            } else if line.starts_with("reviews: total: ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 9 {
                    product.total_reviews = Some(parts[2].parse().unwrap_or_default());
                    product.downloaded_reviews = Some(parts[5].parse().unwrap_or_default());
                    product.avg_rating = Some(parts[8].parse().unwrap_or_default());
                }
            } else if review_regex.is_match(line) {
                let caps = review_regex.captures(line).unwrap();
                let review = Review {
                    date: format!("{}-{}-{}", &caps[1], &caps[2], &caps[3]),
                    customer: caps[4].to_string(),
                    rating: caps[5].parse().unwrap_or_default(),
                    votes: caps[6].parse().unwrap_or_default(),
                    helpful: caps[7].parse().unwrap_or_default(),
                };
                //println!("Parsed Review: {:?}", review); // Debug statement to ensure review is parsed correctly
                product.reviews.push(review);
            } else {
                //println!("Line did not match any expected pattern: {}", line); // Debug: Line did not match
            }
        }
    
        if product.asin.is_some() {
            //println!("Parsed Product: {:?}", product); // Debug statement for the last product
            products.push(product);
        }
    
        self.data = products;
        //println!("Total Products Loaded: {}", self.data.len()); // Debug to check the total products loaded
        Ok(())
    }
    

    pub fn clean_data(&mut self) {
        self.data.retain(|product| product.asin.is_some());

        let max_salesrank = self.data.iter().filter_map(|p| p.salesrank).max().unwrap_or(0);
        for product in &mut self.data {
            if product.title.is_none() {
                product.title = Some("Unknown".to_string());
            }
            if product.salesrank.is_none() {
                product.salesrank = Some(max_salesrank + 1);
            }
        }
    }

    pub fn get_clean_data(&self) -> &Vec<Product> {
        &self.data
    }

    pub fn random_sample(&self, sample_size: usize) -> Vec<Product> {
        let mut rng = rand::thread_rng();
        let sampled_data: Vec<Product> = self
            .data
            .choose_multiple(&mut rng, sample_size)
            .cloned()
            .collect();
        sampled_data
    }

    pub fn summarize_top_categories(&self) -> Vec<(String, usize, f64, Option<f64>)> {
        // Step 1: Count the number of products in each category
        let mut category_counts: HashMap<String, usize> = HashMap::new();
        for product in &self.data {
            if let Some(category) = product.group.clone() {
                *category_counts.entry(category).or_insert(0) += 1;
            }
        }
    
        // Step 2: Sort categories by count in descending order
        let mut sorted_categories: Vec<_> = category_counts.into_iter().collect();
        sorted_categories.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count in descending order
    
        // Take the top 3 categories
        let top_categories = sorted_categories.into_iter().take(3);
    
        // Step 3: Collect summary statistics for the top categories
        let mut summaries = Vec::new();
        for (category, count) in top_categories {
            let products_in_category: Vec<_> = self
                .data
                .iter()
                .filter(|p| p.group.as_ref() == Some(&category))
                .collect();
    
            let avg_sales_rank: f64 = products_in_category
                .iter()
                .filter_map(|p| p.salesrank)
                .map(|r| r as f64)
                .sum::<f64>()
                / count as f64;
    
            let avg_rating: f64 = products_in_category
                .iter()
                .flat_map(|p| p.reviews.iter())
                .map(|r| r.rating as f64)
                .sum::<f64>()
                / products_in_category
                    .iter()
                    .flat_map(|p| p.reviews.iter())
                    .count() as f64;
    
            summaries.push((
                category,
                count,
                avg_sales_rank,
                if avg_rating.is_nan() { None } else { Some(avg_rating) },
            ));
        }
        
        summaries
        
    }
    
    pub fn create_graphs_for_top_categories(
        &self,
        top_categories: Vec<(String, usize, f64, Option<f64>)>,
    ) -> HashMap<String, Graph<(u32, String), ()>> {
        let mut category_graphs = HashMap::new();
        let mut id_to_node_global = HashMap::new(); // Global mapping of product ID to node indices
    
        // Populate the global graph with all products
        for product in &self.data {
            let category = product.group.clone().unwrap_or_else(|| "Unknown".to_string());
            let node_index = id_to_node_global
                .entry(product.id)
                .or_insert_with(|| Graph::<(u32, String), ()>::new().add_node((product.id, category.clone())));
        }
    
        // Build category-specific graphs
        for (category, _, _, _) in top_categories {
            let mut graph = Graph::<(u32, String), ()>::new();
            let mut id_to_node_local = HashMap::new();
    
            // Filter products belonging to the current category
            let products_in_category: Vec<_> = self
                .data
                .iter()
                .filter(|p| p.group.as_ref() == Some(&category))
                .collect();
    
            // Add nodes for all products in the category
            for product in &products_in_category {
                let node_index = graph.add_node((product.id, category.clone()));
                id_to_node_local.insert(product.id, node_index);
            }
    
            // Add edges based on "similar" ASINs
            for product in &products_in_category {
                if let Some(&source_node) = id_to_node_local.get(&product.id) {
                    for similar_asin in &product.similar {
                        let similar_asin_normalized = similar_asin.trim().to_lowercase(); // Normalize similar ASIN
    
                        // Find the matching product in the global data
                        if let Some(similar_product) = self
                            .data
                            .iter()
                            .find(|p| p.asin.as_deref().map(|a| a.trim().to_lowercase()) == Some(similar_asin_normalized.clone()))
                        {
                            if let Some(&target_node) = id_to_node_local.get(&similar_product.id) {
                                // Add edge only if both source and target nodes are valid
                                graph.add_edge(source_node, target_node, ());
                            }
                        }
                    }
                }
            }
    
            category_graphs.insert(category.clone(), graph);
        }
    
        category_graphs
    }
    

    
     // Make `print_adjacency_list` a method
    pub fn print_adjacency_list(&self, graph: &Graph<(u32, String), ()>) {
        for node in graph.node_indices() {
            if let Some((product_id, category)) = graph.node_weight(node) {
                let neighbors: Vec<_> = graph.neighbors(node)
                    .filter_map(|n| graph.node_weight(n).map(|(id, _)| *id))
                    .collect();
                println!(
                    "Product ID: {}, Category: {} -> Neighbors: {:?}",
                    product_id, category, neighbors
                );
            }
        }
    }
    
    pub fn create_global_graph(&self) -> Graph<(u32, String), ()> {
        let mut global_graph = Graph::<(u32, String), ()>::new();
        let mut id_to_node = HashMap::new();

        // Add all products as nodes to the global graph
        for product in &self.data {
            let category = product.group.clone().unwrap_or("Unknown".to_string());
            let node_index = global_graph.add_node((product.id, category.clone()));
            id_to_node.insert(product.id, node_index);
        }

        // Add edges for all "similar" products
        for product in &self.data {
            if let Some(&source_node) = id_to_node.get(&product.id) {
                for similar_asin in &product.similar {
                    let similar_asin_normalized = similar_asin.trim().to_lowercase();

                    // Match similar product in the dataset
                    if let Some(similar_product) = self
                        .data
                        .iter()
                        .find(|p| p.asin.as_deref().map(|a| a.trim().to_lowercase()) == Some(similar_asin_normalized.clone()))
                    {
                        if let Some(&target_node) = id_to_node.get(&similar_product.id) {
                            global_graph.add_edge(source_node, target_node, ());
                        }
                    }
                }
            }
        }

        // Debug: Print edges with their categories
        for edge in global_graph.edge_indices() {
            if let Some((source, target)) = global_graph.edge_endpoints(edge) {
                let source_category = global_graph.node_weight(source).unwrap().1.clone();
                let target_category = global_graph.node_weight(target).unwrap().1.clone();
                println!(
                    "Edge: Source Category: {}, Target Category: {}",
                    source_category, target_category
                );
            }
        }

        global_graph
    }
}
    

impl Product {
    pub fn extract_features(&self) -> HashMap<String, f64> {
        let mut features = HashMap::new();

        // Number of Reviews
        let num_reviews = self.reviews.len() as f64;
        features.insert("num_reviews".to_string(), num_reviews);

        // Average Rating
        let avg_rating = if num_reviews > 0.0 {
            self.reviews.iter().map(|r| r.rating as f64).sum::<f64>() / num_reviews
        } else {
            0.0
        };
        features.insert("avg_rating".to_string(), avg_rating);

        // Sales Rank
        let sales_rank = self.salesrank.unwrap_or(0) as f64;
        features.insert("sales_rank".to_string(), sales_rank);

        // Category Hierarchy Depth
        let category_depth = self.category_list.len() as f64;
        features.insert("category_depth".to_string(), category_depth);

        // Text Length of Title
        let title_length = self.title.as_ref().map_or(0, |title| title.len()) as f64;
        features.insert("title_length".to_string(), title_length);

        features
    }

    
}
