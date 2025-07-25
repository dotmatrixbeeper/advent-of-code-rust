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