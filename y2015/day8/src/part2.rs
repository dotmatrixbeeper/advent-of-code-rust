/// ### Solution for Part 2
/// This extends the part 1 problem by asking to wrap each 
/// of the given strings in additional double quotes `""`
/// This is much easier than part 1 since all we have to do is
/// map the relevant characters in escape sequences.
/// 
/// #### Rust Implementation Details
/// The actual in-memory length is stored in `raw_string_len`
/// The code representation length is stored in `decoded_string_len`
/// 
/// We map each `\` to mean two characters since it has to be escaped
/// as `\\`.
/// We map each `"` to mean two characters since it has to be escaped
/// as `\"`.
/// All other characters can continue how they are

pub fn solve(data: &Vec<&str>) {
    let mut raw_string_len: u32 = 0;
    let mut encoded_string_len: u32 = 0;
    for line in data {
        raw_string_len += line.len() as u32;
        encoded_string_len += encode_line(line) + 2;
    }

    println!("Difference: {}", encoded_string_len - raw_string_len);
}

fn encode_line(line: &str) -> u32 {
    line.chars()
        .map(|c| {
            if c == '\\' {
                2
            } else if c == '"' {
                2
            } else {
                1
            }
        })
        .sum()
}