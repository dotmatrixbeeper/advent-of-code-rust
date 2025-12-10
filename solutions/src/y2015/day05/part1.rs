/// ### Solution for Part 1
/// This is a filter problem. With the given input file, and the criterias
/// we need to find the valid line count. 
/// 
/// #### Rust Implementation Details
/// The filtering is easy enough with iterator filters:
/// 1. At least three vowels: for each like filter the vowes and count
/// 2. At least one repeating character: zip the chars in a line with chars 
///     of the same line from the second position onwards. Then check tuples
///     for equality
/// 3. Does not contain given strings: line.contains(string) will give us the
///     continer check

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn solve(inputs: &Vec<&str>) {
    let nice_lines = inputs.iter()
        .filter(| line | three_vowels(line) && repeating_char(line) && contains_strings(line))
        .count();
    println!("Number of nice strings: {}", nice_lines);
}

fn three_vowels(line: &str) -> bool {
    let vowel_count = line.chars()
        .filter(| c | VOWELS.contains(c))
        .count();

    return vowel_count >= 3;
}

fn repeating_char(line: &str) -> bool {
    line.chars()
        .zip(line.chars().skip(1))
        .any(| (a, b) | a == b)
}

fn contains_strings(line: &str) -> bool {
    !(line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy"))
}