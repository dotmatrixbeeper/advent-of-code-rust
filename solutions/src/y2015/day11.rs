mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 11 =======");
    let data = input.lines().next().unwrap();
    let result = part1::solve(data);
    part2::solve(&result);
    println!("======================\n");
}
