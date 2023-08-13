use crate::reader;
use crate::solution;

pub fn solve() -> usize {
    let map = reader::read_lines();

    let right_offset = 3;
    solution::count_trees(&map, right_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 270);
    }
}
