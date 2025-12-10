mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 20 =======");
    let data = input.parse::<u32>().unwrap();
    part1::solve(data);
    part2::solve(data);
    println!("======================\n");
}
