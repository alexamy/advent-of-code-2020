use std::fs;

#[derive(PartialEq, Debug)]
struct Rule {
  low_bound: u8,
  high_bound: u8,
  letter: char,
}

fn read() -> String {
  let directory = env!("CARGO_MANIFEST_DIR");
  let input_path = format!("{directory}/input.txt");
  let input = fs::read_to_string(input_path)
      .expect("No input file found");

  input
}

fn process_row(row: &str) -> (&str, Rule) {
  let parts: Vec<&str> = row.split(" ").collect();
  if parts.len() != 3 {
    panic!("Must find 3 parts in row");
  }

  let bounds = parts[0];
  let letter = parts[1];
  let password = parts[2];

  let bounds: Vec<&str> = bounds.split("-").collect();
  if bounds.len() != 2 {
    panic!("Must find 2 parts in bounds");
  }

  let letter: Vec<&str> = letter.split(":").collect();
  if letter.len() != 2 {
    panic!("Must find 2 parts in letter")
  }

  let low_bound = bounds[0].parse::<u8>().unwrap();
  let high_bound = bounds[1].parse::<u8>().unwrap();
  let letter = letter[0].parse::<char>().unwrap();

  (password, Rule {
    low_bound,
    high_bound,
    letter,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn processing_row() {
    assert_eq!(
      process_row("9-14 p: bppzpwhzdgnpnh"),
      ("bppzpwhzdgnpnh", Rule {
        low_bound: 9,
        high_bound: 14,
        letter: 'p',
      }),
    )
  }
}
