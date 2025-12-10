/// ### Solution for Part 2
/// The second part re-writes the rules a bit and turns each grid 
/// point into a continuous value instead of boolean on and off.
/// This follows the previous implementation with a few changes.
/// 
///  
/// #### Rust Implementation Details
/// We change the previous implementation to have the vector 
/// contain i16 instead of boolean.
/// On each iteration now we increment or decrement the value
/// in the grid point as required

use crate::y2015::day06::Instruction;
use crate::y2015::day06::Verb;

pub fn solve(data: &Vec<Instruction>) {
    let mut light_grid: Vec<Vec<i16>> = vec![vec![0; 1000]; 1000];
    let mut light_count: i32 = 0;
    for command in data {
        for i in command.start_x..=command.end_x {
            for j in command.start_y..=command.end_y {
                match command.verb {
                    Verb::TurnOn => {
                        light_count += 1;
                        light_grid[i][j] += 1;
                    },
                    Verb::TurnOff => {
                        if light_grid[i][j] > 0 {
                            light_count -= 1;
                            light_grid[i][j] -= 1;
                        }
                    },
                    Verb::Toggle => {
                        light_count += 2;
                        light_grid[i][j] += 2;
                    },
                }
            }
        }
    }

    println!("The number of lights that are on: {}", light_count);
}