/// ### Solution for Part 2
/// 
/// We imagine ground level to be level 0. This is the current level
/// We look at each symbol in the input and repeat the following
/// + If symbol is `(`
///     - add one to the current level
/// + If symbol is `)`
///     - subtract one from the current level
/// 
/// The implementation is similar to part 1 except we don't need to go all the way to evaluate every symbol in input.
/// Once we find the symbol that takes us to the basment (-1) we can exit.
/// 
/// #### Rust Implementation Details
/// We create an iterator over the characters of the input string. This time we also `ennumerate()` which 
/// returns a iterator over a tuple of (index, character). This helps us keep track of which position we 
/// arrive at the solution.
/// 
/// We start with a sum of 0 indicating the current floor santa is in. Our goal is to find at which input position 
/// we reach -1 (basment)
/// We map each character and match against `(` to add 1 sum and add -1 otherwise. 
/// 
/// At the point where sum becomes -1 is where we reach basement. We print the position and finish.

pub fn solve(data: &str) {
    let mut sum: i32 = 0;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => { sum += 1 },
            _   => { sum -= 1 }
        }
        if sum == -1 {
            println!("Santa gets to the basement at position: {}", (i + 1));
            break;
        }
    }
}