/// ### Solution for Part 1
/// We have two reindeers racing and need to find who is ahead at the 
/// 2503 second.
/// 
/// #### Rust Implementation Details
/// We can mathematically calculate the distances travelled by each 
/// reindeer using given speed and rest durations. Then we compare 
/// the results and declare the winner.

use crate::Reindeer;

pub fn solve(reindeers: &Vec<Reindeer>) {
    let evaluation_duration = 2503;
    let mut winning: (&str, u32) = ("", 0);

    for reindeer in reindeers {
        let total_period = reindeer.total_period();
        let mut distance_travelled = (evaluation_duration / total_period) * reindeer.flying_duration() * reindeer.speed();
        let remaining_period = evaluation_duration % total_period ;

        if remaining_period >= reindeer.flying_duration() {
            distance_travelled += reindeer.flying_duration() * reindeer.speed();
        } else {
            distance_travelled += remaining_period * reindeer.speed();
        }

        if distance_travelled > winning.1 {
            winning.0 = reindeer.name();
            winning.1 = distance_travelled;
        }
    }

    println!("Winning reinder {} travelled {} kms", winning.0, winning.1);
}