use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use crate::RANGE;

pub fn solve(seed: &str) {
    let arc_seed = Arc::new(seed.to_string());
    let mut handles = vec![];
    (0..5).for_each(| i | {
        let arc_clone = Arc::clone(&arc_seed);
        let handle: JoinHandle<Option<u32>> = thread::spawn( move || {
            for suffix in RANGE[i].0..=RANGE[i].1 {
                if &format!("{:x}", md5::compute(format!("{}{}", arc_clone, suffix)))[0..5] == "00000" {
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