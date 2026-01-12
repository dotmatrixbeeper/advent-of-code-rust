use memchr::memchr;

pub fn run(input: &str) {
    println!("======= DAY 9 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    let mut position = 0;
    let input = input.as_bytes();
    let mut result: usize = 0;
    while position < input.len() {
        if input[position] == '(' as u8 {
            position += 1;
            decompress(&mut result, &mut position, input);
        } else {
            let next_marker = memchr(b'(', &input[position..])
                .map(|p| p + position)
                .unwrap_or(input.len());
            result += next_marker - position;
            position = next_marker;
        }
    }
    println!("Decompressed file length: {}", result);
    println!("---------------------");
}

/// Calculate decompressed length
fn decompress(result: &mut usize, position: &mut usize, input: &[u8]) {
    let mut marker_end = 0;
    while input[*position + marker_end] != ')' as u8 && (*position + marker_end) < input.len() {
        marker_end += 1; 
    }
    let (data_len, repeat) = parse_marker(input, position);

    *position += 1;
    if *position < input.len() {
        if *position + data_len < input.len() {
            *result += data_len * repeat;
        } else {
            *result += (input.len() - *position) * repeat;
        }
        *position += data_len;
    }
}

fn parse_marker(input: &[u8], position: &mut usize) -> (usize, usize) {
    let mut num_1 =  0usize;

    while input[*position] != b'x' {
        num_1 = num_1 * 10 + (input[*position] - b'0') as usize;
        *position += 1;
    }
    *position += 1;

    let mut num_2 = 0usize;
    while input[*position] != b')' {
        num_2 = num_2 * 10 + (input[*position] - b'0') as usize;
        *position += 1;
    }

    (num_1, num_2)
}

fn part_2(input: &str) {
    println!("Part 2 --------------");
    let result = decompress_recursive(0, input.len(), input.as_bytes());
    println!("Decompressed file length: {}", result);
    println!("---------------------");
}

/// This handles decompression recursively
fn decompress_recursive(start: usize, end: usize, input: &[u8]) -> usize {
    let mut position = start;
    let mut result: usize = 0;
    while position < end {
        // marker start
        if input[position] == '(' as u8 { 
            position += 1; 
            // marker looks line MxN
            // splitting by `x` to extract data length and repeatation count
            let (data_len, repeat) = parse_marker(input, &mut position);

            // increment position to look at data
            position += 1;

            // get the decompressed data by sending the data string to this function again
            // we send only the data segment of the input, not the whole string
            let decompressed_data_len = decompress_recursive(position, position + data_len, input);
            
            // update position to just after the data length to go to next segment
            position += data_len;
            result += decompressed_data_len * repeat;
        } else {
            // for data that doesn't have marker
            let non_decompressed_len = memchr(b'(', &input[position..]).unwrap_or(input[position..].len());

            // update position to just after the non-decompressed segment
            position += non_decompressed_len;
            result += non_decompressed_len;
        }
    }
    return result;
}