use std::collections::HashMap;

use super::parser::{self, Info, Row};

pub fn parse(description: &str) -> HashMap<&str, Row> {
    description
        .split("\n")
        .map(|line| parser::parse(line))
        .map(|row| (row.color, row))
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_parsed() {
        let input = "\
light red bags contain 2 bright white bag.
bright white bags contain 1 shiny gold bag.";

        assert_eq!(
            parse(input),
            HashMap::from([
                (
                    "light red",
                    Row {
                        color: "light red",
                        bags: vec![Info {
                            count: 2,
                            color: "bright white",
                        }],
                    }
                ),
                (
                    "bright white",
                    Row {
                        color: "bright white",
                        bags: vec![Info {
                            count: 1,
                            color: "shiny gold",
                        }],
                    }
                )
            ])
        );
    }
}
