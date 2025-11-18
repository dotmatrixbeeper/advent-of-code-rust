/// ### Solution for Part 2
/// This follows the same premise as part 1, but the solution now
/// requries us to find the longest route. 
/// 
/// #### Rust Implementation Details
/// We change the comparision functions for min_cost to max_cost.

use std::collections::HashMap;

use crate::{Graph, NodeId, Subset, Distance};

pub fn solve(vertex_map: &HashMap<NodeId, String>, graph: &Graph) {
    let node_count = graph.node_count();

    // dp[subset][last] = minimum cost to visit all nodes in subset, ending at 'last'
    let mut dp: HashMap<(Subset, NodeId), Distance> = HashMap::new();
    
    // parent[subset][last] = previous node in optimal path for this state
    let mut parent: HashMap<(Subset, NodeId), NodeId> = HashMap::new();

    // base case: start from each individual node
    for start in 0..node_count {
        dp.insert((1 << start, start), 0);
    }

    // fill DP table for all subset sizes from 2 to node_count
    for subset_size in 2..=node_count {
        // iterate through all possible subset of given size
        for subset in all_subsets_of_size(node_count, subset_size) {
            // try ending the path at each node in the subset
            for last in 0..node_count {
                if !is_in_subset(subset, last) {
                    continue;
                }

                let mut max_cost = Distance::MIN;
                let mut best_prev = 0;

                // tyr all possible previous nodes in the path
                for prev in 0..node_count {
                    if prev == last || !is_in_subset(subset, prev) {
                        continue;
                    }

                    let prev_subset = remove_from_subset(subset, last);
                    if let Some(&prev_cost) = dp.get(&(prev_subset, prev)) {
                        let cost = prev_cost.saturating_add(graph.distance(prev, last));
                        if cost > max_cost {
                            max_cost = cost;
                            best_prev = prev;
                        }
                    }
                }

                if max_cost != Distance::MIN {
                    dp.insert((subset, last), max_cost);
                    parent.insert((subset, last), best_prev);
                }
            }
        }
    }

    // Find the minimum cost path by trying all possible ending nodes
    let full_subset = (1 << node_count) - 1; // All nodes visited
    let mut max_total_cost = Distance::MIN;
    let mut best_last_node = 0;

    for last in 0..node_count {
        if let Some(&cost) = dp.get(&(full_subset, last)) {
            if cost > max_total_cost {
                max_total_cost = cost;
                best_last_node = last;
            }
        }
    }

    println!("Maximum cost: {}", max_total_cost);

    let path = reconstruct_path(&parent, full_subset, best_last_node);
    for node in &path[..path.len() - 1] {
        print!("{} -> ", vertex_map.get(node).unwrap());
    }

    println!("{}", vertex_map.get(path.last().unwrap()).unwrap());


}

fn all_subsets_of_size(node_count: usize, size: usize) -> impl Iterator<Item = Subset> {
    (0..(1u32 << node_count))
        .filter(move |&subset| subset.count_ones() as usize == size)
}

fn is_in_subset(subset: Subset, node: NodeId) -> bool {
    (subset & (1 << node)) != 0
}

fn remove_from_subset(subset: Subset, node: NodeId) -> Subset {
    subset & !(1 << node)
}

fn reconstruct_path(
    parent: &HashMap<(Subset, NodeId), NodeId>,
    mut subset: Subset,
    mut last: NodeId,
) -> Vec<NodeId> {
    let mut path = Vec::new();

    // Build path backwards
    while subset.count_ones() > 1 {
        path.push(last);
        let prev = parent[&(subset, last)];
        subset = remove_from_subset(subset, last);
        last = prev;
    }

    path.push(last);
    path.reverse();

    path
}