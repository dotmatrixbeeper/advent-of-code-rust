mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 1 =======");
    part1::solve(input);
    part2::solve(input);
    println!("=====================\n");
}