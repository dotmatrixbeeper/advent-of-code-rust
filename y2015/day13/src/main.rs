mod part1;
mod part2;

use std::collections::HashMap;

use regex::Regex;

pub struct Graph {
    nodes: usize,
    distances: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new(node_count: usize, distances: Vec<Vec<i32>>) -> Self {
        Self { nodes: node_count, distances: distances }
    }

    pub fn set_distance(&mut self, from: usize, to: usize, dist: i32) {
        self.distances[from][to] = dist;
    }

    pub fn set_node_size(&mut self, node_count: usize) {
        self.nodes = node_count;
    } 

    pub fn distance(&self, from: usize, to: usize) -> i32 {
        self.distances[from][to]
    }

    pub fn node_count(&self) -> usize {
        self.nodes
    }
}

fn main() {

    let mut data = convert_to_map(include_str!("../resources/input.txt"));
    part1::solve(&data.0, &data.1);
    part2::solve(&mut data.0, &mut data.1);
}

fn convert_to_map(input: &str) -> (Graph, HashMap<usize, String>)  {
    let re = Regex::new(r"^(?<from>\w+) would (?<verb>lose|gain) (?<value>\d+) happiness units by sitting next to (?<to>\w+).$").unwrap();
    let mut vertex_map = HashMap::new();
    let mut vertex_count = 0;
    let mut distances = vec![vec![0; 10]; 10];
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let mut value = captures["value"].parse::<i32>().unwrap();
        match &captures["verb"] {
            "lose" => value = value * -1,
            _ => ()
        };
        let from = String::from(&captures["from"]);
        let to = String::from(&captures["to"]);
        let from_vertex = *vertex_map.entry(from).or_insert_with(|| { vertex_count += 1; return vertex_count - 1 });
        let to_vertex = *vertex_map.entry(to).or_insert_with(|| { vertex_count += 1; return vertex_count - 1 });
        distances[from_vertex][to_vertex] = value;
    }

    let inverted_map: HashMap<usize, String> = vertex_map.iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect();

    (Graph::new(vertex_count, distances), inverted_map)
    
}
