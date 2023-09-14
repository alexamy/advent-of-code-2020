use crate::{interpreter, reader};

pub fn solve() -> i32 {
    let input = reader::read_input();

    interpreter::solve(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1797);
    }
}
