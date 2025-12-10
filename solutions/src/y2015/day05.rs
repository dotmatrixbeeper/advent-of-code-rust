mod part1;
mod part2;

pub fn run(input: &str) {
    println!("======= DAY 5 =======");
    let data = convert(input);
    part1::solve(&data);
    part2::solve(&data);
    println!("=====================\n");
}

fn convert(input: &str) -> Vec<&str> {
    input.split('\n').collect()
}