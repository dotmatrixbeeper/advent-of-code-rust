/// ### Solution for Part 1
/// This is a classic travelling salesperson problem. We need to
/// find the shortest path, without repetation, given the distances
/// between each city. 
/// We solve this by using the Held-Karp algorithm, with dynamic 
/// programming.  
/// 
/// #### Rust Implementation Details
/// We use dp to keep track of the minimum cost to visit the nodes 
/// in a subset.
/// parent keeps track of the previous node from which a subset of 
/// nodes are visited while having the lowest cost
/// We start with each individual node, and its distances from itself
/// as 0.
/// After finding the best path, we can retrace the path using the 
/// parent map.

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

                let mut min_cost = Distance::MAX;
                let mut best_prev = 0;

                // tyr all possible previous nodes in the path
                for prev in 0..node_count {
                    if prev == last || !is_in_subset(subset, prev) {
                        continue;
                    }

                    let prev_subset = remove_from_subset(subset, last);
                    if let Some(&prev_cost) = dp.get(&(prev_subset, prev)) {
                        let cost = prev_cost.saturating_add(graph.distance(prev, last));
                        if cost < min_cost {
                            min_cost = cost;
                            best_prev = prev;
                        }
                    }
                }

                if min_cost != Distance::MAX {
                    dp.insert((subset, last), min_cost);
                    parent.insert((subset, last), best_prev);
                }
            }
        }
    }

    // Find the minimum cost path by trying all possible ending nodes
    let full_subset = (1 << node_count) - 1; // All nodes visited
    let mut min_total_cost = Distance::MAX;
    let mut best_last_node = 0;

    for last in 0..node_count {
        if let Some(&cost) = dp.get(&(full_subset, last)) {
            if cost < min_total_cost {
                min_total_cost = cost;
                best_last_node = last;
            }
        }
    }

    println!("Minimum cost: {}", min_total_cost);

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