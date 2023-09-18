#[derive(Debug, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

type Field = Vec<Vec<Cell>>;
struct Position(usize, usize);

pub fn count_seats(map: &str) -> u32 {
    let map = convert_map(map);

    0
}

fn next_state(field: Field, position: Position) -> Cell {
    let Position(x, y) = position;
    let cell = get_field_cell(&field, position);

    if let Some(Cell::Floor) = cell {
        return Cell::Floor;
    }

    Cell::Empty
}

fn get_field_cell(field: &Field, position: Position) -> Option<&Cell> {
    let Position(x, y) = position;

    field.get(y).and_then(|row| row.get(x))
}

fn convert_map(map: &str) -> Field {
    map.split("\n")
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
            convert_map(".L#"),
            vec![vec![Cell::Floor, Cell::Empty, Cell::Occupied]],
        );
    }

    #[test]
    fn seats_are_counted() {
        let input = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(count_seats(input), 37);
    }
}
