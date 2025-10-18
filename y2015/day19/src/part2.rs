use regex::Regex;

use std::collections::HashMap;

pub fn solve(input: &str, initial_molecule: &str) {
    let transform_map = encode(input);
    let mut source_molecule = vectorize(initial_molecule);
    let mut transform_molecule = vec![];
    let mut step_count = 0;
    while source_molecule != vec!["O", "Mg"] && source_molecule != vec!["N", "Al"] && source_molecule != vec!["H", "F"] { 
        let mut current_index = 0;
        while current_index < source_molecule.len() {
            let mut molecule = source_molecule[current_index].clone();
            let mut match_found = false;
            for i in (2..=8).rev() {
                if (current_index + i) >= source_molecule.len() {
                    continue;
                }
                let temp = source_molecule[current_index..(current_index + i)].iter().cloned().collect::<String>();
                if transform_map.contains_key(&temp) {
                    match_found = true;
                    molecule = temp.clone();
                    current_index += i;
                    step_count += 1;
                    break;
                }
            }
            if match_found {
                transform_molecule.push(transform_map.get(&molecule).unwrap().clone());
            } else {
                current_index += 1;
                transform_molecule.push(molecule);
            }
        }
        source_molecule = transform_molecule.clone();
        transform_molecule = vec![];
    }

    println!("Steps to create target molecule from electron: {}", step_count + 1);
}

fn encode(input: &str) -> HashMap<String, String> {
    let mut lines = input.lines();
    let mut transform_map = HashMap::new();
    let re = Regex::new(r"^(?<source>\w+) => (?<target>\w+)$").unwrap();
    for _ in 0..40 {
        let captures = re.captures(lines.next().unwrap()).unwrap();
        let source = String::from(&captures["source"]);
        let target = String::from(&captures["target"]);
        transform_map.entry(target).or_insert(source);
    }

    transform_map
}

fn vectorize(initial_molecule: &str) -> Vec<String> {
    let mut char_iter = initial_molecule.chars().peekable();
    let mut result = vec![];
    while let Some(current) = char_iter.next() {
        if !current.is_lowercase() {
            if let Some(&next_char) = char_iter.peek() {
                if next_char.is_lowercase() {
                    result.push(format!("{}{}", current, next_char));
                } else {
                    result.push(current.to_string());
                }
            } else {
                result.push(current.to_string());
            }
        }
    }
    return result;
}