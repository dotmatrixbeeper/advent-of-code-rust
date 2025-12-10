/// ### Solution for Part 2
/// The second part requires a little more complex matching. 
/// We need to ensure that the repetation is not overlapping.
/// 
/// #### Rust Implementation Details
/// This filtering would take place as follows:
/// 1. Repeating pair: check the part of the string before current
///     position and the part of the string after current position 
///     for existance of current pair
/// 2. Letter pair: check the current position with the letter at
///     position +2

pub fn solve(inputs: &Vec<&str>) {
    let nice_lines = inputs.iter()
        .filter(| line | validate(line))
        .count();
    println!("Number of nice strings: {}", nice_lines);
}

fn repeats(key: &str, line: &str, i: usize) -> bool {
    return (&line)[0..i].contains(key) || (&line)[i+2..line.len()].contains(key);
}

fn validate(line: &str) -> bool{
    let line_chars: Vec<char> = line.chars().collect();
    let mut repeated = false;
    let mut pair_match = false;

    for i in 0..(line.len() - 2) {
        if line_chars[i] == line_chars[i + 2] {
            repeated = true;
        }

        if repeats(&line[i..(i+2)], line, i) {
            pair_match = true;
        }

        if repeated && pair_match {
            return true;
        }
    }
    false
}