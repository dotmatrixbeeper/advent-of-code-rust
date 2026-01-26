use std::{collections::HashMap};
use std::io;

pub fn run(input: &str) {
    println!("======= DAY 10 =======");
    let mut readline = String::new();
    println!("Part 1 --------------");
    println!("Enter chips to watch out for:");
    println!("CHIP 1:");
    io::stdin().read_line(&mut readline).expect("Failed to read anything.");
    let chip_1 = readline.trim().parse::<u8>().unwrap();
    readline.clear();
    println!("CHIP 2:");
    io::stdin().read_line(&mut readline).expect("Failed to read anything.");
    let chip_2 = readline.trim().parse::<u8>().unwrap();
let mut output = HashMap::new();
    part_1(input, &mut output, chip_1, chip_2);
    part_2(&output);
    println!("=====================\n");
}

struct Robot {
    id: u8,
    low_node: Target,
    high_node: Target,
}

enum Target {
    Output(u8),
    Robot(u8)
}

impl Robot {
    fn new(id: u8, low_node: Target, high_node: Target) -> Self {
        Robot {
            id,
            low_node,
            high_node,
        }
    }
}

fn part_1(input: &str, output_boxes: &mut HashMap<u8, Vec<u8>>, chip_1: u8, chip_2: u8) {

    let (instructions, mut value_map, mut execution_stack) = get_translated_input(input);
    // while stack has values keep executing
    // if value in bot is equal to chips then break and print bot number

    while let Some(val) = execution_stack.pop() {
        let values = value_map.get_mut(&val).unwrap();
        let max = values[0].max(values[1]);
        let min = values[0].min(values[1]);

        if values.contains(&chip_1) && values.contains(&chip_2) {
            println!("Bot {} has both chips.", val);
        }

        match instructions[val as usize].high_node {
            Target::Output(destination) => {
                output_boxes.entry(destination).or_insert(vec![]).push(max);
            },
            Target::Robot(destination) => {
                value_map.entry(destination).or_insert(vec![]).push(max);
                if value_map.get(&destination).unwrap().len() == 2 {
                    execution_stack.push(destination);
                }
            }
        }

        match instructions[val as usize].low_node {
            Target::Output(destination) => {
                output_boxes.entry(destination).or_insert(vec![]).push(min);
            },
            Target::Robot(destination) => {
                value_map.entry(destination).or_insert(vec![]).push(min);
                if value_map.get(&destination).unwrap().len() == 2 {
                    execution_stack.push(destination);
                }
            }
        }
    } 
    println!("---------------------");
}

fn get_translated_input(input: &str) -> (Vec<Robot>, HashMap<u8, Vec<u8>>, Vec<u8>) {
    let mut robot_values = HashMap::new();
    let mut robots = Vec::new();
    let mut execution_stack = vec![];
    
    input
        .lines()
        .for_each(|line| {
            let ins_split = line.split(' ').collect::<Vec<&str>>();
            if ins_split[0] == "bot" {
                robots.push(Robot::new(ins_split[1].parse::<u8>().unwrap(),
                get_target(ins_split[6].parse::<u8>().unwrap(), ins_split[5]),
                get_target(ins_split[11].parse::<u8>().unwrap(), ins_split[10])),
                );
            } else {
                let value = ins_split[1].parse::<u8>().unwrap();
                let bot_id = ins_split[5].parse::<u8>().unwrap();
                let entry = robot_values.entry(bot_id).or_insert(vec![]);
                entry.push(value);

                if entry.len() == 2 {
                    execution_stack.push(bot_id);
                }
            }
        });

    robots.sort_by_key(|bot| bot.id);
    (robots, robot_values, execution_stack)
}

fn get_target(value: u8, target_type: &str) -> Target {
    if target_type == "bot" {
        Target::Robot(value)
    } else {
        Target::Output(value)
    }
}

fn part_2(output: &HashMap<u8, Vec<u8>>) {
    println!("Part 2 --------------");
    let product: u16 = *output.get(&0u8).unwrap().first().unwrap() as u16 * 
        *output.get(&1u8).unwrap().first().unwrap() as u16 *
        *output.get(&2u8).unwrap().first().unwrap() as u16;
    println!("Products of outputs: {}", product);
    println!("---------------------");
}