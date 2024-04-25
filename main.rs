use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use petgraph::graph::{UnGraph};
use petgraph::algo::dijkstra;

fn read_data_file(filename: &str) -> Result<Vec<(usize, usize)>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let (Ok(node1), Ok(node2)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                edges.push((node1, node2));
            }
        }
    }

    Ok(edges)
}

fn calculate_average_distance(edges: &[(usize, usize)]) -> f64 {
    let mut graph = UnGraph::<usize, ()>::default();
    let mut node_indices: HashMap<usize, _> = HashMap::new();

    for &(node1, node2) in edges {
        let idx1 = *node_indices.entry(node1).or_insert_with(|| graph.add_node(node1));
        let idx2 = *node_indices.entry(node2).or_insert_with(|| graph.add_node(node2));
        graph.add_edge(idx1, idx2, ());
    }

    let mut total_distance = 0;
    let mut num_pairs = 0;
    for node in graph.node_indices() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        for (&target_node, &distance) in distances.iter() {
            if target_node != node {
                total_distance += distance;
                num_pairs += 1;
            }
        }
    }

    if num_pairs > 0 {
        total_distance as f64 / num_pairs as f64
    } else {
        0.0
    }
}

fn main() {
    let filename = "/Users/ysfsouayah/SP2024/DS210/Rust/final_project/cleaned-twitter.txt";

    // Read the data file
    let edges = match read_data_file(filename) {
        Ok(edges) => edges,
        Err(err) => {
            eprintln!("Error reading data file: {}", err);
            return;
        }
    };

    // Calculate average distance
    let average_distance = calculate_average_distance(&edges);

    println!("Average distance between users: {:.2}", average_distance);
}
