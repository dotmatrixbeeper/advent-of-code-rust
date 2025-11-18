/// ### Solution for Part 1
/// This requires us to sum all the numbers in the document,
/// thus we can just read all the numbers and then sum them.
/// 
/// #### Rust Implementation Details
/// We read the json text, and extract via regex all the numbers 
/// that occurs in it. 

use regex::Regex;

pub fn solve(input: &str) {
    let re = Regex::new(r"(?<number>-?\d+)").unwrap();
    println!("Sum of all numbers: {}", re.captures_iter(input)
        .map(|cap| *&cap["number"].parse::<i32>().unwrap())
        .sum::<i32>());
}