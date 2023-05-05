#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque, HashMap};

fn read_file(filename: &str) -> Vec<(u32, u32)> {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        if line.starts_with('#') {
            continue;
        }
        let nodes: Vec<&str> = line.split('\t').collect();
        let from_node: u32 = nodes[0].parse().expect("Invalid from node");
        let to_node: u32 = nodes[1].parse().expect("Invalid to node");
        edges.push((from_node, to_node));
    }

    edges
}

fn adjacency_list(edges: &[(u32, u32)]) -> HashMap<u32, Vec<u32>> {
    let mut adj_list = HashMap::new();
    for (from, to) in edges {
        adj_list.entry(*from).or_insert_with(Vec::new).push(*to);
    }
    adj_list
}

fn bfs(node: u32, adj_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut dist = HashMap::new();
    let mut visited = HashMap::new();
    let mut q = VecDeque::new();

    dist.insert(node, 0);
    visited.insert(node, true);
    q.push_back(node);

    while !q.is_empty() {
        let curr_node = q.pop_front().unwrap();

        for &neigh in adj_list.get(&curr_node).unwrap_or(&vec![]) {
            if !visited.contains_key(&neigh) {
                visited.insert(neigh, true);
                dist.insert(neigh, dist[&curr_node] + 1);
                q.push_back(neigh);
            }
        }
    }

    dist
}

fn bfs_all_nodes(adj_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, f64> {
    let mut avg_dists = HashMap::new();

    for node in adj_list.keys() {
        let dist = bfs(*node, adj_list);
        let sum_dists = dist.values().sum::<u32>() as f64;
        let num_nodes = dist.len() as f64 - 1.0; // subtract 1 for the starting node
        let avg_dist = sum_dists / num_nodes;
        avg_dists.insert(*node, avg_dist);
    }

    avg_dists
}

fn avg_degrees_of_separation(filename: &str) -> f64 {
    let edges = read_file(filename);
    let adj_list = adjacency_list(&edges);
    let avg_dists = bfs_all_nodes(&adj_list);
    let mut total_dist = 0.0;
    let mut num_nodes_with_valid_dist = 0;

    for (node, avg_dist) in &avg_dists {
        if !avg_dist.is_nan() {
            total_dist += avg_dist;
            num_nodes_with_valid_dist += 1;
        }
    }

    total_dist / num_nodes_with_valid_dist as f64
}
fn main() {
    let filename = "email-net.txt";
    let edges = read_file(filename);
    println!("Number of edges: {}", edges.len());
    let adj_list = adjacency_list(&edges);
    println!("{:?}", adj_list.len());
    let avg_dists = bfs_all_nodes(&adj_list);
    for (node, avg_dist) in &avg_dists {
        println!("Node {}: average distance {}", node, avg_dist);
    }
    let avg_degrees = avg_degrees_of_separation(filename);
    println!("Average degrees of separation: {}", avg_degrees);
}
