enum Direction {
    South,
    North,
    East,
    West,
}

struct Position {
    x: i32,
    y: i32,
}

struct Turtle {
    direction: Direction,
    position: Position,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            direction: Direction::East,
            position: Position { x: 0, y: 0 },
        }
    }
}
