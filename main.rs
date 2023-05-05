mod bfs;
use bfs::{average_distance, read_txt, read_csv};


fn main() {
    let fb_graph = read_txt("fb_data.txt");
    let fb_avg_distance = average_distance(&fb_graph);
    println!("Average distance between nodes in Facebook dataset: {}", fb_avg_distance);

    let deezer_graph = read_csv("deezer.csv");
    let deezer_avg_distance = average_distance(&deezer_graph);
    println!("Average distance between nodes in Deezer dataset: {}", deezer_avg_distance);

    let difference: f64 = fb_avg_distance - deezer_avg_distance;
    println!("Difference between the average distance between nodes in the datasets: {}", difference.abs());

}

