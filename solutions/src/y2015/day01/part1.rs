/// ### Solution for Part 1
/// 
/// We imagine ground level to be level 0. This is the current level
/// We look at each symbol in the input and repeat the following
/// + If symbol is `(`
///     - add one to the current level
/// + If symbol is `)`
///     - subtract one from the current level
/// 
/// After going through all the symbols we will have the resulting level we are at.
/// 
/// #### Rust Implementation Details
/// We create an iterator over the characters of the input string. 
/// We map each character and match against `(` to return 1 and -1 otherwise. 
/// Then we use the collecting function `sum` to add all the map returned 1s and -1s to arrive at the answer.

pub fn solve(data: &str) {
    let level: i32 = data.chars()
        .map(|c| 
            match c {
                '(' => return 1,
                _   => return -1
            })
        .sum();

    println!("Santa is on level: {level}");
}