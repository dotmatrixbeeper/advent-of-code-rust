/// ### Solution for Part 1
/// This problem requires finding the smallest subset
/// that can sum upto sum / 3 exactly. This can be 
/// solved by backtracking since we need to find 
/// all the combinations, which can be variable in length.
/// 
/// #### Rust Implementation Details
/// We find the subsets that can make the sum first, using
/// a recursive backtracking method.
/// Then we find the minimum size out of the combinations
/// Then we find the combination with the lowest 'quantum 
/// entanglement'

pub fn solve(weights: &Vec<u32>) {
    let target_weight = weights.iter().sum::<u32>() / 3;

    // find all combinations of weights that make target_weight
    let combinations = find_subsets(&weights, target_weight);
    let min = combinations.iter()
        .map(|combination| combination.len())
        .min()
        .unwrap();

    let min_combinations = combinations.iter()
        .filter(|combinations| combinations.len() == min)
        .map(|combinations| combinations.iter().map(|n| *n as u64).product::<u64>())
        .min()
        .unwrap();

    println!("Min Quantum Entanglement: {min_combinations}");
}

fn find_subsets(weights: &Vec<u32>, target_weight: u32) -> Vec<Vec<u32>> {
    let mut result = vec![];
    backtrack(0, &mut vec![], 0, &mut result, target_weight, &weights);
    result
}

fn backtrack(start: u32, current_subset: &mut Vec<u32>, current_sum: u32, result: &mut Vec<Vec<u32>>, target: u32, weights: &Vec<u32>) {
    if current_sum == target {
        result.push(current_subset.clone());
        return;
    }

    if current_sum > target {
        return;
    }

    for i in start..weights.len() as u32{
        if i > start && weights[i as usize] == weights[i as usize - 1] {
            continue;
        }

        current_subset.push(weights[i as usize]);
        backtrack(i + 1, current_subset, current_sum + weights[i as usize], result, target, weights);
        current_subset.pop();
    }
}