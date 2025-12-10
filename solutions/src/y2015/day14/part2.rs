/// ### Solution for Part 2
/// In part 2, we need to award points for each second that a reindeer
/// is in lead. And thus we need to track the duration period per second.
/// 
/// #### Rust Implementation Details
/// We iterate from 1 to 2503 and after each tick we increment the point 
/// to the leading reindeer.

use std::collections::{HashMap, HashSet};

use crate::y2015::day14::Reindeer;

pub fn solve(reindeers: &Vec<Reindeer>) {
    let evaluation_duration = 2503;
    let mut tracker_map: HashMap<&str, u32> = HashMap::new();
    let mut winning: (HashSet<&str>, u32) = (HashSet::new(), 0);

    for tick in 1..=evaluation_duration {
        for reindeer in reindeers {

            let total_period = reindeer.total_period();
            let mut distance_travelled = (tick / total_period) * reindeer.flying_duration() * reindeer.speed();
            let remaining_period = tick % total_period ;

            if remaining_period >= reindeer.flying_duration() {
                distance_travelled += reindeer.flying_duration() * reindeer.speed();
            } else {
                distance_travelled += remaining_period * reindeer.speed();
            }

            if distance_travelled > winning.1 {
                winning.0 = HashSet::new();
                winning.0.insert(reindeer.name());
                winning.1 = distance_travelled;
            } else if distance_travelled == winning.1 {
                winning.0.insert(reindeer.name());
            }
        }

        winning.0.iter()
            .for_each(|name| {
                let points = tracker_map.entry(name).or_insert(0);
                *points += 1;
            }
        );
    }

    let mut winning = (vec![], 0);

    for (k, v) in tracker_map {
        if v > winning.1 {
            winning.0 = vec![k];
            winning.1 = v;
        }
    }

    println!("Winning reinder {} has points {}", winning.0[0], winning.1);
}