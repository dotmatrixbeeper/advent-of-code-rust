/// ### Solution for Part 2
/// For this part, the value at register a 
/// starts at 1.
/// 
/// #### Rust Implementation Details
/// Rerun the same code with initial `a` value
/// at 1.

use crate::Instruction;

pub fn solve(instructions: &Vec<Instruction>) {
    let mut registers = vec![1, 0];

    let mut ins_count: i32 = 0;
    while let Some(ins) = instructions.get(ins_count as usize) {
        match ins {
            Instruction::JMP(offset) => {
                ins_count += offset;
            },
            Instruction::JIE(reg, offset) => {
                if registers[*reg as usize] % 2 == 0 {
                    ins_count += offset;
                } else {
                    ins_count += 1;
                }
            },
            Instruction::JIO(reg, offset) => {
                if registers[*reg as usize] == 1 {
                    ins_count += offset;
                } else {
                    ins_count += 1;
                }
            },
            Instruction::HLF(reg) => {
                registers[*reg as usize] /= 2;
                ins_count += 1;
            },
            Instruction::TPL(reg) => {
                registers[*reg as usize] *= 3;
                ins_count += 1;
            },
            Instruction::INC(reg) => {
                registers[*reg as usize] += 1;
                ins_count += 1;
            },
        }
    }

    println!("Value in register b: {}", registers[1]);

}