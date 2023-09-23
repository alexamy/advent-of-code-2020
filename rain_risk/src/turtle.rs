enum Direction {
    North,
    East,
    South,
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
    East(i32),
    South(i32),
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

    pub fn go(&mut self, directive: Directive) {
        match directive {
            Directive::South(n) => self.position.y += n,
            Directive::North(n) => self.position.y -= n,
            Directive::East(n) => self.position.x += n,
            Directive::West(n) => self.position.x -= n,
            Directive::Right(n) => self.rotate(n),
            Directive::Left(n) => self.rotate(360 - n),
            Directive::Forward(n) => {
                let forward = Self::get_forward_directive(&self.direction, n);
                self.go(forward);
            }
        }
    }

    fn get_forward_directive(direction: &Direction, amount: i32) -> Directive {
        match direction {
            Direction::East => Directive::East(amount),
            Direction::West => Directive::West(amount),
            Direction::North => Directive::North(amount),
            Direction::South => Directive::South(amount),
        }
    }

    fn rotate(&mut self, angle: i32) {
        let amount = angle / 90;
        for _ in 0..amount {
            self.direction = Self::turn_clockwise(&self.direction);
        }
    }

    fn turn_clockwise(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
