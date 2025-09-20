mod part1;
mod part2;

/// ### Day 1: Not Quite Lisp
fn main() {
    let data = convert(include_str!("../resources/input.txt"));
    part1::solve(&data);
    part2::solve(&data);
}

fn convert(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}