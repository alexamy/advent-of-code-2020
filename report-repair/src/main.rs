use std::fs;

fn main() {
    let input_path = env!("CARGO_MANIFEST_DIR").to_owned() + "/input.txt";
    let numbers = fs::read_to_string(input_path)
        .expect("No input file found");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        main();
    }
}
