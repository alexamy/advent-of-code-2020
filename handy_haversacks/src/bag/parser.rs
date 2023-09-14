use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Row<'a> {
    pub count: u32,
    pub color: &'a str,
    pub bags: Vec<Box<Row<'a>>>,
}

pub fn parse(description: &str) -> Row {
    Row {
        count: 1,
        color: parse_color(description),
        bags: parse_bags(description),
    }
}

fn parse_color(description: &str) -> &str {
    let re = Regex::new(r"([\w\s]+) bags contain").unwrap();
    let (_, [color]) = re.captures(description).unwrap().extract();

    color
}

fn parse_bags(description: &str) -> Vec<Box<Row>> {
    let re = Regex::new(r"(\d+) ([\w\s]+) bag").unwrap();

    let mut bags = Vec::new();
    for (_, [count, color]) in re.captures_iter(description).map(|c| c.extract()) {
        let count = count.parse().unwrap();
        bags.push(Box::new(Row {
            color,
            count,
            bags: Vec::from([]),
        }));
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
                count: 1,
                color: "light red",
                bags: Vec::from([
                    Box::new(Row {
                        count: 1,
                        color: "bright yellow",
                        bags: Vec::new(),
                    }),
                    Box::new(Row {
                        count: 2,
                        color: "muted red",
                        bags: Vec::new(),
                    }),
                ]),
            }
        );
    }

    #[test]
    fn parses_empty_bag() {
        assert_eq!(
            parse("faded blue bags contain no other bags."),
            Row {
                count: 1,
                color: "faded blue",
                bags: Vec::new(),
            }
        );
    }
}
