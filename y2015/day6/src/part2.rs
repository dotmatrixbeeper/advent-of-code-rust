use crate::Instruction;

pub fn solve(data: &Vec<Instruction>) {
    let mut light_grid: Vec<Vec<i16>> = vec![vec![0; 1000]; 1000];
    let mut light_count: i32 = 0;
    for command in data {
        for i in command.start_x..=command.end_x {
            for j in command.start_y..=command.end_y {
                match command.verb {
                    crate::Verb::TurnOn => {
                        light_count += 1;
                        light_grid[i][j] += 1;
                    },
                    crate::Verb::TurnOff => {
                        if light_grid[i][j] > 0 {
                            light_count -= 1;
                            light_grid[i][j] -= 1;
                        }
                    },
                    crate::Verb::Toggle => {
                        light_count += 2;
                        light_grid[i][j] += 2;
                    },
                }
            }
        }
    }

    println!("The number of lights that are on: {}", light_count);
}