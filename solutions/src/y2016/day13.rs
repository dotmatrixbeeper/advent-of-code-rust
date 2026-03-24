use std::collections::{HashMap, HashSet, VecDeque};

const SOURCE: (u32, u32) = (1, 1);
const TARGET: (u32, u32) = (31, 39);

pub fn run(input: &str) {
    println!("======= DAY 13 =======");
    let seed = input.trim().parse::<u32>().unwrap();
    part_1(seed);
    part_2(seed);
    println!("======================\n");
}

fn is_wall(seed: u32, x: u32, y: u32) -> bool {
    ((x * x + 3 * x + 2 * x * y + y + y * y) + seed).count_ones() % 2 != 0
}

fn part_1(seed: u32) {
    println!("Part 1 ---------------");
    let mut queue = VecDeque::new();
    let mut came_from: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

    queue.push_back((SOURCE.0, SOURCE.1, 0u32));
    came_from.insert(SOURCE, SOURCE);

    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == TARGET {
            println!("Steps to target: {}", steps);
            retrace_steps(seed, came_from);
            break;
        }

        for (nx, ny) in neighbors(x, y) {
            if !is_wall(seed, nx, ny) && !came_from.contains_key(&(nx, ny)) {
                queue.push_back((nx, ny, steps + 1));
                came_from.insert((nx, ny), (x, y));
            }
        }
    }
    println!("----------------------");
}

fn retrace_steps(seed: u32, path_map: HashMap<(u32, u32), (u32, u32)>) {
    let mut path = HashSet::new();
    let mut current = TARGET;
    while current != SOURCE {
        path.insert(current);
        current = path_map[&current];
    }

    let max_x = path.iter().map(|&(x, _)| x).max().unwrap_or(TARGET.0) + 2;
    let max_y = path.iter().map(|&(_, y)| y).max().unwrap_or(TARGET.1) + 2;

    for j in 0..max_y {
        for i in 0..max_x {
            if (i, j) == SOURCE {
                print!("\x1b[31mS\x1b[0m");
            } else if (i, j) == TARGET {
                print!("\x1b[31mT\x1b[0m");
            } else if path.contains(&(i, j)) {
                print!("\x1b[32mO\x1b[0m");
            } else if is_wall(seed, i, j) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn neighbors(x: u32, y: u32) -> Vec<(u32, u32)> {
    let mut result = vec![(x + 1, y), (x, y + 1)];
    if x > 0 { result.push((x - 1, y)); }
    if y > 0 { result.push((x, y - 1)); }
    result
}


fn part_2(seed: u32) {
    println!("Part 2 ---------------");
    let mut queue = VecDeque::new();
    let mut visited: HashSet<(u32, u32)> = HashSet::new();

    queue.push_back((SOURCE.0, SOURCE.1, 0u32));
    visited.insert(SOURCE);

    while let Some((x, y, steps)) = queue.pop_front() {
        for (nx, ny) in neighbors(x, y) {
            if !is_wall(seed, nx, ny) && !visited.contains(&(nx, ny)) && steps < 50 {
                queue.push_back((nx, ny, steps + 1));
                visited.insert((nx, ny));
            }
        }
    }

    println!("Number of unique locations visited within 50 steps: {}", visited.len());
    println!("----------------------");
}
