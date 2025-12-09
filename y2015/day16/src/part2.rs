/// ### Solution for Part 2
/// The second part of this problem changes the original problem
/// statement to say that for certain attributes the machine 
/// detects range instead of a particular value.
/// We change the matching algorithm to check for ranges instead.
/// 
/// #### Rust Implementation Details
/// We iter() over the list of Aunties and for each aunt find the number
/// of matches.
/// If the number of matches is three (since we only have three 
/// parameters) we return the current Aunt.
use crate::Aunt;

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

    println!("Matching aunts in range: {:?}", aunts.iter().find(|&aunt| {
        let mut matched = 0;
        if aunt.children == ref_aunt.children {
            matched += 1;
        }

        if aunt.cats.is_some() && aunt.cats > ref_aunt.cats {
            matched += 1;
        }

        if aunt.samoyeds == ref_aunt.samoyeds {
            matched += 1;
        }

        if aunt.pomeranians.is_some() && aunt.pomeranians < ref_aunt.pomeranians {
            matched += 1;
        }

        if aunt.akitas == ref_aunt.akitas {
            matched += 1;
        }

        if aunt.vizslas == ref_aunt.vizslas {
            matched += 1;
        }

        if aunt.goldfish.is_some() && aunt.goldfish < ref_aunt.goldfish {
            matched += 1;
        }

        if aunt.trees.is_some() && aunt.trees > ref_aunt.trees {
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