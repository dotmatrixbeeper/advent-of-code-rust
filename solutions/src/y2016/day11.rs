use std::collections::{HashSet, VecDeque};
use std::fmt::Display;
use std::{collections::BTreeSet, hash::Hash, hash::Hasher};
use std::cmp::Ordering;
use std::fmt::Formatter;
use itertools::Itertools;

use regex::Regex;

pub fn run(input: &str) {
    println!("======= DAY 11 =======");
    let re = Regex::new(r"(a (?<generator>[a-z]+) generator)|(a (?<microchip>[a-z]+)-compatible microchip)").unwrap();
    // collect tokens by iterating over each of the lines 
    // transform into Item structs and save them in a set
    let init_state = input
        .lines()
        .enumerate()
        .map(| (index, line) | {
            
            let items = re.captures_iter(line)
                .map(|capture| {
                    if let Some(mtch) = capture.name("generator") {
                        let item = Item::new(mtch.as_str(), ItemType::RTG);
                        return item;
                    } else  {
                        let mtch = capture.name("microchip").unwrap();
                        let item = Item::new(mtch.as_str(), ItemType::Chip);
                        return item;
                    }
                })
                .collect::<Vec<Item<'_>>>();
            (index, items)
        })
        .collect::<Vec<(usize, Vec<Item<'_>>)>>();
    
    let mut elevator = Elevator::new(0, 0);
    let mut chips = Vec::new();
    for (index, level) in &init_state {
        elevator.floors[*index] = level
            .iter()
            .collect::<BTreeSet<&Item<'_>>>();
        level
            .iter()
            .for_each(| item | chips.push(item));
    }
    part_1(elevator.clone(), chips.clone());
    let new_items = [
        Item {
            name: "elerium",
            item_type: ItemType::Chip
        },
        Item {
            name: "elerium",
            item_type: ItemType::RTG
        },
        Item {
            name: "dilithim",
            item_type: ItemType::Chip
        },
        Item {
            name: "dilithim",
            item_type: ItemType::RTG
        }
    ];

    for item in &new_items {
        elevator.floors[0].insert(&item);
        chips.push(&item);
    }
        
    part_2(elevator, chips);
    println!("======================\n");
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum ItemType {
    RTG,
    Chip
}

impl Display for ItemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Chip => write!(f, "Chip"),
            ItemType::RTG => write!(f, "RTG")
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Item<'a> {
    name: &'a str,
    item_type: ItemType,
}

impl<'a> Item<'a> {
    fn new(name: &'a str, item_type: ItemType) -> Self {
        Self {
            name,
            item_type,
        }
    }
}

impl<'a> PartialOrd for Item<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Item<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_string = self.item_type.to_string() + self.name;
        let other_string = other.item_type.to_string() + other.name;

        self_string.cmp(&other_string)
    }
}

#[derive(Eq, Clone)]
struct Elevator<'a> {
    elevator_position: u8,
    floors: [BTreeSet<&'a Item<'a>>; 4],
    parent_path: u16
}

impl<'a> Hash for Elevator<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator_position.hash(state);
        for i in 0..self.floors.len() {
            let (generator, chip) = self.floor_content_count(i);
            generator.hash(state);
            chip.hash(state);
        }
    }
}

impl<'a> PartialEq for Elevator<'a> {
    fn eq(&self, other: &Self) -> bool {
        let self_counts = self.floors.iter().enumerate().map(| (i, _) | self.floor_content_count(i)).collect::<Vec<(u8, u8)>>();
        let other_counts = other.floors.iter().enumerate().map(| (i, _) | other.floor_content_count(i)).collect::<Vec<(u8, u8)>>();
        
        self_counts == other_counts && self.elevator_position == other.elevator_position
    }
}

impl<'a> Elevator<'a> {
    fn new(elevator_position: u8, parent_path: u16) -> Self {
        Self {
            elevator_position,
            floors: [BTreeSet::new(), BTreeSet::new(), BTreeSet::new(), BTreeSet::new()],
            parent_path
        }
    }

    fn travel(&self, from: u8, to: i8, value: &'a Item, visited: &HashSet<Elevator<'_>>, top_down: bool) -> Result<Elevator<'a>, ()> {
        if !(0..4).contains(&from) || !(0..4).contains(&to) {
            return Err(())
        }

        if top_down && from as usize > to as usize && self.floors[to as usize].is_empty() {
            return Err(())
        }

        if Self::can_be_moved(&value, &self.floors[to as usize]) {
            let mut new_elevator = self.clone();
            new_elevator.elevator_position = to as u8;
            new_elevator.floors[to as usize].insert(value);
            new_elevator.floors[from as usize].remove(value);
            new_elevator.parent_path += 1;

            if visited.contains(&new_elevator) {
                return Err(());
            } else {
                return Ok(new_elevator);
            }
        } else {
            return Err(());
        }
    }

    fn travel_in_pair(&self, from: u8, to: i8, first: &'a Item, second: &'a Item, visited: &HashSet<Elevator<'_>>, top_down: bool) -> Result<Elevator<'a>, ()> {
        if !(0..4).contains(&from) || !(0..4).contains(&to) {
            return Err(())
        }

        if top_down && from as usize > to as usize && self.floors[to as usize].is_empty() {
            return Err(())
        }

        if Self::can_be_moved_in_pairs(&first, &second, &self.floors[to as usize]) {
            let mut new_elevator = self.clone();
            new_elevator.elevator_position = to as u8;  
            new_elevator.floors[to as usize].insert(first);
            new_elevator.floors[from as usize].remove(first);
            new_elevator.floors[to as usize].insert(second);
            new_elevator.floors[from as usize].remove(second);
            new_elevator.parent_path += 1;

            if visited.contains(&new_elevator) {
                return Err(());
            } else {
                return Ok(new_elevator)
            }
        } else {
            return Err(());
        }
    }

    fn can_be_moved(value: &Item, floor: &BTreeSet<&Item>) -> bool {
        let generator_count = floor.iter().filter(| item | item.item_type == ItemType::RTG).count();
        let microchip_count = floor.iter().filter(| item | item.item_type == ItemType::Chip).count();
        if value.item_type == ItemType::RTG {
            // can be moved if the generator count will be greater than microchip count
            return generator_count + 1 > microchip_count;
        } else {
            return (generator_count == 0  && microchip_count + 1 > 0) || (generator_count > microchip_count + 1);
        }
    }

    fn can_be_moved_in_pairs(first: &Item, second: &Item, floor: &BTreeSet<&Item>) -> bool {
        let mut generator_count = floor.iter().filter(| item | item.item_type == ItemType::RTG).count();
        let mut microchip_count = floor.iter().filter(| item | item.item_type == ItemType::Chip).count();
        
        if first.item_type == second.item_type {
            if first.item_type == ItemType::Chip {
                microchip_count += 2;
                return (generator_count == 0  && microchip_count > 0) || (generator_count > microchip_count);
            } else {
                generator_count += 2;
                return generator_count > microchip_count;
            }
        } else {
            generator_count += 1;
            microchip_count += 1;
            return (generator_count == 0  && microchip_count > 0) || (generator_count > microchip_count); 
        }
    }

    fn floor_content_count(&self, floor_number: usize) -> (u8, u8) {
        
        (self.floors[floor_number].iter().filter(| item | item.item_type == ItemType::RTG).count() as u8, 
            self.floors[floor_number].iter().filter(| item | item.item_type == ItemType::Chip).count() as u8)
    }
}

fn part_1(elevator: Elevator, chips: Vec<&Item>) {
    println!("Part 1 --------------");
    println!("Minimum steps to travel: {}", solver(elevator, chips));
    println!("---------------------");
}

fn solver(elevator: Elevator, chips: Vec<&Item>) -> u16 {

    let mut target_elevator = Elevator::new(3, 0);
    target_elevator.floors[3] = chips.iter().map(| item | *item).collect::<BTreeSet<&Item<'_>>>();

    let mut front_visited: HashSet<Elevator<'_>> = HashSet::new();
    let mut back_visited: HashSet<Elevator<'_>> = HashSet::new();

    let mut top_down_queue = VecDeque::new();
    let mut bottom_up_queue = VecDeque::new();

    top_down_queue.push_back(elevator);
    bottom_up_queue.push_back(target_elevator);

    let mut steps = 0;

    while !top_down_queue.is_empty() && !bottom_up_queue.is_empty() {
        let front = top_down_queue.pop_front().unwrap();
        let back = bottom_up_queue.pop_front().unwrap();
        if back_visited.contains(&front) {
            steps = back_visited.get(&front).unwrap().parent_path + front.parent_path;
            break;
        }

        if front_visited.contains(&back) {
            steps = front_visited.get(&back).unwrap().parent_path + back.parent_path;
            break;
        }

        let mut pair_move_up_front = false;
        let mut single_move_down_front = false;

        // front move 
        for item in front.floors[front.elevator_position as usize].iter().combinations(2) {
            if let Ok(new_state) = front.travel_in_pair(front.elevator_position, (front.elevator_position + 1) as i8, item[0], item[1], &front_visited, true) {
                pair_move_up_front = true;
                front_visited.insert(new_state.clone());
                top_down_queue.push_back(new_state);
            }
        }

        if !pair_move_up_front {

            for item in &front.floors[front.elevator_position as usize] {
                if let Ok(new_state) = front.travel(front.elevator_position, (front.elevator_position + 1) as i8, item, &front_visited, true) {
                    front_visited.insert(new_state.clone());
                    top_down_queue.push_back(new_state);
                }
                
            }
        }
        
        for item in &front.floors[front.elevator_position as usize] {
            if let Ok(new_state) = front.travel(front.elevator_position, front.elevator_position as i8 - 1, item, &front_visited, true) {
                single_move_down_front = true;
                front_visited.insert(new_state.clone());
                top_down_queue.push_back(new_state);
            }
        }

        if !single_move_down_front {
            for item in front.floors[front.elevator_position as usize].iter().combinations(2) {
                if let Ok(new_state) = front.travel_in_pair(front.elevator_position, front.elevator_position as i8 - 1, item[0], item[1], &front_visited, true) {
                    front_visited.insert(new_state.clone());
                    top_down_queue.push_back(new_state);
                }
            }
        }
            
        for item in back.floors[back.elevator_position as usize].iter().combinations(2) {
            if let Ok(new_state) = back.travel_in_pair(back.elevator_position, (back.elevator_position + 1) as i8, item[0], item[1], &back_visited, false) {
                back_visited.insert(new_state.clone());
                bottom_up_queue.push_back(new_state);
            }

            if let Ok(new_state) = back.travel_in_pair(back.elevator_position, back.elevator_position as i8 - 1, item[0], item[1], &back_visited, false) {
                back_visited.insert(new_state.clone());
                bottom_up_queue.push_back(new_state);
            }
        }


        for item in &back.floors[back.elevator_position as usize] {
            if let Ok(new_state) = back.travel(back.elevator_position, (back.elevator_position + 1) as i8, item, &back_visited, false) {
                back_visited.insert(new_state.clone());
                bottom_up_queue.push_back(new_state);
            }
            
            if let Ok(new_state) = back.travel(back.elevator_position, back.elevator_position as i8 - 1, item, &back_visited, false) {
                back_visited.insert(new_state.clone());
                bottom_up_queue.push_back(new_state);
            }
        }
    }

    return steps;
}

fn part_2(elevator: Elevator, chips: Vec<&Item>) {
    println!("Part 2 --------------");
    println!("Minimum steps to travel: {}", solver(elevator, chips));
    println!("---------------------");
}