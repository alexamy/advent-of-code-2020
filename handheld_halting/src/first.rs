use crate::{interpreter, parser, reader};

pub fn solve() -> i32 {
    let input = reader::read_input();
    let instructions = parser::parse(&input);

    interpreter::find_cycle(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1797);
    }
}
