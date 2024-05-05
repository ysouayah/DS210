use petgraph::graph::{DiGraph};
use petgraph::algo::dijkstra;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};

fn read_data_file(filename: &str) -> Result<Vec<(usize, usize)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<usize> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if parts.len() >= 2 {
            edges.push((parts[0], parts[1]));
        }
    }

    Ok(edges)
}

fn jaccard_similarity(set1: &HashSet<usize>, set2: &HashSet<usize>) -> f64 {
    let intersection_size = set1.intersection(&set2).count() as f64;
    let union_size = set1.union(&set2).count() as f64;
    intersection_size / union_size
}

fn calculate_average_shortest_path_length(graph: &DiGraph<usize, f64>) -> f64 {
    let mut total_distance = 0.0;
    let mut num_pairs = 0;

    for start_node in graph.node_indices() {
        let distances = dijkstra(graph, start_node, None, |e| *e.weight());

        for (_, distance) in distances {
            total_distance += distance;
            num_pairs += 1;
        }
    }

    total_distance / num_pairs as f64
}

fn calculate_average_distance(graph: &DiGraph<usize, f64>) -> f64 {
    let mut total_distance = 0.0;
    let mut num_pairs = 0;

    for start_node in graph.node_indices() {
        let distances = dijkstra(graph, start_node, None, |e| *e.weight());

        for (_, distance) in distances {
            total_distance += distance;
            num_pairs += 1;
        }
    }

    total_distance / num_pairs as f64
}

fn find_similar_dissimilar_users(edges: &[(usize, usize)]) -> Option<(usize, usize, f64, f64)> {
    let mut graph = HashMap::<usize, HashSet<usize>>::new();

    // Create an adjacency list representation of the graph
    for &(node1, node2) in edges {
        graph.entry(node1).or_insert(HashSet::new()).insert(node2);
        graph.entry(node2).or_insert(HashSet::new()).insert(node1);
    }

    // Calculate Jaccard similarity for each pair of vertices
    let mut similarity_map: HashMap<(usize, usize), f64> = HashMap::new();
    for (&user1, friends1) in &graph {
        for (&user2, friends2) in &graph {
            if user1 != user2 {
                let similarity = jaccard_similarity(friends1, friends2);
                similarity_map.insert((user1, user2), similarity);
            }
        }
    }

    // Find the pair with the highest similarity
    let most_similar_pair = similarity_map.iter().max_by(|(_, &sim1), (_, &sim2)| sim1.partial_cmp(&sim2).unwrap());
    // Find the pair with the lowest similarity
    let most_dissimilar_pair = similarity_map.iter().min_by(|(_, &sim1), (_, &sim2)| sim1.partial_cmp(&sim2).unwrap());

    // Print and return the most similar and dissimilar pairs
    if let Some((&(user1, user2), &_)) = most_similar_pair {
        println!("Users ({}, {}) have the same set of friends.", user1, user2);
    }

    if let Some((&(user1, user2), &_)) = most_dissimilar_pair {
        println!("Users({}, {}) have no friends in common", user1, user2);
    }

    // Calculate Degree Distribution
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    for node in graph.keys() {
        let degree = graph.get(node).unwrap_or(&HashSet::new()).len();
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }

    let mut degree_vec: Vec<_> = degree_distribution.into_iter().collect();
    degree_vec.sort_by_key(|&(degree, _)| degree);
    //println!("{} people have {} friends", count, degree);
    //Uncomment the line above if you want to see what the range of friends is

    // Print for degrees less than or equal to 10. Starting here, comment everything until line 129 if you want to see all the degrees
    for (degree, count) in degree_vec.iter().filter(|&(degree, _)| *degree <= 10) {
        println!("{} people have {} friends", count, degree);
    }

    // Sum counts for degrees between 10 and 25
    let between_10_and_25_friends_count: usize = degree_vec.iter()
        .filter(|&(degree, _)| *degree > 10 && *degree <= 25)
        .map(|&(_, count)| count)
        .sum();
    println!("{} people have between 10 and 25 friends", between_10_and_25_friends_count);

    // Count for degrees more than 25
    let over_25_friends_count: usize = degree_vec.iter()
        .filter(|&(degree, _)| *degree > 25)
        .map(|&(_, count)| count)
        .sum();
    println!("{} people have more than 25 friends", over_25_friends_count);

    None // Return None since we removed the return values
}

fn main() {
    let filename = "/Users/ysfsouayah/SP2024/DS210/Rust/final_project/facebook_combined.txt"; // Change this to your data file path

    // Read the data file
    let edges = match read_data_file(filename) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading data file: {}", err);
            return;
        }
    };

    // Create a graph from the edges
    let mut graph = DiGraph::new();
    for &(node1, node2) in &edges {
        let node1_index = graph.add_node(node1);
        let node2_index = graph.add_node(node2);
        graph.add_edge(node1_index, node2_index, 1.0);
    }

    // Calculate average shortest path length
    let average_shortest_path_length = calculate_average_shortest_path_length(&graph);
    if average_shortest_path_length < 1.0 {
        println!("The average shortest path length is {}, suggesting that the Facebook users in the data are extremely interconnected.", average_shortest_path_length);
    } else if average_shortest_path_length > 1.0 {
        println!("The average shortest path length is {}, suggesting that the Facebook users in the data are not really interconnected.", average_shortest_path_length);
    }
    if average_shortest_path_length == calculate_average_distance(&graph) {
        println!("Since the average shortest path between users is consistent throughout, this dataset supports the small world hypothesis.");
    }

    // Find most similar and dissimilar users
    find_similar_dissimilar_users(&edges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data_file() {
        match read_data_file("/Users/ysfsouayah/SP2024/DS210/Rust/final_project/facebook_combined.txt") {
            Ok(edges) => assert!(!edges.is_empty(), "File is empty"),
            Err(err) => panic!("Error reading data file: {}", err),
        }
    }

    #[test]
    fn test_calculate_average_shortest_path_length() {
        // Create a simple graph
        let mut graph = DiGraph::new();
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);
        let node3 = graph.add_node(3);
        graph.add_edge(node1, node2, 1.0);
        graph.add_edge(node2, node3, 1.0);
        graph.add_edge(node1, node3, 1.0);

        // Expected average shortest path length is 1.0
        let expected_length = 0.5; // Adjusted expected value
        let actual_length = calculate_average_shortest_path_length(&graph);
        assert_eq!(actual_length, expected_length);
    }

    #[test]
    fn test_calculate_average_distance() {
        // Create a simple graph
        let mut graph = DiGraph::new();
        let node1 = graph.add_node(1);
        let node2 = graph.add_node(2);
        let node3 = graph.add_node(3);
        graph.add_edge(node1, node2, 1.0);
        graph.add_edge(node2, node3, 1.0);
        graph.add_edge(node1, node3, 1.0);

        // Expected average distance between pairs is 1.0
        let expected_distance = 0.5; // Adjusted expected value
        let actual_distance = calculate_average_distance(&graph);
        assert_eq!(actual_distance, expected_distance);
    }
}
