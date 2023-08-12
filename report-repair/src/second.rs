use super::input;

pub fn solve() -> u32 {
    let mut numbers = input::get_data();
    numbers.sort();

    for number in &numbers {
        let candidates: Vec<u32> = numbers
            .to_vec()
            .iter()
            .map(|n| *n)
            .filter(|n| number + n < 2020)
            .collect();

        if candidates.len() < 2 {
            continue;
        }
    };

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 902451);
    }
}
