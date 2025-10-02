use regex::Regex;

mod part1;
mod part2;

#[derive(Debug)]
pub struct Reindeer {
    name: String,
    speed: u32,
    flying_duration: u32,
    rest_duration: u32,
}

impl Reindeer {
    fn new(name: String, speed: u32, flying_duration: u32, rest_duration: u32) -> Self {
        Self { name, speed, flying_duration, rest_duration }
    }

    pub fn total_period(&self) -> u32 {
        self.flying_duration + self.rest_duration
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn flying_duration(&self) -> u32 {
        self.flying_duration
    }

    pub fn rest_duration(&self) -> u32 {
        self.rest_duration
    }
}

fn main() {
    let data = encode(include_str!("../resources/input.txt"));
    part1::solve(&data);
    part2::solve(&data);
}

fn encode(input: &str) -> Vec<Reindeer> {
    let mut data = vec![];

    let re = Regex::new(r"^(?<name>\w+) can fly (?<speed>\d+) km/s for (?<flying_duration>\d+) seconds, but then must rest for (?<rest_duration>\d+) seconds\.").unwrap();
    
    for line in input.lines() {
        let captures = re.captures(line).unwrap();

        data.push(
            Reindeer::new(String::from(&captures["name"]),
                captures["speed"].parse::<u32>().unwrap(),
                captures["flying_duration"].parse::<u32>().unwrap(),
                captures["rest_duration"].parse::<u32>().unwrap())
        );
    }
    data
}