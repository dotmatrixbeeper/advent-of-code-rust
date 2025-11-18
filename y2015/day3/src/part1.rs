/// ### Solution for Part 1
/// The problem requires you to go in a path dictacted by the input of `^V<>`
/// Since we can go over a house more than once, we need to only count unique houses.
/// Thus we need to remember the houses we have gone over.
/// We can do this by using a set, such that we only have unique entries.
/// 
/// #### Rust Implementation Details
/// We use `HashSet` from the Standard Library as our set implementation
/// We iterate over the directions starting from a (0, 0) grid position
/// We add decide the next house to go to based on the input symbol

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