/// ### Solution for Part 1
/// In a 1000x1000 grid we need to keep track of the lights that 
/// are on and the lights that are off. This should be easy enough
/// with a vector of vectors.
/// 
/// #### Rust Implementation Details
/// Construct a grid of Vec<Vec<bool>> where each grid point is a 
/// boolean.
/// true = on
/// false = off
/// 
/// We iterate over each instruction, go through the range and switch
/// on or off each light based on what the current instruction is.
/// This does make the time complexity as O(n^3). I might sit down
/// and figure out a cleverer way, but for now, this will suffice.

use crate::Instruction;

pub fn solve(data: &Vec<Instruction>) {
    let mut light_grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    let mut light_count: i32 = 0;
    for command in data {
        for i in command.start_x..=command.end_x {
            for j in command.start_y..=command.end_y {
                match command.verb {
                    crate::Verb::TurnOn => {
                        if light_grid[i][j] == false {
                            light_count += 1;
                        }
                        light_grid[i][j] = true;
                    },
                    crate::Verb::TurnOff => {
                        if light_grid[i][j] == true {
                            light_count -= 1;
                        }
                        light_grid[i][j] = false;
                    },
                    crate::Verb::Toggle => {
                        if light_grid[i][j] == true {
                            light_count -= 1;
                        } else {
                            light_count += 1;
                        }
                        light_grid[i][j] = !light_grid[i][j];
                    },
                }
            }
        }
    }

    println!("The number of lights that are on: {}", light_count);
}