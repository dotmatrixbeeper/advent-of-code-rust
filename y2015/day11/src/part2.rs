use std::collections::HashSet;

pub fn solve() {
    let input = "hxbxxyzz";
    let mut input_mapping = input.chars()
        .map(|c| c as u8 - 'a' as u8)
        .collect::<Vec<u8>>();

    loop {
        increment(&mut input_mapping);
        if has_consecutive_trio(&input_mapping) && has_two_unique_pairs(&input_mapping) {
            println!("Next password: {}", input_mapping.iter().map(|n| ('a' as u8 + n) as char).collect::<String>());
            break;
        }
    }
}

fn increment(input: &mut Vec<u8>) {
    if let Some(last) = input.last_mut() {
        *last = safe_increment(*last);
    }

    for i in (1..input.len()).rev() {
        if input[i] > 25 {
            input[i] = 0;
            input[i - 1] = safe_increment(input[i - 1]);
        }
    }
}

fn safe_increment(value: u8) -> u8 {
    if [8, 11, 14].contains(&(value + 1)) {
        value + 2
    } else {
        value + 1
    }
}

fn has_consecutive_trio(input: &Vec<u8>) -> bool {
    input.windows(3)
        .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
}

fn has_two_unique_pairs(input: &Vec<u8>) -> bool {
    input.windows(2)
        .filter(|pair| pair[0] == pair[1])
        .collect::<HashSet<_>>()
        .iter().len() >= 2
}