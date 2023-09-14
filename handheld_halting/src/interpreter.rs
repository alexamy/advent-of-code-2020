use crate::parser::Instruction;
use std::collections::HashMap;

pub fn find_cycle(instructions: Vec<Instruction>) -> i32 {
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
    use crate::parser;

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

        let instructions = parser::parse(input);

        assert_eq!(find_cycle(instructions), 5);
    }
}
