/// ### Solution for Part 2
/// This is the same as the previous problem, just with 50 passes
/// instead of 40.
/// 
/// #### Rust Implementation Details
/// Implementation remains thesame except the loop runs from 0 to 49

pub fn solve(input: &str) {
    let input = input.chars().collect::<Vec<char>>();
    let mut solution = look_and_say(&input);

    for _ in 0..49 {
        solution = look_and_say(&solution);
    }

    println!("Length after 50 iterations: {}", solution.len());

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