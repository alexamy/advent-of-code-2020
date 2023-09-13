#[derive(Debug, PartialEq)]
pub struct Bag<'a> {
    pub count: u32,
    pub color: &'a str,
    pub bags: Vec<Box<Bag<'a>>>,
}

pub fn parse(infos: Vec<&str>) -> Bag {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_parsed() {
        assert_eq!(
            parse(Vec::from([
                "light red bags contain 2 bright white bag.",
                "bright white bags contain 1 shiny gold bag."
            ])),
            Bag {
                count: 1,
                color: "light red",
                bags: Vec::from([Box::new(Bag {
                    count: 2,
                    color: "bright white",
                    bags: Vec::from([Box::new(Bag {
                        count: 1,
                        color: "shiny gold",
                        bags: Vec::new(),
                    })]),
                })]),
            }
        )
    }
}
