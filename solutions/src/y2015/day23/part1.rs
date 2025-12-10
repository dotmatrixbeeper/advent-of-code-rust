/// ### Solution for Part 1
/// This is a simple emulator problem.
/// We read the instructions and execute them.
/// 
/// #### Rust Implementation Details
/// We store the instructions in a vector, since
/// we have jump instructions in the given input.
/// We can use the order of the vector to jump to
/// locations conditionally.
/// while the next instruction exists
/// we evaluate the instruction and update
/// the registers if requried.
/// else we are done and we can print the value 
/// in register b.

use crate::y2015::day23::Instruction;

pub fn solve(instructions: &Vec<Instruction>) {
    let mut registers = vec![0, 0];

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