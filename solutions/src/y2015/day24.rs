mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 24 =======");
    let weights = input.lines().map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    part1::solve(&weights);
    part2::solve(&weights);
    println!("======================\n");
}
