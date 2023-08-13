use crate::reader;
use crate::solution;

pub fn solve() -> usize {
    let map = reader::read_lines();

    solution::count_trees(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 270);
    }
}
