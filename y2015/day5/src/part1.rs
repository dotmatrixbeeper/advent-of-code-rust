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