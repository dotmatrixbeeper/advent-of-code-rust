use regex::Regex;

mod part1;
mod part2;

pub struct  Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn run(input: &str) {
    println!("======= DAY 15 =======");
    let data = encode(input);
    part1::solve(&data);
    part2::solve(&data);
    println!("======================\n");
}

fn encode(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(r"^(?<name>\w+): capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)$").unwrap();
    let mut data = vec![];
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        data.push( Ingredient {
            capacity: caps["capacity"].parse::<i32>().unwrap(),
            durability: caps["durability"].parse::<i32>().unwrap(),
            flavor: caps["flavor"].parse::<i32>().unwrap(),
            texture: caps["texture"].parse::<i32>().unwrap(),
            calories: caps["calories"].parse::<i32>().unwrap()
        });
    }
    
    data
}
