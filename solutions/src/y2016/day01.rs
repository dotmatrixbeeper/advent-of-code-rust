use std::collections::HashSet;

#[derive(Clone, Copy)]
struct Point {
    x: i16,
    y: i16
}

struct Coordinate {
    point: Point,
    direction: Direction
}

// Direction enum holds its own increment/decrement multiplier
enum Direction {
    North(i16, i16),
    West(i16, i16),
    South(i16, i16),
    East(i16, i16)
}

impl Direction {
    // Need a function to change direction given left 
    // or right
    fn change_direction(&mut self, turn: &str) {
        match turn {
            "L" => self.turn_left(),
            "R" => self.turn_right(),
            _ => panic!("Invalid turn")
        }
    }
    
    // Easiest to hardcode the changes
    fn turn_left(&mut self) {
        *self = match self {
            Self::North(_, _) => Self::West(-1, 0),
            Self::West(_, _) => Self::South(0, -1),
            Self::South(_, _) => Self::East(1, 0),
            Self::East(_, _) => Self::North(0, 1)
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Self::North(_, _) => Self::East(1, 0),
            Self::East(_, _) => Self::South(0, -1),
            Self::South(_, _) => Self::West(-1, 0),
            Self::West(_, _) => Self::North(0, 1)
        }
    }
}

impl Coordinate {
    fn new() -> Self {
        Coordinate { point: Point { x: 0, y: 0 }, direction: Direction::North(0, 1) }
    }

    fn change_direction(&mut self, turn: &str) {
        self.direction.change_direction(turn);
    }

    fn get_deltas(&self) -> (i16, i16) {
        match self.direction {
            Direction::North(x, y) => (x, y),
            Direction::West(x, y) => (x, y),
            Direction::South(x, y) => (x, y),
            Direction::East(x, y) => (x, y),
        }
    }
    
}

pub fn run(input: &str) {
    println!("======= DAY 1 =======");
    let data = input.split(", ").collect::<Vec<&str>>();
    part_1(&data);
    part_2(&data);
    println!("=====================\n");
}

fn part_1(data: &Vec<&str>) {
    println!("Part 1 --------------");
    let mut coordinate = Coordinate::new();
    for instruction in data {
        // For each instruction split the turn instruction from the steps
        let (turn, steps) = instruction.split_at(1);
        let steps = steps.parse::<i16>().unwrap();
        coordinate.change_direction(turn);                      // First change direction
        match coordinate.direction {
            // For any direction just multiply steps with incrementer/decrementer of the direction
            Direction::North(x, y) | Direction::West(x, y) | Direction::East(x, y) | Direction::South(x, y) => {
                coordinate.point.x += x * steps;
                coordinate.point.y += y * steps;
            }
        }
    }

    // Find absolute distance by finding x, y difference from initial (0, 0) 
    // and calling abs() on it
    let distance_to_hq = (0 - coordinate.point.x).abs() + (0 - coordinate.point.y).abs();
    println!("Distance to HQ is: {} blocks", distance_to_hq);
    println!("---------------------")
}

/// in the second part we need to find the first location that's revisited.
/// this requires us to remember the points visited, so now we should change our
/// implementation to record these points in a set. 
/// before adding to the set we check if it exists in the set, and if so we 
/// output that and exit.
fn part_2(data: &Vec<&str>) {
    println!("Part 2 --------------");

    let mut coordinate = Coordinate::new();
    // This vector stores where the turns happen
    // We can derive the route lines from this and find intersection
    let mut coord_set: HashSet<(i16, i16)> = HashSet::new();
    coord_set.insert((0, 0));
    'outer: for instruction in data {
        // For each instruction split the turn instruction from the steps
        let (turn, steps) = instruction.split_at(1);
        let steps = steps.parse::<i16>().unwrap();
        coordinate.change_direction(turn);
        let (delta_x, delta_y) = coordinate.get_deltas();
        for _ in 0..steps {
            coordinate.point.x += delta_x;
            coordinate.point.y += delta_y;

            if coord_set.contains(&(coordinate.point.x, coordinate.point.y)) { 
                let distance_to_hq = (0 - coordinate.point.x).abs() + (0 - coordinate.point.y).abs();
                println!("Distance to HQ is: {} blocks", distance_to_hq);
                break 'outer;
            } else {
                coord_set.insert((coordinate.point.x, coordinate.point.y));
            }
        } 
    }
    println!("---------------------")
}