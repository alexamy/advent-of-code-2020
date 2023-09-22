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

#[derive(Debug, PartialEq)]
pub struct Position(isize, isize);

impl Field {
    pub fn from_map(map: &str) -> Self {
        Self(map.split("\n").map(Self::from_line).collect())
    }

    pub fn get_cell(&self, position: Position) -> Option<&Cell> {
        let Field(vec) = self;
        let Position(x, y) = position;

        vec.get(y as usize).and_then(|row| row.get(x as usize))
    }

    fn from_line(line: &str) -> Vec<Cell> {
        line.chars().map(Cell::from_character).collect()
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
