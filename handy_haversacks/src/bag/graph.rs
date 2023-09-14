use crate::bag::parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Bag<'a> {
    pub color: &'a str,
    pub bags: Vec<Box<Bag<'a>>>,
}

pub fn parse(infos: Vec<&str>) -> Vec<Bag> {
    let bags = Vec::new();
    // let mut hash = HashMap::new();
    let rows = infos.iter().map(|line| parser::parse(line));

    bags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_parsed() {
        let shiny_gold = Bag {
            color: "shiny gold",
            bags: Vec::from([]),
        };

        let bright_white = Bag {
            color: "bright white",
            bags: Vec::from([Box::new(shiny_gold)]),
        };

        let light_red = Bag {
            color: "light red",
            bags: Vec::from([Box::new(bright_white)]),
        };

        assert_eq!(
            parse(Vec::from([
                "light red bags contain 2 bright white bag.",
                "bright white bags contain 1 shiny gold bag."
            ])),
            Vec::from([light_red]),
        )
    }
}
