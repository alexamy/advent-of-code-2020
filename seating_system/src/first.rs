use crate::{reader, solver};

pub fn solve() -> u32 {
    let input = reader::read_input();

    solver::count_seats(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 0)
    }
}
