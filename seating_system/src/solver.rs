use crate::field::{self, Cell, Field};

pub fn count_seats(map: &str) -> u32 {
    let mut current_field = field::convert_field(map);
    let mut next_field = current_field;

    loop {
        current_field = next_field;
        next_field = next_state(&current_field);

        if current_field == next_field {
            break;
        }
    }

    let occupied_seats = current_field
        .into_iter()
        .flatten()
        .filter(|cell| cell == &Cell::Occupied)
        .count();

    occupied_seats as u32
}

fn next_state(field: &Field) -> Field {
    let mut result = Vec::new();

    for (y, row) in field.iter().enumerate() {
        let mut next_row = Vec::new();

        for (x, _) in row.iter().enumerate() {
            let next_state = next_state_for_cell(&field, Position(x as isize, y as isize));
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

#[cfg(test)]
mod tests {
    use super::*;

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
