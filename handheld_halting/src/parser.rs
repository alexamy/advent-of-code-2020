#[derive(Debug, PartialEq)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input.split("\n").map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let code = &line[0..3];
    let instruction = match code {
        "nop" => Instruction::Nop,
        "jmp" => Instruction::Jmp,
        "acc" => Instruction::Acc,
        _ => panic!("Unknown instruction"),
    };

    let offset = parse_number(&line[4..]);

    instruction(offset)
}

fn parse_number(line: &str) -> i32 {
    line.parse().expect("Expect a number for instruction")
}
