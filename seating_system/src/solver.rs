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

fn next_state(field: Field) -> Field {
    let mut result = Vec::new();

    for (y, row) in field.iter().enumerate() {
        let mut next_row = Vec::new();

        for (x, _) in row.iter().enumerate() {
            let next_state = next_state_for_cell(&field, Position(x, y));
            next_row.push(next_state);
        }

        result.push(next_row);
    }

    result
}

fn next_state_for_cell(field: &Field, position: Position) -> Cell {
    let Position(x, y) = position;
    let cell = get_field_cell(field, position);

    if let Some(Cell::Floor) = cell {
        return Cell::Floor;
    }

    let neighbours = vec![
        get_field_cell(field, Position(x - 1, y - 1)),
        get_field_cell(field, Position(x + 1, y + 1)),
        get_field_cell(field, Position(x + 1, y - 1)),
        get_field_cell(field, Position(x - 1, y + 1)),
        get_field_cell(field, Position(x, y - 1)),
        get_field_cell(field, Position(x, y + 1)),
        get_field_cell(field, Position(x - 1, y)),
        get_field_cell(field, Position(x + 1, y)),
    ];

    let occupied_count = neighbours
        .into_iter()
        .filter(|el| el == &Some(&Cell::Occupied))
        .count();

    let cell_state = match cell {
        Some(Cell::Empty) => {
            if occupied_count == 0 {
                Cell::Occupied
            } else {
                Cell::Empty
            }
        }
        Some(Cell::Occupied) => {
            if occupied_count >= 4 {
                Cell::Empty
            } else {
                Cell::Occupied
            }
        }
        state => panic!("Unexpected state cell: {:?}", state),
    };

    cell_state
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
