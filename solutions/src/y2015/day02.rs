mod part1;
mod part2;

use regex::Regex;
pub struct Dimension(u32, u32, u32);

pub fn run(input: &str) {
    println!("======= DAY 2 =======");
    let data = decode(input);
    part1::solve(&data);
    part2::solve(&data);
    println!("=====================\n");
}

fn decode(data: &str) -> Vec<Dimension> {
    let sides_re = Regex::new(r"^(?<l>\d+)x(?<w>\d+)x(?<h>\d+)$").unwrap();
    data.split('\n')
        .map(| s| {
            let Some(caps) = sides_re.captures(s) else { return Dimension(0, 0, 0) };
            return Dimension(
                caps["l"].parse::<u32>().unwrap(), 
                caps["w"].parse::<u32>().unwrap(), 
                caps["h"].parse::<u32>().unwrap()
            )
        }).collect()
}
