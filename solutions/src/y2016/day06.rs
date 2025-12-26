pub fn run(input: &str) {
    println!("======= DAY 6 =======");
    part_1(input);
    part_2(input);
    println!("=====================\n");
}

fn part_1(input: &str) {
    println!("Part 1 --------------");
    // we use a shape like 
    // [[0, 0, 0 .... 0],[0, 0, 0 .... 0],[0, 0, 0 .... 0],[0, 0, 0 .... 0],[0, 0, 0 .... 0],[0, 0, 0 .... 0],[0, 0, 0 .... 0][0, 0, 0 .... 0]]
    // one item in the array for each position, so 8 items
    // in each item which are arrays themselves we have 26 items, one for each letter
    let mut count_table: [[u16; 26]; 8] = [[0; 26]; 8];

    // Go through each line
    input
        .lines()
        .for_each(|line| {
            line
                .chars()
                .enumerate()
                // find character position in alphabet and update corresponding count
                .map(|(i, c)| (i, c as u8 - b'a' as u8))
                .for_each(|(i, n)| count_table[i][n as usize] += 1); 
        });
    let message = count_table
        .map(|count| 
            // find the poistion with highest value in each inner array
            // for that position find the letter converting into char. Collect as string
            (b'a' as u8 + count.iter().enumerate().max_by_key(|(_, value)| *value).unwrap().0 as u8) 
            as char)
        .iter()
        .collect::<String>();
    println!("The Actual Message: {}", message);
    println!("---------------------");
}
fn part_2(input: &str) {
    println!("Part 2 --------------");
    // same as part 1 except we find min freq character in each column
    let mut count_table: [[u16; 26]; 8] = [[0; 26]; 8];

    input
        .lines()
        .for_each(|line| {
            line
                .chars()
                .enumerate()
                .map(|(i, c)| (i, c as u8 - b'a' as u8))
                .for_each(|(i, n)| count_table[i][n as usize] += 1); 
        });
    let message = count_table
        .map(|count| 
            (b'a' as u8 + count.iter().enumerate().min_by_key(|(_, value)| *value).unwrap().0 as u8) 
            as char)
        .iter()
        .collect::<String>();
    println!("The Actual Message: {}", message);
    println!("---------------------");
}