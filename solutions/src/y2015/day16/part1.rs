/// ### Solution for Part 1
/// This is a find a match problem, and I have chosen to go the easy
/// way and just compare the given input to the search parameters.
/// Once we have three matches, we return.
/// 
/// #### Rust Implementation Details
/// We iter() over the list of Aunties and for each aunt find the number
/// of matches.
/// If the number of matches is three (since we only have three 
/// parameters) we return the current Aunt.

use crate::y2015::day16::Aunt;

pub fn solve(aunts: &Vec<Aunt>) {
    let ref_aunt = Aunt {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
        index: 0,
    };

    println!("Matching aunt: {:?}", aunts.iter().find(|&aunt| {
        let mut matched = 0;
        if aunt.children == ref_aunt.children {
            matched += 1;
        }

        if aunt.cats == ref_aunt.cats {
            matched += 1;
        }

        if aunt.samoyeds == ref_aunt.samoyeds {
            matched += 1;
        }

        if aunt.pomeranians == ref_aunt.pomeranians {
            matched += 1;
        }

        if aunt.akitas == ref_aunt.akitas {
            matched += 1;
        }

        if aunt.vizslas == ref_aunt.vizslas {
            matched += 1;
        }

        if aunt.goldfish == ref_aunt.goldfish {
            matched += 1;
        }

        if aunt.trees == ref_aunt.trees {
            matched += 1;
        }

        if aunt.cars == ref_aunt.cars {
            matched += 1;
        }

        if aunt.perfumes == ref_aunt.perfumes {
            matched += 1;
        }
        matched == 3
    }));
}