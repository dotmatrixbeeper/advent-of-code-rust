mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 8 =======");
    let data = input.lines().collect::<Vec<&str>>();
    part1::solve(&data);
    part2::solve(&data);
    println!("=====================\n");
}