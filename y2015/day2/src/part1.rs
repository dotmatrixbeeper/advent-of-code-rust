/// ### Solution for Part 1
/// 
/// 
/// #### Rust Implementation Details
/// We create an iterator over the characters of the input string. 
/// We map each character and match against `(` to return 1 and -1 otherwise. 
/// Then we use the collecting function `sum` to add all the map returned 1s and -1s to arrive at the answer.

use std::cmp::min;

use crate::Dimensions;

pub fn solve(data: Vec<Dimensions>) {
    let size: u32 = data.iter()
        .map(| side: &Dimensions | {
            let areas: (u32, u32, u32) =  ((side.0 as u32 * side.1 as u32), (side.1 as u32 * side.2 as u32), (side.2 as u32 * side.0 as u32));
            return 2 * (areas.0 + areas.1 + areas.2) + min(areas.2, min(areas.0, areas.1));
        })
        .sum();
    println!("Total size of wrapping papers requried: {size}")
}