use crate::reader;
use crate::solution::{self, Slope};

pub fn solve() -> usize {
    let map = reader::read_lines();

    let slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 },
    ];

    let mut result = 1;

    for slope in slopes {
        let count = solution::count_trees(&map, slope);
        result *= count;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 2122848000);
    }
}
