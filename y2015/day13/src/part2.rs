/// ### Solution for Part 2
/// The second part needs us to add one more node to the problem, with 
/// zero costs for any relationship to any other.
/// 
/// #### Rust Implementation Details
/// We keep the same solution while adding one more node to the graph 
/// and vertex_map

use std::{collections::HashMap, i32};

use crate::Graph;

pub fn solve(graph: &mut Graph, vertex_map: &mut HashMap<usize, String>) {
    add_one_to_graph(graph, vertex_map);

    let node_count = graph.node_count();

    let mut dp: HashMap<(u32, usize), i32> = HashMap::new();
    let mut parent: HashMap<(u32, usize), usize> = HashMap::new();
    println!("Node map: {:?}", vertex_map);

    dp.insert((1, 0), 0);

    for subset_size in 2..=node_count {
        for subset in subsets_containing_zero(node_count, subset_size) {
            for last in 1..node_count {
                if !is_in_subset(subset, last) {
                    continue;
                }

                let mut max_cost = i32::MIN;
                let mut best_prev = 0;

                for prev in 0..node_count {
                    if prev == last {
                        continue;
                    }

                    let prev_subset = remove_from_subset(subset, last);
                    if let Some(&prev_cost) = dp.get(&(prev_subset, prev)) {
                        let cost = prev_cost.saturating_add(graph.distance(prev, last)).saturating_add(graph.distance(last, prev));
                        if cost > max_cost {
                            max_cost = cost;
                            best_prev = prev;
                        }
                    }
                }

                if max_cost != i32::MIN {
                    dp.insert((subset, last), max_cost);
                    parent.insert((subset, last), best_prev);
                }
            }
        }
    }

    let full_subset = (1 << node_count) - 1;
    let mut max_total_cost = i32::MIN;
    let mut best_last_node = 0;

    for last in 1..node_count {
        if let Some(&cost) = dp.get(&(full_subset, last)) {
            let total_cost = cost.saturating_add(graph.distance(last, 0)).saturating_add(graph.distance(0, last));
            if total_cost > max_total_cost {
                max_total_cost = total_cost;
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

fn reconstruct_path(
    parent: &HashMap<(u32, usize), usize>,
    mut subset: u32,
    mut last: usize,
) -> Vec<usize> {
    let mut path = Vec::new();

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

fn remove_from_subset(subset: u32, node: usize) -> u32 {
    subset & !(1 << node)
}

fn is_in_subset(subset: u32, node: usize) -> bool {
    (subset & (1 << node)) != 0
}

fn subsets_containing_zero(node_count: usize, subset_size: usize) -> impl Iterator<Item = u32> {
    (0..(1u32 << node_count))
        .filter(move | &subset | {
            subset & 1 == 1 &&
            subset.count_ones() as usize == subset_size
        })
}

fn add_one_to_graph(graph: &mut Graph, vertex_map: &mut HashMap<usize, String>) {
    // add a new vertex and increment node count
    for i in 0..=graph.node_count() {
        graph.set_distance(graph.node_count(), i, 0);
        graph.set_distance(i, graph.node_count(), 0);
    }
    vertex_map.insert(graph.node_count(), String::from("You"));
    graph.set_node_size(graph.node_count() + 1);
}