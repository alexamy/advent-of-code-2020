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

enum Directive {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            direction: Direction::East,
            position: Position { x: 0, y: 0 },
        }
    }

    pub fn go(&self, directive: Directive) {
        match directive {
            Directive::South(n) => self.position.y += n,
            Directive::North(n) => self.position.y -= n,
            Directive::East(n) => self.position.x += n,
            Directive::West(n) => self.position.x -= n,
        }
    }
}
