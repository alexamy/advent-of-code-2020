pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

pub fn solve(input: &str) -> i32 {
    0
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
