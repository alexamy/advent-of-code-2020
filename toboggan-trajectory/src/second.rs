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

    let result = slopes
        .into_iter()
        .map(|slope| solution::count_trees(&map, slope))
        .product();

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
