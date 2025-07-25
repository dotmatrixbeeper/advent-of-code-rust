pub fn solve(data: &str) {
    let level: i32 = data.chars()
        .map(|c| 
            match c {
                '(' => return 1,
                _   => return -1
            })
        .sum();

    println!("Santa is on level: {level}");
}