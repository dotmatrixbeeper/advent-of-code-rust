pub fn run(input: &str) {
    println!("======= DAY 2 =======");
    let data = input.lines().collect::<Vec<&str>>();
    part_1(&data);
    part_2(&data);
    println!("=====================\n");
}

const KEYPAD_PART_1: [[u8; 3]; 3] = [
    [1, 2, 3], 
    [4, 5, 6], 
    [7, 8, 9]
];

const KEYPAD_PART_2: [[char; 5]; 5] = [
    ['_', '_', '1', '_', '_'],
    ['_', '2', '3', '4', '_'],
    ['5', '6', '7', '8', '9'],
    ['_', 'A', 'B', 'C', '_'],
    ['_', '_', 'D', '_', '_']
];

fn part_1(data: &Vec<&str>) {
    println!("Part 1 --------------");
    let mut start_pos = (1, 1);
    let code = data
        .iter()
        .map(| line | {
            for c in line.chars() {
                match c {
                    'U' => use_keypad((-1, 0), &mut start_pos),
                    'L' => use_keypad((0, -1), &mut start_pos),
                    'D' => use_keypad((1, 0), &mut start_pos),
                    'R' => use_keypad((0, 1), &mut start_pos),
                    _ => panic!("Invlaid command"),
                }
            }
            return (b'0' + KEYPAD_PART_1[start_pos.0 as usize][start_pos.1 as usize]) as char;
        })
        .collect::<String>();
    println!("Code for the door: {}", code);
    println!("---------------------");
}

fn use_keypad((x, y): (i8, i8), start_pos: &mut (i8, i8)) {
    if start_pos.0 + x >= 0 
        && start_pos.0 + x <= 2 
        && start_pos.1 + y >= 0
        && start_pos.1 + y <= 2 {
        
        start_pos.0 += x;
        start_pos.1 += y;
    }
}

fn part_2(data: &Vec<&str>) {
    println!("Part 2 --------------");
    let mut start_pos = (2, 0);
    let code = data
        .iter()
        .map(| line | {
            for c in line.chars() {
                match c {
                    'U' => use_new_keypad((-1, 0), &mut start_pos),
                    'L' => use_new_keypad((0, -1), &mut start_pos),
                    'D' => use_new_keypad((1, 0), &mut start_pos),
                    'R' => use_new_keypad((0, 1), &mut start_pos),
                    _ => panic!("Invlaid command"),
                }
            }
            return KEYPAD_PART_2[start_pos.0 as usize][start_pos.1 as usize];
        })
        .collect::<String>();
    println!("Code for the door: {}", code);
    println!("---------------------");
}

fn use_new_keypad((x, y): (i8, i8), start_pos: &mut (i8, i8)) {
    if start_pos.0 + x >= 0 
        && start_pos.0 + x <= 4 
        && start_pos.1 + y >= 0
        && start_pos.1 + y <= 4 
        && KEYPAD_PART_2[(start_pos.0 + x) as usize][(start_pos.1 + y) as usize] != '_' {
        
        start_pos.0 += x;
        start_pos.1 += y;
    }    
}