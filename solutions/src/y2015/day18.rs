mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 18 =======");
    let data = encode(input);
    part1::solve(data.clone());
    part2::solve(data.clone());
    println!("======================\n");
}

pub fn encode(input: &str) -> Vec<Vec<char>> {

    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}
