mod part1;
mod part2;

fn main() {
    let data = encode(include_str!("../resources/input.txt"));
    part1::solve(data.clone());
    part2::solve(data.clone());
}

fn encode(input: &str) -> Vec<Vec<char>> {

    input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>()
}
