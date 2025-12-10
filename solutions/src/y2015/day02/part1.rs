/// ### Solution for Part 1 
/// Wrapping paper area equals surface area of the box being wrapped.
/// Find surface area of all the boxes and add them up along with the 
/// area of the smallest side.
/// 
/// #### Rust Implementation Details
/// iterate over the `Dimension` vector, where each `Dimension` holds 
/// the length, breadth, and height.
/// Calculate area of the sides.
/// Find the min area among them.
/// Accumulate with `.sum()`

use std::cmp::min;

use crate::y2015::day02::Dimension;

pub fn solve(data: &Vec<Dimension>) {
    let size: u32 = data.iter()
        .map(| side: &Dimension | {
            let areas: (u32, u32, u32) =  ((side.0 as u32 * side.1 as u32), (side.1 as u32 * side.2 as u32), (side.2 as u32 * side.0 as u32));
            return 2 * (areas.0 + areas.1 + areas.2) + min(areas.2, min(areas.0, areas.1));
        })
        .sum();
    println!("Total size of wrapping papers requried: {size}")
}