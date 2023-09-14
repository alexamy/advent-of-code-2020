use std::collections::HashMap;

use super::parser::{self, Info, Row};

pub fn count(description: &str) -> u32 {
    0
}

fn collect(description: Vec<&str>) -> HashMap<&str, Row> {
    description
        .iter()
        .map(|line| parser::parse(line))
        .map(|row| (row.color, row))
        .collect::<HashMap<_, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_counted() {
        let input = "\
bright white bags contain 1 shiny gold bag.
light red bags contain 2 bright white bag.
lemon orange bags contain 3 shiny gold bag.
dark green bags contain 1 ultra pink bag.
        ";

        assert_eq!(count(input), 3);
    }

    #[test]
    fn is_parsed() {
        let input = vec![
            "light red bags contain 2 bright white bag.",
            "bright white bags contain 1 shiny gold bag.",
        ];

        assert_eq!(
            collect(input),
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
