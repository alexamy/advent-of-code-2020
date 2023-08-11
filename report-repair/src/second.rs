use std::fs;

pub fn solve() -> u32 {
    let directory = env!("CARGO_MANIFEST_DIR");
    let input_path = format!("{directory}/input.txt");
    let input = fs::read_to_string(input_path)
        .expect("No input file found");

    let numbers: Vec<u32> = input
        .split("\n")
        .filter(|r| !r.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

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
        assert_eq!(solve(), 0);
    }
}
