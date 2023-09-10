#[derive(Debug, PartialEq)]
struct Row {
    pub color: String,
    pub bags: Vec<Info>,
}

#[derive(Debug, PartialEq)]
struct Info {
    pub color: String,
    pub count: u32,
}

pub fn parse(description: &str) -> Row {
    Row {
        color: String::from(""),
        bags: Vec::from([]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bag() {
        assert_eq!(
            parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Row {
                color: String::from("light red"),
                bags: Vec::from([
                    Info {
                        count: 1,
                        color: String::from("bright white")
                    },
                    Info {
                        count: 2,
                        color: String::from("muted yellow")
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
                color: String::from("faded blue"),
                bags: Vec::from([]),
            }
        );
    }
}
