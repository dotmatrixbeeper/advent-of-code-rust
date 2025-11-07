pub fn solve() {
    let mut element: u64 = 27995004;
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