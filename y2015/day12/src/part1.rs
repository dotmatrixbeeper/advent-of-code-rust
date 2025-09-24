use regex::Regex;

pub fn solve(input: &str) {
    let re = Regex::new(r"(?<number>-?\d+)").unwrap();
    println!("Sum of all numbers: {}", re.captures_iter(input)
        .map(|cap| *&cap["number"].parse::<i32>().unwrap())
        .sum::<i32>());
}