#[derive(PartialEq, Debug)]
struct Seat {
    pub row: u32,
    pub column: u32,
    pub id: u32,
}

impl Seat {
    pub fn parse(code: &str) -> Self {
        let row = 0;
        let column = 0;
        let id = Self::calculate_id(row, column);

        Seat { row, column, id }
    }

    fn calculate_id(row: u32, column: u32) -> u32 {
        row * 8 + column
    }
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
