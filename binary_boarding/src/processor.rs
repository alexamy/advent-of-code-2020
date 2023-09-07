#[derive(PartialEq, Debug)]
struct Seat {
    pub row: u32,
    pub column: u32,
    pub id: u32,
}

impl Seat {
    fn parse(code: &str) -> Self {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_values() {
        assert_eq!(
            Seat::parse("BFFFBBFRRR"),
            Seat {
                row: 70,
                column: 7,
                id: 567,
            },
        );

        assert_eq!(
            Seat::parse("FFFBBBFRRR"),
            Seat {
                row: 14,
                column: 7,
                id: 119,
            },
        );

        assert_eq!(
            Seat::parse("BBFFBBFRLL"),
            Seat {
                row: 102,
                column: 4,
                id: 820,
            },
        );
    }
}
