use std::collections::HashMap;

use super::parser::{self, Info, Row};

pub fn count(description: &str, target: &str) -> u32 {
    let rows = as_hashmap(description);

    let mut count = 0;
    for (_, Row { color, bags }) in rows {
        if color == target {
            continue;
        }

        for bag in bags {
            if bag.color == target {
                count += 1;
            } else {
            }
        }
    }

    count
}

fn count_rec(hash: HashMap<&str, Row>, target: &str, count: u32) {}

fn as_hashmap(description: &str) -> HashMap<&str, Row> {
    description
        .split("\n")
        .map(|line| parser::parse(line))
        .map(|row| (row.color, row))
        .collect()
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
dark green bags contain 1 ultra pink bag.";

        assert_eq!(count(input, "shiny gold"), 3);
    }
}
