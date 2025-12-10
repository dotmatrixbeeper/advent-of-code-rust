mod part1;
mod part2;

pub enum Directions {
    Up,
    Right,
    Down,
    Left
}

pub fn run(input: &str) {
    println!("======= DAY 3 =======");
    let data = decode(input);
    part1::solve(&data);
    part2::solve(&data);
    println!("=====================\n");
}

fn decode(data: &str) -> Vec<Directions> {
    data.chars()
        .map(|c| {
            match c {
                '^' => Directions::Up,
                '>' => Directions::Right,
                'v' => Directions::Down,
                _   => Directions::Left
            }
        })
        .collect()
}
