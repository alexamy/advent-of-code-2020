#[derive(Debug, PartialEq)]
pub enum Cell {
    Floor,
    Empty,
    Occupied,
}

pub type Field = Vec<Vec<Cell>>;

pub fn convert_field(field: &str) -> Field {
    field
        .split("\n")
        .map(|line| line.chars().map(get_cell_type).collect())
        .collect()
}

fn get_cell_type(character: char) -> Cell {
    match character {
        '.' => Cell::Floor,
        'L' => Cell::Empty,
        '#' => Cell::Occupied,
        c => panic!("Unknown cell character: {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_map() {
        assert_eq!(
            convert_field(".L#"),
            vec![vec![Cell::Floor, Cell::Empty, Cell::Occupied]],
        );
    }
}
