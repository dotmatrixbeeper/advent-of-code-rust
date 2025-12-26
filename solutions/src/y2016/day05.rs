use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub fn run(input: &str) {
    println!("======= DAY 5 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let step = 50_000_000;
    let mut range_start = 0;
    let mut result: Vec<char> = Vec::new();
    let thread_count = 10;
    while result.len() < 8 {
        let ranges = (0..thread_count)
            .map(|_| {
                let start = range_start;
                let end = range_start + step;
                range_start = end + 1;
                return (start, end);
            })
            .collect::<Vec<(u64, u64)>>();
        result.extend(generate_hash_list_without_position(input, &ranges));
    }

    println!("The password for the door is: {}", result[0..8].iter().collect::<String>());
    println!("---------------------");
}

fn generate_hash_list_without_position(seed: &str, ranges: &Vec<(u64, u64)>) -> Vec<char> {
    let arc_seed = Arc::new(seed.to_string());
    let handles = (0..(ranges.len())).map(| i | {
        let arc_clone = Arc::clone(&arc_seed);
        let range_start = ranges[i].0;
        let range_stop = ranges[i].1;
        let handle: JoinHandle<Option<Vec<(u64, char)>>> = thread::spawn(move || {
            let mut valids: Vec<(u64, char)> = Vec::new();
            for suffix in range_start..=range_stop {
                let hash = format!("{:x}", md5::compute(format!("{}{}", arc_clone, suffix)));
                if &hash[0..5] == "00000" {
                    valids.push((suffix, hash.chars().nth(5).unwrap()));
                }
            }
            if valids.is_empty() {
                return None;
            } else {
                return Some(valids)
            }
        });
        return handle;
    })
    .collect::<Vec<JoinHandle<_>>>();

    let mut results: Vec<(u64, char)> = handles
        .into_iter()
        .filter_map(| handle | handle.join().unwrap() )
        .flat_map(| val | val.into_iter())
        .collect();
    results.sort_by(|a, b| (a.0).cmp(&b.0));
    return results.iter().map(|&(_, c)| c).collect();
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let step = 5_000_000;
    let mut range_start = 0;
    let mut result: Vec<(u32, char)> = Vec::new();
    let thread_count = 10;
    while result.len() < 8 {
        let ranges = (0..thread_count)
            .map(|_| {
                let start = range_start;
                let end = range_start + step;
                range_start = end + 1;
                return (start, end);
            })
            .collect::<Vec<(u64, u64)>>();
        place_in_result(&mut result, generate_hash_list_with_position(input, &ranges));
    }

    println!("The password for the door is: {}", result.iter().map(|(_, c)| c).collect::<String>());
    println!("---------------------");
}

fn generate_hash_list_with_position(seed: &str, ranges: &Vec<(u64, u64)>) -> Vec<(u32, char)> {
    let arc_seed = Arc::new(seed.to_string());
    let handles = (0..(ranges.len())).map(| i | {
        let arc_clone = Arc::clone(&arc_seed);
        let range_start = ranges[i].0;
        let range_stop = ranges[i].1;
        let handle: JoinHandle<Option<Vec<(u64, u32, char)>>> = thread::spawn(move || {
            let mut valids: Vec<(u64, u32, char)> = Vec::new();
            for suffix in range_start..=range_stop {
                let hash = format!("{:x}", md5::compute(format!("{}{}", arc_clone, suffix)));
                let position = hash.chars().nth(5).unwrap().to_digit(10);
                if &hash[0..5] == "00000" && position.is_some() {
                    valids.push((suffix, position.unwrap(), hash.chars().nth(6).unwrap()));
                }
            }
            if valids.is_empty() {
                return None;
            } else {
                return Some(valids)
            }
        });
        return handle;
    })
    .collect::<Vec<JoinHandle<_>>>();

    let mut results: Vec<(u64, u32, char)> = handles
        .into_iter()
        .filter_map(| handle | handle.join().unwrap() )
        .flat_map(| val | val.into_iter())
        .collect();
    results.sort_by(|a, b| (a.0).cmp(&b.0));
    return results.iter().map(|&(_, pos, c)| (pos, c)).collect();
}

fn place_in_result(result: &mut Vec<(u32, char)>, generated: Vec<(u32, char)>) {
    for (pos, c) in generated {
        let found_indices = result.iter().map(|(pos, _)| *pos).collect::<Vec<u32>>();
        if !found_indices.contains(&pos) && pos < 8 {
            result.push((pos, c));
        } 
    }
    result.sort_by(|(pos_1, _), (pos_2, _)| pos_1.cmp(&pos_2));
}