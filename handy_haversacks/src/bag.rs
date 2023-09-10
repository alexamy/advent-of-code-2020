#[derive(Debug, PartialEq)]
struct Bag {
    pub color: String,
    pub bags: Vec<String>,
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
                bags: Vec::from([String::from("bright white"), String::from("muted yellow"),]),
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
