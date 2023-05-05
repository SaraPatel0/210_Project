use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_csv(file_name: &str) -> HashMap<i32, Vec<i32>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // Creates empty hashmap to organize all edges connected to a particular node
    let mut graph = HashMap::new();

    // Iterates through each edge and adds each node to its respective "friend's" vec
    for line in reader.lines() {
        let edge = line.unwrap();
        let nodes: Vec<i32> = edge.split(',').map(|n| n.parse().unwrap()).collect();

        if nodes.len() != 2 {
            panic!("Invalid input file format: each line must contain two nodes");
        }

        graph.entry(nodes[0]).or_insert(Vec::new()).push(nodes[1]);
        graph.entry(nodes[1]).or_insert(Vec::new()).push(nodes[0]);
    }

    graph
}

pub fn read_txt(file_name: &str) -> HashMap<i32, Vec<i32>> {
    // Same as read_csv but splits by spaces
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // Creates empty hashmap to organize all edges connected to a particular node
    let mut graph = HashMap::new();

    // Iterates through each edge and adds each node to its respective "friend's" vec
    for line in reader.lines() {
        let edge = line.unwrap();
        let nodes: Vec<i32> = edge.split_whitespace().map(|n| n.parse().unwrap()).collect();

        if nodes.len() != 2 {
            panic!("Invalid input file format: each line must contain two nodes");
        }

        graph.entry(nodes[0]).or_insert(Vec::new()).push(nodes[1]);
        graph.entry(nodes[1]).or_insert(Vec::new()).push(nodes[0]);
    }

    graph
}

pub fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32, end: i32) -> Option<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();

    visited.insert(start);
    queue.push_back(start);
    dist.insert(start, 0);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if node == end {
            return Some(dist[&node]);
        }

        for &neighbor in &graph[&node] {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
                dist.insert(neighbor, dist[&node] + 1);
            }
        }
    }

    None
}

pub fn average_distance(graph: &HashMap<i32, Vec<i32>>) -> f64 {
    let mut total_distance = 0;
    let mut num_pairs = 0;

    let nodes: HashSet<i32> = graph.keys().cloned().collect();

    for start in &nodes {
        for end in &nodes {
            if start < end {
                match bfs(graph, *start, *end) {
                    Some(distance) => {
                        total_distance += distance;
                        num_pairs += 1;
                    },
                    None => (),
                }
            }
        }
    }

    total_distance as f64 / num_pairs as f64
}

