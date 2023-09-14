use crate::{
    parser::{self, Instruction},
    reader,
};
use std::collections::HashMap;

pub fn solve() -> i32 {
    let input = reader::read_input();

    solve_input(&input)
}

fn solve_input(input: &str) -> i32 {
    let instructions = parser::parse(input);

    interpret(instructions)
}

fn interpret(instructions: Vec<Instruction>) -> i32 {
    let mut accumulator: i32 = 0;
    let mut position: i32 = 0;
    let mut counts = HashMap::new();

    loop {
        if counts.contains_key(&position) {
            return accumulator;
        }

        counts
            .entry(position)
            .and_modify(|count| *count += 1)
            .or_insert(1);

        let instruction = &instructions[position as usize];
        let mut next = 1;
        match instruction {
            Instruction::Nop(_) => (),
            Instruction::Acc(offset) => accumulator += offset,
            Instruction::Jmp(offset) => next = *offset,
        }

        position += next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1797);
    }

    #[test]
    fn is_solved_simple() {
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

        assert_eq!(solve_input(input), 5);
    }
}
