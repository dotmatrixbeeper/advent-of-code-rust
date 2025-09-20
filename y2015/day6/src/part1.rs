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