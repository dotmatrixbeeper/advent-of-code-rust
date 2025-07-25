mod part1;
mod part2;

fn main() {
    let data = include_str!("../resources/input.txt");
    part1::solve(data);
    part2::solve(data);
}