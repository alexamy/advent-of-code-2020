use crate::parser::Instruction;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Result {
    Cycle(i32),
    Finish(i32),
}

pub fn fix_corruption(instructions: Vec<Instruction>) -> i32 {
    for (i, instruction) in instructions.iter().enumerate() {
        match *instruction {
            Instruction::Acc(_) => continue,
            Instruction::Jmp(_) => {
                let copy = switch_instruction(&instructions, i);
                if let Result::Finish(accumulator) = interpret(copy) {
                    return accumulator;
                }
            }
            Instruction::Nop(_) => {
                let copy = switch_instruction(&instructions, i);
                if let Result::Finish(accumulator) = interpret(copy) {
                    return accumulator;
                }
            }
        }
    }

    0
}

fn switch_instruction(original: &Vec<Instruction>, index: usize) -> Vec<Instruction> {
    let mut instructions = original.to_vec();

    instructions[index] = match instructions[index] {
        Instruction::Jmp(offset) => Instruction::Nop(offset),
        Instruction::Nop(offset) => Instruction::Jmp(offset),
        Instruction::Acc(_) => panic!("Cant change Acc instruction"),
    };

    instructions
}

pub fn find_cycle(instructions: Vec<Instruction>) -> i32 {
    match interpret(instructions) {
        Result::Cycle(accumulator) => accumulator,
        _ => panic!("No cycle found"),
    }
}

fn interpret(instructions: Vec<Instruction>) -> Result {
    let mut accumulator: i32 = 0;
    let mut position: i32 = 0;
    let mut counts = HashMap::new();

    loop {
        if counts.contains_key(&position) {
            return Result::Cycle(accumulator);
        }

        counts
            .entry(position)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let mut next = 1;
        let instruction = &instructions[position as usize];
        match instruction {
            Instruction::Nop(_) => (),
            Instruction::Acc(offset) => accumulator += offset,
            Instruction::Jmp(offset) => next = *offset,
        }

        position += next;

        if position == instructions.len() as i32 {
            return Result::Finish(accumulator);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn corruption_fixed() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions = parser::parse(input);

        assert_eq!(fix_corruption(instructions), 8);
    }

    #[test]
    fn cycle_found() {
        let input = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        let instructions = parser::parse(input);

        assert_eq!(find_cycle(instructions), 5);
    }

    #[test]
    fn is_finished() {
        let instructions = vec![Instruction::Nop(0), Instruction::Acc(1)];

        assert_eq!(interpret(instructions), Result::Finish(1));
    }
}
