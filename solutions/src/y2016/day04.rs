use std::{cmp::Ordering, collections::HashMap};

use regex::Regex;

pub fn run(input: &str) {
    println!("======= DAY 4 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let re = Regex::new(r"^(?<name>[a-z-]+)-(?<id>\d+)\[(?<checksum>[a-z]+)\]$").unwrap();
    let sum_of_ids = input
        .lines()
        .filter_map(|line| {
            let captures = re.captures(line).unwrap();
            let id = &captures["id"];
            if get_checksum(&captures["name"]) == &captures["checksum"] {
                return Some(id.parse::<u32>().unwrap())
            } else {
                return None
            }
        })
        .sum::<u32>();
    println!("The sum of IDs: {}", sum_of_ids);
    println!("---------------------");
}

fn get_checksum(name: &str) -> String {
    // iterate letters -> collect only non-duplicates and frequencies going from highest to lowest
    // in case of tie, we take the alphabetical order
    let mut unique_string = Vec::new();
    let mut frequency_map = HashMap::new();
    name
        .chars()
        .for_each(|letter| {
            if letter != '-' {
                let value = frequency_map.entry(letter).or_insert_with(|| {
                    unique_string.push(letter); 
                    0
                });
                *value += 1;   
            }
            
        }
    );

    unique_string.sort_by(|a, b| {
        let freq_a = frequency_map.get(a).unwrap();
        let freq_b = frequency_map.get(b).unwrap();
        let freq_cmp = freq_b.cmp(freq_a);
        if freq_cmp == Ordering::Equal {
            a.cmp(b)
        } else {
            freq_cmp
        }
    });
    
    return unique_string[..5].iter().collect::<String>();
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let re = Regex::new(r"^(?<name>[a-z-]+)-(?<id>\d+)\[(?<checksum>[a-z]+)\]$").unwrap();
    let id = input
        .lines()
        .filter_map(|line| {
            let captures = re.captures(line).unwrap();
            let room_id = &captures["id"].parse::<u32>().unwrap();
            let name = &captures["name"];

            if get_checksum(name) == &captures["checksum"] && is_north(name, room_id) {
                return Some(*room_id);
            } else {
                return None;
            }
        })
        .next()
        .unwrap();
    println!("Id of room with North Pole objects: {}", id);
    println!("---------------------");
}

fn is_north(name: &str, id: &u32) -> bool {
    return name
        .chars()
        .map(|c| {
            if c == '-' {
                return ' ';
            } else {
                let index = ((c as u32 - 'a' as u32) + id) % 26;
                // something i learnt recently, adding to b'char' gives integer result
                return (b'a' + index as u8) as char
            }
        })
        .collect::<String>()
        .contains("north");
}