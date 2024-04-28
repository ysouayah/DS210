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

fn calculate_global_clustering_coefficient(graph: &HashMap<usize, HashSet<usize>>) -> f64 {
    let mut total_triangles = 0;
    let mut total_possible_triangles = 0;

    // Iterate over each node in the graph
    for (_, neighbors) in graph {
        let num_neighbors = neighbors.len();
        if num_neighbors >= 2 {
            // Count the number of triangles the current node participates in
            let mut num_triangles = 0;
            for &neighbor1 in neighbors {
                for &neighbor2 in neighbors {
                    if neighbor1 != neighbor2 && graph.contains_key(&neighbor1) && graph[&neighbor1].contains(&neighbor2) {
                        num_triangles += 1;
                    }
                }
            }
            // Increment the total triangle count
            total_triangles += num_triangles;
            // Increment the total possible triangle count
            total_possible_triangles += num_neighbors * (num_neighbors - 1) / 2;
        }
    }

    // Calculate the global clustering coefficient
    if total_possible_triangles > 0 {
        total_triangles as f64 / total_possible_triangles as f64
    } else {
        0.0 // Return 0 if there are no possible triangles
    }
}

fn degree_assortativity(graph: &HashMap<usize, HashSet<usize>>) -> f64 {
    // Calculate the average degree of the network
    let avg_degree: f64 = graph.values().map(|neighbors| neighbors.len() as f64).sum::<f64>() / graph.len() as f64;

    // Calculate the correlation coefficient
    let mut numer = 0.0;
    let mut denom1 = 0.0;
    let mut denom2 = 0.0;

    for (node, neighbors) in graph {
        for &neighbor in neighbors {
            let deg_product = (graph[&node].len() as f64 - avg_degree) * (graph[&neighbor].len() as f64 - avg_degree);
            numer += deg_product;
            denom1 += deg_product.powi(2);
            denom2 += (graph[&node].len() as f64 - avg_degree).powi(2) * (graph[&neighbor].len() as f64 - avg_degree).powi(2);
        }
    }

    numer / (denom1 * denom2.sqrt())
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

    if let Some((&(user1, user2), &similarity)) = most_similar_pair {
        println!("Most similar or dissimilar users: ({}, {}) with similarity {}", user1, user2, similarity);
    }

    // Calculate Degree Distribution
    let mut degree_distribution: HashMap<usize, usize> = HashMap::new();
    for node in graph.keys() {
        let degree = graph.get(node).unwrap_or(&HashSet::new()).len();
        *degree_distribution.entry(degree).or_insert(0) += 1;
    }

    let mut degree_vec: Vec<_> = degree_distribution.into_iter().collect();
    degree_vec.sort_by_key(|&(degree, _)| degree);

    // Print for degrees less than or equal to 10
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

    // Calculate clustering coefficient for the graph
    let clustering_coefficient = calculate_global_clustering_coefficient(&graph);
    println!("The clustering coefficient is {}, so people are not likely to be friends of friends.", clustering_coefficient);

    // Calculate degree assortativity
    let assortativity = degree_assortativity(&graph);
    if assortativity > 0.5 {
        println!("The Degree Assortativity is {}, so people are likely to be connected with friends of friends", assortativity);
    } else {
        println!("The Degree Assortativity is {}, so people are not likely to be connected with friends of friends", assortativity);
    }

    None
}

fn main() {
    let filename = "/Users/ysfsouayah/SP2024/DS210/Rust/final_project/cleaned-twitter.txt"; //Will change depending on where the file is located on your computer

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
    
    // Take the first 10,000 edges as a sample
    let sample_size = 10_000;
    edges.truncate(sample_size);

    // Find similar and dissimilar users
    if let Some((user1, user2, similarity, clustering_coefficient)) = find_similar_dissimilar_users(&edges) {
        println!("Most similar or dissimilar users: ({}, {}) with similarity {} and clustering coefficient {}", user1, user2, similarity, clustering_coefficient);
    }
}
