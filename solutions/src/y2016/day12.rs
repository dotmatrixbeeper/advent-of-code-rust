use std::collections::HashMap;

pub fn run(input: &str) {
    println!("======= DAY 12 =======");
    let mut registers = HashMap::new();
    registers.insert("a", 0);
    registers.insert("b", 0);
    registers.insert("c", 0);
    registers.insert("d", 0);
    part_1(&input, &mut registers);
    part_2(&input, &mut registers);
    println!("======================\n");
}

fn part_1<'a>(input: &'a str, registers: &mut HashMap<&'a str, i32>) {
    println!("Part 1 ---------------");
    execute(input, registers);
    println!("----------------------");
}

fn execute<'a>(input: &'a str, registers: &mut HashMap<&'a str, i32>) {
    let instructions = input.lines().collect::<Vec<&str>>();
    let mut count = 0;
    while (count as usize) < instructions.len() {
        let parts = instructions[count as usize].split(' ').collect::<Vec<&str>>();
        if parts[0] == "cpy" {
            exec_cpy(parts[1], parts[2], registers);
            count += 1;
        } else if parts[0] == "inc" {
            exec_inc(parts[1], registers);
            count += 1;
        } else if parts[0] == "dec" {
            exec_dec(parts[1], registers);
            count += 1;
        } else {
            exec_jnz(&mut count, parts[1], parts[2], registers);
        }
    }
    println!("Result in register in a: {}", registers.get("a").unwrap());
}

fn exec_cpy<'a>(first: &'a str, second: &'a str, registers: &mut HashMap<&'a str, i32>) {
    if let Ok(value) = first.parse::<i32>() {
        registers.insert(second, value);
    } else {
        registers.insert(second, *registers.get(first).unwrap());
    }
}

fn exec_inc<'a>(register: &'a str, registers: &mut HashMap<&'a str, i32>) {
    registers.entry(register).and_modify(|e| { *e += 1; });
}

fn exec_dec<'a>(register: &'a str, registers: &mut HashMap<&'a str, i32>) {
    registers.entry(register).and_modify(|e| { *e -= 1; });
}

fn exec_jnz<'a>(count: &mut i32, first: &'a str, second: &'a str, registers: &mut HashMap<&'a str, i32>) {
    if let Ok(value) = first.parse::<i32>() {
        if value != 0 {
            *count += second.parse::<i32>().unwrap();
        } else {
            *count += 1;
        }
    } else {
        if *registers.get(first).unwrap() != 0 {
            *count += second.parse::<i32>().unwrap();
        } else {
            *count += 1;
        }
    }
}

fn part_2<'a>(input: &'a str, registers: &mut HashMap<&'a str, i32>) {
    println!("Part 2 ---------------");
    registers.insert("c", 1);
    execute(input, registers);
    println!("----------------------");
}