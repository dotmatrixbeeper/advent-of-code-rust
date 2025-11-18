/// ### Solution for Part 2
/// Same as part 1 with the exception of wire b being supplied
/// with the value in wire b from part 1.
/// 
/// #### Rust Implementation Details
/// We maintain a map for the wire values.
/// We create a Instruction graph:
///     - Instructions are operations with two operands and
///         one operator.
///     - If an instruction does not have an operator we make
///         it the assignment operator
/// The `resolve_wire` function is a recursive function that 
/// returns value for a wire given to it.
/// When it finds a dead end, it returns an Error
/// Else it returns an Ok(value) where value is the value of 
/// the wire.

use std::collections::HashMap;
use regex::Regex;

struct Instruction<'a> {
    op1: Option<&'a str>,
    operation: Operation,
    op2: Option<&'a str>
}

enum Operation {
    And,
    Or,
    Not,
    Lshift,
    Rshift,
    Assignment,
}

impl From<&str> for Operation {
    fn from(operator: &str) -> Self {
        match operator {
            "AND" => Self::And,
            "OR" => Self::Or,
            "LSHIFT" => Self::Lshift,
            "RSHIFT" => Self::Rshift,
            _ => Self::Not,
        }        
    }
}

pub fn solve() {
    let data = include_str!("../resources/input.txt");
    let mut wire_map: HashMap<&str, u16> = HashMap::new();
    let mut instruction_map: HashMap<&str, Instruction> = HashMap::new();
    let re = Regex::new(r"^(?<op1>\d*|\w*)? ?(?<operator>AND|OR|NOT|LSHIFT|RSHIFT)? ?(?<op2>\d*|\w*) -> (?<res>\w+)$").unwrap();

    for instruction in data.lines() {
        let caps = re.captures(instruction).unwrap();
        let wire = caps.name("res").unwrap().as_str();
        
        match caps.name("operator") {
            Some(operator) => {
                instruction_map.insert(wire, Instruction {
                    op1: match caps.name("op1") {
                        Some(op1) => {
                            Some(op1.as_str())
                        }
                        None => None
                    },
                    operation: Operation::from(operator.as_str()),
                    op2: Some(caps.name("op2").unwrap().as_str()),
                });
            },
            None => {
                let op = vec![caps.name("op1"), caps.name("op2")]
                    .iter()
                    .filter(|op| !op.unwrap().as_str().is_empty())
                    .last()
                    .unwrap()
                    .unwrap()
                    .as_str();

                if let Ok(val) = op.parse::<u16>() {
                    if wire == "b" {
                        wire_map.insert(wire, 16076);
                    } else {
                        wire_map.insert(wire, val);
                    }
                } else {
                    instruction_map.insert(wire, 
                        Instruction { 
                            op1: None, 
                            operation: Operation::Assignment, 
                            op2: Some(op),
                        }
                    );
                }
            }
        }
    }

    if let Ok(val) = resolve_wire("a", &mut wire_map, &instruction_map) {
        println!("Signal on wire a: {val}")
    }
}

fn resolve_wire<'a>(wire: &'a str, wire_map: &mut HashMap<&'a str, u16>, instruction_map: &HashMap<&str, Instruction<'a>>) -> Result<u16, ()> {
    if let Ok(val) = wire.parse::<u16>() {
        return Ok(val);
    }

    if wire_map.contains_key(wire) {
        return Ok(*wire_map.get(wire).unwrap());
    }

    let Some(instruction) = instruction_map.get(wire) else { return Err(()) };

    match instruction.operation {
        Operation::And => {
            let Ok(op1) = resolve_wire(instruction.op1.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = op1 & op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
        Operation::Or => {
            let Ok(op1) = resolve_wire(instruction.op1.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = op1 | op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
        Operation::Not => {
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = !op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
        Operation::Lshift => {
            let Ok(op1) = resolve_wire(instruction.op1.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = op1 << op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
        Operation::Rshift => {
            let Ok(op1) = resolve_wire(instruction.op1.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = op1 >> op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
        Operation::Assignment => {
            let Ok(op2) = resolve_wire(instruction.op2.unwrap(), wire_map, instruction_map) else { return Err(()) };
            let value = op2;
            wire_map.insert(wire, value);
            return Ok(value);
        },
    }
}