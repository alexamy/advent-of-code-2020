use std::fs;

pub fn get_data() -> Vec<u32> {
  let content = read();
  let value = process(&content);

  value
}

fn read() -> String {
  let directory = env!("CARGO_MANIFEST_DIR");
  let input_path = format!("{directory}/input.txt");
  let input = fs::read_to_string(input_path)
      .expect("No input file found");

  input
}

fn process(input: &str) -> Vec<u32> {
  input
    .split("\n")
    .filter(|r| !r.is_empty())
    .map(|n| n.parse::<u32>().unwrap())
    .collect()
}
