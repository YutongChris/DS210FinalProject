let category_a = graph.add_node("Category A");
let category_b = graph.add_node("Category B");
let category_c = graph.add_node("Category C");

graph.add_edge(category_a, category_b, ());
graph.add_edge(category_b, category_c, ());
graph.add_edge(category_a, category_c, ());

// Analyze graph properties using the functions in the data_analysis module
calculate_neighbors(&graph);
calculate_average_degree_centrality(&graph);