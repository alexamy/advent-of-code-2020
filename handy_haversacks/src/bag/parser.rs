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
        let count_str = parts.get(0).expect("Must have count");
        let count: u32 = count_str.parse().expect("Must be count");

        let shade = parts.get(1).expect("Must have shade");
        let base = parts.get(2).expect("Must have base");

        let start = count_str.len() + 1;
        let end = shade.len() + base.len() + 3;
        let color = &color[start..end];

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
