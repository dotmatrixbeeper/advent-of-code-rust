mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 7 =======");
    let a = part1::solve(input);
    part2::solve(input, a);
    println!("=====================\n");
}