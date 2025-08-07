mod part1;
mod part2;

pub enum Directions {
    Up,
    Right,
    Down,
    Left
}

fn main() {
    let data = decode(include_str!("../resources/input.txt"));
    part1::solve(&data);
    part2::solve(&data);
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
