mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 10 =======");
    let data = input.lines().next().unwrap();
    part1::solve(data);
    part2::solve(data);
    println!("======================\n");
}
