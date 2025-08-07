use std::collections::HashSet;

use crate::Directions;

pub fn solve(directions: &Vec<Directions>) {
    let mut house_set = HashSet::new();
    house_set.insert((0, 0));

    let mut current_house = (0, 0);

    directions
        .iter()
        .for_each(| direction | {
            match direction {
                Directions::Up => { current_house.1 += 1; },
                Directions::Down => { current_house.1 -= 1; },
                Directions::Right => { current_house.0 += 1; },
                _ => { current_house.0 -= 1; }
            };
            house_set.insert(current_house);
        });
    println!("Santa visits {} houses as instructed by the drunk elf.", house_set.len());
}