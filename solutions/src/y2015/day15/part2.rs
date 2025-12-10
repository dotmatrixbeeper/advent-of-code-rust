use std::collections::HashSet;

use crate::y2015::day15::Ingredient;

pub fn solve(ingredients: &Vec<Ingredient>) {
    let mut amounts: Vec<i32> = vec![0; ingredients.len()];
    amounts[0] = 100;

    let mut to_check = vec![];
    to_check.push(amounts);

    let mut visited = HashSet::new();
    let mut max = 0;
    while !to_check.is_empty() {
        let current = to_check.remove(0);
        let current_score = calculate_score(&current, &ingredients);
        max = max.max(current_score);
        for i in 0..current.len() {
            for j in 0..current.len() {
                if i == j {
                    continue;
                }
                let mut new = current.clone();
                new[i] += 1;
                new[j] -= 1;
                if new[i] > 100 || new[j] < 0 {
                    continue;
                }
                if visited.contains(&new) {
                    continue;
                }
                to_check.push(new.clone());
                visited.insert(new);
            }
        }
    }
    println!("Max score: {}", max);
}

fn calculate_score(current: &Vec<i32>, ingredients: &Vec<Ingredient>) -> i32 {
    let mut scores = vec![0; 5];
    for (i, ingredient) in ingredients.iter().enumerate() {
        scores[0] += ingredient.capacity * current[i];
        scores[1] += ingredient.durability * current[i];
        scores[2] += ingredient.flavor * current[i];
        scores[3] += ingredient.texture * current[i];
        scores[4] += ingredient.calories * current[i];
    }

    let mut result = 1;

    for i in 0..4 {
        if scores[4] == 500 {
            result *= 0.max(scores[i]);
        } else {
            result = 0;
        }
    }
    result

}