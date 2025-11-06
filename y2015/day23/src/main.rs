use regex::{Captures, Regex};

mod part1;
mod part2;

type Register = u8;

#[derive(Debug)]
enum Instruction {
    JMP(i32),
    JIE(Register, i32),
    JIO(Register, i32),
    HLF(Register),
    TPL(Register),
    INC(Register)
}

fn main() {
    let instructions = transform(include_str!("../resources/input.txt"));
    part1::solve(&instructions);
    part2::solve(&instructions);
}

fn transform(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"^(?<instruction>jmp|jie|jio|hlf|tpl|inc) (((?<register>\w+),? ?(?<offset2>(\+|-)?\d*))|(?<offset1>(\+|-)\d+))$").unwrap();
    input.lines()
        .map(|instruction| {
            let captures = re.captures(instruction).unwrap();
            get_instruction(&captures)
        })
        .collect()
}

fn get_instruction(captures: &Captures<'_>) -> Instruction {
    match &captures["instruction"] {
        "jmp" => {
            Instruction::JMP(get_offset(&captures["offset1"]))
        },
        "jie" => {
            Instruction::JIE(get_register(&captures["register"]), get_offset(&captures["offset2"]))
        },
        "jio" => {
            Instruction::JIO(get_register(&captures["register"]), get_offset(&captures["offset2"]))
        },
        "hlf" => {
            Instruction::HLF(get_register(&captures["register"]))
        },
        "tpl" => {
            Instruction::TPL(get_register(&captures["register"]))
        },
        _ => {
            Instruction::INC(get_register(&captures["register"]))
        }
    }
}

fn get_register(reg: &str) -> Register {
    match reg {
        "a" => 0,
        _ => 1
    }
}

fn get_offset(offset: &str) -> i32 {
    offset.parse().unwrap()
    
}