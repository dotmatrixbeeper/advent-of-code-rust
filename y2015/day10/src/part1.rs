/// ### Solution for Part 1
/// Look and say requires us to convert each number or consecutive 
/// sequence of numbers in a string into its spoke form, and then 
/// convert them back into a number.
/// 
/// #### Rust Implementation Details
/// We start at the second letter and keep looking back to see if
/// we find the end of a sequence. 
/// Once we find the end of a sequence we put the count of the 
/// sequence with the sequence number. 
/// Then we just need to length of the resulting sequence.

pub fn solve() {
    let input = "3113322113".chars().collect::<Vec<char>>();
    let mut solution = look_and_say(&input);

    for _ in 0..39 {
        solution = look_and_say(&solution);
    }

    println!("Length after 40 iterations: {}", solution.len());

}

fn look_and_say(input: &Vec<char>) -> Vec<char> {
    let mut prev = input[0];
    let mut count = 1;
    let mut solution_vec = vec![];

    for i in 1..input.len() {
        let current = input[i];
        if current == prev {
            count += 1;
        } else {
            solution_vec.push(char::from_digit(count, 10).unwrap());
            solution_vec.push(prev);
            count = 1;
        }
        prev = current;
    }

    solution_vec.push(char::from_digit(count, 10).unwrap());
    solution_vec.push(*input.last().unwrap());

    solution_vec
}