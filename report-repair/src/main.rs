use std::fs;

fn main() {
    let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
    let input = fs::read_to_string(input_path)
        .expect("No input file found");

    let numbers: Vec<i16> = input
        .split("\n")
        .filter(|r| !r.is_empty())
        .map(|n| n.parse().unwrap())
        .collect();

    dbg!(numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        main();
    }
}
