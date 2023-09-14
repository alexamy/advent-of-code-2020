use std::collections::HashMap;

use super::parser::Row;

pub fn parse(description: &str) -> HashMap<&str, Row> {
    let result = HashMap::new();

    result
}

#[cfg(test)]
mod tests {
    use crate::bag::parser::Info;

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
