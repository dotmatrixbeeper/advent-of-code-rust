pub fn solve(data: &str) {
    let mut sum: i32 = 0;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => { sum += 1 },
            _   => { sum -= 1 }
        }
        if sum == -1 {
            println!("Santa gets to the basement at position: {}", (i + 1));
            break;
        }
    }
}