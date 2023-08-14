use crate::reader;
use crate::solution::{self, Slope};

pub fn solve() -> usize {
    let map = reader::read_lines();

    let slope = Slope { x: 3, y: 1 };
    solution::count_trees(&map, slope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 270);
    }
}
