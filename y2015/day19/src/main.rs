use std::collections::HashMap;

use regex::Regex;

mod part1;
mod part2;

fn main() {
    let (transform_map, initial_molecule) = encode(include_str!("../resources/input.txt"));
    part1::solve(&transform_map, &initial_molecule);
    part2::solve(include_str!("../resources/input.txt"), &initial_molecule);
}

fn encode(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let mut lines = input.lines();
    let mut transform_map = HashMap::new();
    let re = Regex::new(r"^(?<source>\w+) => (?<target>\w+)$").unwrap();
    for _ in 0..43 {
        let captures = re.captures(lines.next().unwrap()).unwrap();
        let source = String::from(&captures["source"]);
        let target = String::from(&captures["target"]);
        transform_map.entry(source).or_insert(vec![]).push(target);
    }
    lines.next();

    (transform_map, String::from(lines.next().unwrap()))
}
