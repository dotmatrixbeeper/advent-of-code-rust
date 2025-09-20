use std::collections::HashMap;
use regex::Regex;

mod part1;
mod part2;

pub type Distance = u32;
pub type NodeId = usize;
pub type Subset = u32;

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: usize,
    distances: Vec<Vec<Distance>>,
}

impl Graph {
    pub fn new(node_count: usize, distances: Vec<Vec<Distance>>) -> Self {
        let nodes = node_count;
        Self { nodes, distances }
    }

    pub fn distance(&self, from: NodeId, to: NodeId) -> Distance {
        self.distances[from][to]
    }

    pub fn node_count(&self) -> usize {
        self.nodes
    }
}

fn main() {
    let data = form_graph(include_str!("../resources/input.txt"));
    part1::solve(&data.1, &data.0);
    part2::solve(&data.1, &data.0);


}

fn form_graph(data: &str) -> (Graph, HashMap<NodeId, String>) {
    let re = Regex::new(r"^(?<from>\w+) to (?<to>\w+) = (?<distance>\d+)$").unwrap();
    let mut vertex_map: HashMap<String, NodeId> = HashMap::new();
    let mut distances: Vec<Vec<Distance>> = vec![vec![0; 10]; 10];
    let mut vertex_count = 0_usize;
    for line in data.lines() {
        let captures = re.captures(line).unwrap();
        let from = String::from(&captures["from"]);
        let to = String::from(&captures["to"]);
        let distance = captures["distance"].parse::<u32>().unwrap();

        let from_vertex = *vertex_map.entry(from).or_insert_with(|| { vertex_count += 1; return vertex_count - 1 } );
        let to_vertex = *vertex_map.entry(to).or_insert_with(|| { vertex_count += 1; return vertex_count - 1 } );
        distances[from_vertex as usize][to_vertex as usize] = distance;
        distances[to_vertex as usize][from_vertex as usize] = distance;
    }

    let inverted_map: HashMap<NodeId, String> = vertex_map.iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect();

    (Graph::new(vertex_count, distances), inverted_map)
}