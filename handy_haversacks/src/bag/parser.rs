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
    let parts: Vec<_> = description.split(" bags contain ").collect();
    let color = *parts.get(0).expect("Must have color");
    let parts = *parts.get(1).expect("Must have parts");

    let is_none = parts.starts_with("no");
    if is_none {
        return Row {
            color,
            bags: Vec::new(),
        };
    }

    let mut bags = Vec::new();
    for color in parts.split(", ") {
        let parts: Vec<_> = color.split(" ").collect();
        let count: u32 = parts
            .get(0)
            .expect("Must have count")
            .parse()
            .expect("Must be count");

        let color = "black";

        bags.push(Info { count, color })
    }

    Row { color, bags }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bag() {
        assert_eq!(
            parse("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            Row {
                color: "light red",
                bags: Vec::from([
                    Info {
                        count: 1,
                        color: "bright white",
                    },
                    Info {
                        count: 2,
                        color: "muted yellow",
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
