mod part1;
mod part2;

fn main() {
    let containers: Vec<u32> = vec![33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42];
    part1::solve(&containers, 150);
    part2::solve(&containers, 150);
}
