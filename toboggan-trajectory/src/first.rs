use crate::reader;
use crate::solution::{self, Offset};

pub fn solve() -> usize {
    let map = reader::read_lines();

    let offset = Offset { x: 3, y: 1 };
    solution::count_trees(&map, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 270);
    }
}
