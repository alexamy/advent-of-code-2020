use super::input;

pub fn solve() -> u32 {
    let numbers = input::get_data();

    for number in &numbers {
        let target = 2020 - number;
        let is_target = numbers.contains(&target);
        if is_target {
            let result = number * target;
            return result;
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
