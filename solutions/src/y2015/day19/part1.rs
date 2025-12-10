/// ### Solution for Part 1
/// This problem requires us to take the given molecule, 
/// find number of unique molecules we can transform this 
/// molecule using the list of transformations.
/// 
/// #### Rust Implementation Details
/// To solve this we iterate over the keys we have and find
/// matches for them in the given molecule. Then for the 
/// match we take the substitutions available and apply them 
/// one by one, pushing them into a set.
/// At the end we just need the length of the set.

use std::{collections::{HashMap, HashSet}, hash::{DefaultHasher, Hash, Hasher}};

pub fn solve(transform_map: &HashMap<String, Vec<String>>, initial_molecule: &String) {
    let mut result_set = HashSet::new();
    for key in transform_map.keys() {
        initial_molecule
            .match_indices(key)
            .for_each(|(index, _)| {
                transform_map.get(key).unwrap()
                    .iter()
                    .for_each(|replacement| {
                        let mut replaced_string = String::new();
                        replaced_string.push_str(&initial_molecule[..index]);
                        replaced_string.push_str(&replacement);
                        if index + key.len() <= initial_molecule.len() {
                            replaced_string.push_str(&initial_molecule[index + key.len()..]);
                        }
                        let mut hasher = DefaultHasher::new();
                        replaced_string.hash(&mut hasher);
                        result_set.insert(hasher.finish());
                    });
            });
    }
    println!("Number of distinct molecules: {}", result_set.len());
    
}