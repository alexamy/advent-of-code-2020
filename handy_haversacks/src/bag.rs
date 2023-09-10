#[derive(Debug, PartialEq)]
struct Bag {
    pub color: String,
    pub bags: Vec<BagInfo>,
}

#[derive(Debug, PartialEq)]
struct BagInfo {
    pub color: String,
    pub count: u32,
}

impl Bag {
    pub fn parse(description: &str) -> Bag {
        Bag {
            color: String::from(""),
            bags: Vec::from([]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bag() {
        assert_eq!(
            Bag::parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Bag {
                color: String::from("light red"),
                bags: Vec::from([
                    BagInfo {
                        count: 1,
                        color: String::from("bright white")
                    },
                    BagInfo {
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
            Bag::parse("faded blue bags contain no other bags."),
            Bag {
                color: String::from("faded blue"),
                bags: Vec::from([]),
            }
        );
    }
}
