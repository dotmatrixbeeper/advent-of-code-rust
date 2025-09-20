pub mod part1;
pub mod part2;

use regex::Regex;

pub enum Verb {
    TurnOn,
    TurnOff,
    Toggle
}

pub struct Instruction {
    verb: Verb,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize
}

fn main() {
    let data = convert(include_str!("../resources/input.txt"));
    part1::solve(&data);
    part2::solve(&data);
}

fn convert(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"^(?<verb>turn on|turn off|toggle) (?<start_x>\d+),(?<start_y>\d+) through (?<end_x>\d+),(?<end_y>\d+)$").unwrap();
    input.split('\n')
        .map(| line | {
            let captures = re.captures(line).unwrap();
            return Instruction {
                verb: match &captures["verb"] {
                    "turn on" => Verb::TurnOn,
                    "turn off" => Verb::TurnOff,
                    _ => Verb::Toggle
                },
                start_x: captures["start_x"].parse::<usize>().unwrap(),
                start_y: captures["start_y"].parse::<usize>().unwrap(),
                end_x: captures["end_x"].parse::<usize>().unwrap(),
                end_y: captures["end_y"].parse::<usize>().unwrap(),
            }
        })
        .collect()
}
