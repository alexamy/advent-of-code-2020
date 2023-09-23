use std::fs;

pub struct Input(String);

impl Input {
    pub fn read() -> Self {
        let directory = env!("CARGO_MANIFEST_DIR");
        let input_path = format!("{directory}/input.txt");
        let input = fs::read_to_string(input_path).expect("No input file found");

        Input(input)
    }

    pub fn raw(&self) -> &str {
        let Input(input) = self;

        input
    }

    pub fn lines(&self) -> Vec<&str> {
        let Input(input) = self;

        input.split("\n").filter(|line| !line.is_empty()).collect()
    }
}
