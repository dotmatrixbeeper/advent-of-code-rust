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