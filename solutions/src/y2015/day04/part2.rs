/// ### Solution for Part 2
/// This is more of the same problem, only this time we need to match against six zeros 
/// instead of the five zeros from the first part of the problem. However we do need to 
/// expand the range of numbers to search through.
/// 
/// #### Rust Implementation Details
/// We take groups of 2_000,000 starting from 1 and spread the processing across
/// five threads that simultaneously try to find the required hash. 
/// Once joined the handles contain Option<u32>
/// We can then filter through the handles and find the minimum suffix

use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub const RANGE: [(u32, u32); 5] = [(1, 2_000_000), (2_000_001, 4_000_000), (4_000_001, 6_000_000), (6_000_001, 8_000_000), (8_000_001, 10_000_000)];

pub fn solve(seed: &str) {
    let arc_seed = Arc::new(seed.to_string());
    let mut handles = vec![];
    (0..5).for_each(| i | {
        let arc_clone = Arc::clone(&arc_seed);
        let handle: JoinHandle<Option<u32>> = thread::spawn( move || {
            for suffix in RANGE[i].0..=RANGE[i].1 {
                if &format!("{:x}", md5::compute(format!("{}{}", arc_clone, suffix)))[0..6] == "000000" {
                    return Some(suffix);
                }
            }
            return None;
        });
        handles.push(handle);
    });

    let results: Vec<u32> = handles
        .into_iter()
        .filter_map(| handle | handle.join().unwrap() )
        .collect();
    
    if let Some(min_val) = results.iter().min() {
        println!("Easiest coin comes from {}{}", seed, min_val);
    } else {
        println!("No coins here. Sorry Santa.");
    }
}