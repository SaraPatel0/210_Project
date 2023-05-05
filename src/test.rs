
use bfs::{bfs, average_distance, read_txt, read_csv};

fn test_read_txt_empty_file() {
    // Test case where the input txt file is empty
    let graph = read_txt("empty_test.txt");
    assert_eq!(graph.len(), 0);
}

fn test_read_csv() {
    // Checks read_csv output on small scale with 2 edges
    let graph = read_csv("test_input.txt");
    assert_eq!(graph.len(), 3);
    assert_eq!(graph[&1], vec![2]);
    assert_eq!(graph[&2], vec![1, 3]);
    assert_eq!(graph[&3], vec![2]);
}

fn test_bfs_shortest_path() {
    // Checks bfs output on small scale with 2 edges
    let graph = read_csv("test_input.csv");
    assert_eq!(bfs(&graph, 1, 3), Some(2));
}

fn test_bfs_no_path() {
    // Checks bfs with nonexistent edge
    let graph = read_csv("test_input.csv");
    assert_eq!(bfs(&graph, 1, 4), None);
}

