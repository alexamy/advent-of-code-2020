#[derive(Debug, PartialEq)]
pub enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Cell {
    pub fn from_character(character: char) -> Self {
        match character {
            '.' => Cell::Floor,
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            c => panic!("Unknown cell character: {}", c),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Field(Vec<Vec<Cell>>);

impl Field {
    pub fn from_map(map: &str) -> Self {
        Self(
            map.split("\n")
                .map(|line| line.chars().map(Cell::from_character).collect())
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_map() {
        assert_eq!(
            Field::from_map(".L#"),
            Field(vec![vec![Cell::Floor, Cell::Empty, Cell::Occupied]]),
        );
    }
}
