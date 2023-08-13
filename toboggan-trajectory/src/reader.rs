use std::fs;

pub fn read_lines() -> Vec<String> {
    read()
        .split("\n")
        .filter(|r| !r.is_empty())
        .map(String::from)
        .collect()
}

pub fn read() -> String {
    let directory = env!("CARGO_MANIFEST_DIR");
    let input_path = format!("{directory}/input.txt");
    let input = fs::read_to_string(input_path).expect("No input file found");

    input
}
