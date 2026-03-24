use std::collections::HashMap;

use hashbrownie::md5;

pub fn run(input: &str) {
    println!("======= DAY 14 ======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(salt: &str) {
    println!("Part 1 --------------");
    let mut found_hashes = 0usize;
    let mut triplet_records: HashMap<char, Vec<(String, usize, bool)>> = HashMap::new();
    let mut index: usize = 0;

    'outer: while found_hashes <= 64 {
        let hash = md5(&format!("{}{}", salt, index).as_bytes())
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        if let Some(triplet) = find_triplet(&hash) {
            let entry = triplet_records.entry(triplet).or_insert(vec![]);
            entry.push((hash.clone(), index, true));
        }

        if let Some(pentlet) = find_pentlet(&hash) {
            if let Some(record) = triplet_records.get_mut(&pentlet) {
                for i in 0..record.len() {
                    if index != record[i].1 && index - record[i].1 <= 1000 {
                        found_hashes += 1;
                        record[i].2 = false;
                    }

                    if found_hashes == 64 {
                        println!("index to reach 64th hash: {}", record[i].1);
                        break 'outer;
                    }
                }

                record.retain(|item| item.2);

                if record.is_empty() {
                    triplet_records.remove(&pentlet);
                }
            }
        }
        index += 1;
    }
    println!("---------------------");
}

fn find_triplet(hash: &str) -> Option<char> {
    let hash_bytes = hash.as_bytes();
    
    if hash_bytes.len() < 3 {
        return None;
    }

    for i in 0..=(hash_bytes.len() - 3) {
        if hash_bytes[i] == hash_bytes[i + 1] && hash_bytes[i + 1] == hash_bytes[i + 2] {
            return Some(hash_bytes[i] as char);
        }
    }
    return None;
}

fn find_pentlet(hash: &str) -> Option<char> {
    let hash_bytes = hash.as_bytes();
    
    if hash_bytes.len() < 5 {
        return None;
    }
    
    for i in 0..=(hash_bytes.len() - 5) {
        if hash_bytes[i] == hash_bytes[i + 1]
            && hash_bytes[i + 1] == hash_bytes[i + 2]
            && hash_bytes[i + 2] == hash_bytes[i + 3]
            && hash_bytes[i + 3] == hash_bytes[i + 4]
        {
            return Some(hash_bytes[i] as char);
        }
    }
    return None;
}

fn part_2(salt: &str) {
    println!("Part 2 --------------");
    let mut found_hashes = 0usize;
    let mut triplet_records: HashMap<char, Vec<(String, usize, bool)>> = HashMap::new();
    let mut index: usize = 0;

    'outer: while found_hashes <= 64 {
        let hash = md5_2017_times(&format!("{}{}", salt, index));
        if let Some(triplet) = find_triplet(&hash) {
            let entry = triplet_records.entry(triplet).or_insert(vec![]);
            entry.push((hash.clone(), index, true));
        }

        if let Some(pentlet) = find_pentlet(&hash) {
            if let Some(record) = triplet_records.get_mut(&pentlet) {
                for i in 0..record.len() {
                    if index != record[i].1 && index - record[i].1 <= 1000 {
                        found_hashes += 1;
                        record[i].2 = false;
                    }

                    if found_hashes == 64 {
                        println!("index to reach 64th hash: {}", record[i].1);
                        break 'outer;
                    }
                }

                record.retain(|item| item.2);

                if record.is_empty() {
                    triplet_records.remove(&pentlet);
                }
            }
        }
        index += 1;
    }
    println!("---------------------");
}

fn md5_2017_times(data: &str) -> String {
    let mut hash = String::from(data);
    for _ in 0..2017 {
        hash = md5(hash.as_bytes())
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
    }
    hash
}
