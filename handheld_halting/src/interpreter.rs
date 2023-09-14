#[derive(Debug, PartialEq)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub fn solve(input: &str) -> i32 {
    let instructions = parse(input);
    println!("{:?}", instructions);

    0
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input.split("\n").map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let code = &line[0..3];
    let offset = parse_number(&line[4..]);

    match code {
        "nop" => Instruction::Nop(offset),
        "jmp" => Instruction::Jmp(offset),
        "acc" => Instruction::Acc(offset),
        _ => panic!("Unknown instruction"),
    }
}

fn parse_number(line: &str) -> i32 {
    line.parse().expect("Expect a number for instruction")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
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

        assert_eq!(solve(input), 5);
    }
}
