use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use rand::seq::SliceRandom;
use rand::thread_rng;

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

fn find_similar_dissimilar_users(edges: &[(usize, usize)]) -> Option<(usize, usize, f64)> {
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

    if let Some((&(user1, user2), &similarity)) = most_similar_pair {
        println!("Most similar or dissimilar users: ({}, {}) with similarity {}", user1, user2, similarity);
    }

    // Calculate Degree Distribution
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    for node in graph.keys() {
        let degree = graph.get(node).unwrap_or(&HashSet::new()).len();
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }
    println!("Degree Distribution:");
    let mut degree_vec: Vec<_> = degree_distribution.into_iter().collect();
    degree_vec.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for (degree, count) in degree_vec {
        println!("Degree {}: {}", degree, count);
    }

    None
}

fn main() {
    let filename = "/Users/ysfsouayah/SP2024/DS210/Rust/final_project/cleaned-twitter.txt";

    // Read the data file
    let mut edges = match read_data_file(filename) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading data file: {}", err);
            return;
        }
    };

    // Shuffle the edges randomly
    let mut rng = thread_rng();
    edges.shuffle(&mut rng);
    
    // Take the first 1000 edges as a sample
    let sample_size = 1000;
    edges.truncate(sample_size);

    // Find similar and dissimilar users
    if let Some((user1, user2, similarity)) = find_similar_dissimilar_users(&edges) {
        println!("Most similar or dissimilar users: ({}, {}) with similarity {}", user1, user2, similarity);
    }
}
