use crate::{interpreter, parser, reader};

pub fn solve() -> i32 {
    let input = reader::read_input();
    let instructions = parser::parse(&input);

    interpreter::fix_corruption(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1036);
    }
}
