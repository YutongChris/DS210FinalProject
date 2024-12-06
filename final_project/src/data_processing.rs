use std::collections::HashMap;
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
    filepath: String,
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

    pub fn save_clean_data(&self, output_filepath: &str) -> Result<(), Box<dyn Error>> {
        let serialized_data = serde_json::to_string(&self.data)?;
        fs::write(output_filepath, serialized_data)?;
        Ok(())
    }

    pub fn summarize_by_category(&self, min_products: usize) {
        // Step 1: Count the number of products in each category
        let mut category_counts: HashMap<String, usize> = HashMap::new();
        for product in &self.data {
            if let Some(category) = product.group.clone() {
                *category_counts.entry(category).or_insert(0) += 1;
            }
        }

        // Step 2: Filter categories with more than `min_products` products
        let filtered_categories: Vec<_> = category_counts
            .iter()
            .filter(|&(_, &count)| count >= min_products)
            .collect();

        // Step 3: Calculate summary statistics for each category
        for (category, &count) in filtered_categories {
            let products_in_category: Vec<_> = self
                .data
                .iter()
                .filter(|p| p.group.as_ref() == Some(category))
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

            println!("Category: {}", category);
            println!("  Number of Products: {}", count);
            println!("  Average Sales Rank: {:.2}", avg_sales_rank);
            if avg_rating.is_nan() {
                println!("  Average Review Rating: No reviews available");
            } else {
                println!("  Average Review Rating: {:.2}", avg_rating);
            }
        }
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



pub fn create_graph_from_data(categories: Vec<Category>, edges: Vec<(usize, usize)>) -> Graph<Category, ()> {
    let mut graph = Graph::<Category, ()>::new();

    // Add nodes to the graph from the category data
    let mut nodes = HashMap::new();
    for (index, category) in categories.into_iter().enumerate() {
        let node_index = graph.add_node(category);
        nodes.insert(index, node_index);
    }

    // Add edges to the graph using the given indices
    for (source, target) in edges {
        if let (Some(&source_node), Some(&target_node)) = (nodes.get(&source), nodes.get(&target)) {
            graph.add_edge(source_node, target_node, ());
        }
    }

    graph
}



// Function to summarize data by category
pub fn summarize_by_category(graph: &Graph<Category, ()>) {
    let mut category_summary: HashMap<String, (usize, f64)> = HashMap::new();

    for node_index in graph.node_indices() {
        let category_name = &graph[node_index].name;
        let neighbors_count = graph.neighbors(node_index).count();

        category_summary.entry(category_name.clone()).or_insert((0, 0.0)).0 += neighbors_count;
    }

    let total_nodes = graph.node_count();
    let mut updated_summary: HashMap<String, (usize, f64)> = HashMap::new();
    for (category, (total_neighbors, _)) in &category_summary {
        let average_degree_centrality = *total_neighbors as f64 / total_nodes as f64;
        updated_summary.insert(category.clone(), (*total_neighbors, average_degree_centrality));
    }

    for (category, (neighbors, avg_centrality)) in &updated_summary {
        println!("Category: {}, Total Neighbors: {}, Average Degree Centrality: {:.2}", category, neighbors, avg_centrality);
    }
}
