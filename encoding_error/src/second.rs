use crate::{finder, reader};

pub fn solve() -> u64 {
    let input = reader::read_input();
    let numbers: Vec<u64> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    let breaking_number = finder::find_breaking(&numbers, 25).unwrap();
    let result = finder::find_sum_sequence(&numbers, breaking_number).unwrap();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 129444555);
    }
}
