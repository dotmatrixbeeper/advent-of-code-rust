mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 17 =======");
    let containers: Vec<u32> = input.lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    part1::solve(&containers, 150);
    part2::solve(&containers, 150);
    println!("======================\n");
}
