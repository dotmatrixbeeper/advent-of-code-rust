/// ### Solution for Part 2
/// The second part of the problem has Santa and Robo-Santa taking orders
/// alternatively. Thus we have to have two workers that independently take turns moving.
/// So we need to track the houses being visited by both.
/// 
/// #### Rust Implementation Details
/// The difference to the first part, is that we have two trackers now
/// 'current_santa_house` and `current_robo_santa_house`. 
/// We maintain a boolean `turn` flag:
/// - if it is `true` update current_santa_house
/// - if it is `false` update current_robo_santa_house
/// Each turn we insert the house visited into the hashset
/// At the end we count the number of elements in the set.

use std::collections::HashSet;

use crate::Directions;

pub fn solve(directions: &Vec<Directions>) {
    let mut house_set = HashSet::new();
    house_set.insert((0, 0));

    let mut current_santa_house = (0, 0);
    let mut current_robo_santa_house = (0, 0);
    let mut turn = true;

    directions
        .iter()
        .for_each(| direction | {
            let current_house;
            if turn {
                current_house = &mut current_santa_house;
                turn = false;
            } else {
                current_house = &mut current_robo_santa_house;
                turn = true;
            }

            match direction {
                Directions::Up => { (*current_house).1 += 1; },
                Directions::Down => { (*current_house).1 -= 1; },
                Directions::Right => { (*current_house).0 += 1; },
                _ => { (*current_house).0 -= 1; }
            };
            house_set.insert(*current_house);
        });
    println!("Santa and Robo-Santa visit {} houses as instructed by the drunk elf.", house_set.len());
}