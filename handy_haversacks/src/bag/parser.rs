use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Row<'a> {
    pub color: &'a str,
    pub bags: Vec<Info<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Info<'a> {
    pub color: &'a str,
    pub count: u32,
}

pub fn parse(description: &str) -> Row {
    Row {
        color: parse_source(description),
        bags: parse_bags(description),
    }
}

fn parse_source(description: &str) -> &str {
    let re = Regex::new(r"([\w\s]+) bags contain").unwrap();
    let (_, [color]) = re.captures(description).unwrap().extract();

    color
}

fn parse_bags(description: &str) -> Vec<Info> {
    let re = Regex::new(r"(\d+) ([\w\s]+) bag").unwrap();

    let mut bags = Vec::new();
    for (_, [count, color]) in re.captures_iter(description).map(|c| c.extract()) {
        let count = count.parse().unwrap();
        bags.push(Info { color, count });
    }

    bags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bag() {
        assert_eq!(
            parse("light red bags contain 1 bright yellow bag, 2 muted red bags."),
            Row {
                color: "light red",
                bags: Vec::from([
                    Info {
                        count: 1,
                        color: "bright yellow",
                    },
                    Info {
                        count: 2,
                        color: "muted red",
                    },
                ]),
            }
        );
    }

    #[test]
    fn parses_empty_bag() {
        assert_eq!(
            parse("faded blue bags contain no other bags."),
            Row {
                color: "faded blue",
                bags: Vec::from([]),
            }
        );
    }
}
