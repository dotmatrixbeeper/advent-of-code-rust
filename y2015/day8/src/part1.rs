pub fn solve(data: &Vec<&str>) {
    let mut raw_string_len: u32 = 0;
    let mut decoded_string_len: u32 = 0;
    for line in data {
        raw_string_len += line.len() as u32;
        decoded_string_len += decode_line(line.as_bytes());
    }

    println!("Difference: {}", raw_string_len - decoded_string_len);
}

fn decode_line(line: &[u8]) -> u32 {
    let mut i: usize = 1;
    let mut count: u32 = 0;
    while i < line.len() - 1 {
        if line[i] == b'\\' {
            if line[i + 1] == b'x' {
                i += 4;
            } else {
                i += 2;
            }
        } else {
            i +=1;
        }
        count += 1;
    }
    count
}