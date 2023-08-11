use std::fs;

fn main() {
    println!("Hello, world!");
}

fn solve() -> u32 {
    let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
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
    fn first_solution() {
        assert_eq!(solve(), 902451);
    }
}
