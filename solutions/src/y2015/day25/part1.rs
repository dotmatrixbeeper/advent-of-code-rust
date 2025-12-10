/// ### Solution for Part 1
/// This is a simple problem of finding the code at 
/// a given position with the given formula.
/// 
/// #### Rust Implementation Details
/// We find the serial position for the asked coordinates.
/// After that we just need to iterate from the last known 
/// position to the target position while applying the formula.

pub fn solve(input: &str) {
    let mut element: u64 = input.parse().unwrap();
    let start_position = find_position(6, 6) + 1;
    let target_position = find_position(2978, 3083);

    (start_position..=target_position)
        .for_each(|_| {
            element = (element * 252533) % 33554393;
        });

    println!("The code is: {element}");
}

fn find_position(x: u64, y: u64) -> u64 {
    let row = x + (y - 1);
    (1..row).sum::<u64>() + y
}