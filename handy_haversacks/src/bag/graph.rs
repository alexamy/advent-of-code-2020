use std::collections::HashMap;

use super::parser::{self, Info, Row};

pub fn count(description: &str, target: &str) -> u32 {
    let rows = description.split("\n").map(|line| parser::parse(line));

    let mut entries = HashMap::new();
    for Row { color, bags } in rows {
        for Info { color, count: _ } in bags {}
    }

    entries.keys().len() as u32
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

        assert_eq!(count(input, "shiny gold"), 3);
    }
}
