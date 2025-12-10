/// ### Solution for Part 2
/// This changes the first part of the problem by saying that 
/// the lights in four corners stay on. This is stragiht-forward
/// to handle by putting in a condition to not change anything in 
/// the corners.
/// 
/// #### Rust Implementation Details
/// The solution remains same as the first part, except we add
/// a small condition to check if we are currently looking at 
/// the corners.

const LOOKUP_OFFSETS: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1),
                                        (0, -1), (0, 1),
                                        (1, -1), (1, 0), (1, 1)];

pub fn solve(mut data: Vec<Vec<char>>) {
    let mut lights_on = 0;
    data[0][0] = '#';
    data[0][99] = '#';
    data[99][0] = '#';
    data[99][99] = '#';

    let mut next_step = data.clone();
    for i in 0..100 {
        for j in 0..100 {
            for k in 0..100 {
                if (j, k) == (0, 0) || (j, k) == (0, 99) || (j, k) == (99, 0) || (j, k) == (99, 99) {
                    continue;
                } 
                // check from top-left corner to bottom-right corner and decide the change in the current j, k
                let surround_light_count = LOOKUP_OFFSETS.iter()
                    .map(|(delta_j, delta_k)| {
                        let res_j = j + delta_j;
                        let res_k = k + delta_k;

                        if res_j < 0 || res_j > 99 || res_k < 0 || res_k > 99 {
                            return 0;
                        } else if data[res_j as usize][res_k as usize] == '#' {
                            return 1;
                        } else {
                            return 0;
                        }
                    })
                    .sum::<i32>();
                
                if data[j as usize][k as usize] == '#' && surround_light_count != 2 && surround_light_count != 3 {
                    next_step[j as usize][k as usize] = '.'; 
                } else if data[j as usize][k as usize] == '.' &&  surround_light_count == 3 {
                    next_step[j as usize][k as usize] = '#'; 
                }

                if i == 99 && next_step[j as usize][k as usize] == '#' {
                    lights_on += 1;
                }
            }
        }
        data = next_step.clone();
    }

    println!("Number of on lights: {}", lights_on + 4);
}
