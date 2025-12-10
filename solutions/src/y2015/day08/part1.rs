/// ### Solution for Part 1
/// The problem requires us to count the difference between 
/// the in-memory literal length and the code representation
/// of the literals. To do this we must transalte the literals
/// into their code representations.
/// 
/// #### Rust Implementation Details
/// In each line we traverse the code-points to find how many 
/// code representations are there. 
/// The actual in-memory length is stored in `raw_string_len`
/// The code representation length is stored in `decoded_string_len`
/// 
/// For counting code representation each line we follow the 
/// traversal rules:
/// 1. '\x' -> position count + 4 since \x is followed by two
///     hexadecimal numbers
/// 2. '\' -> position count + 2 to account for one escaped 
///     character and one
/// 3. in all othere cases position count increments by 1
/// 4. Once we traverse a character, we increment the character 
///     count by 1

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