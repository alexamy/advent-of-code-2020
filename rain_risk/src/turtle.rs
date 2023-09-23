#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate(&self, angle: i32) -> Self {
        let amount = angle / 90;
        let mut direction = self.clone();

        for _ in 0..amount {
            direction = direction.turn_clockwise();
        }

        direction
    }

    fn turn_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn as_directive(&self, amount: i32) -> Directive {
        match self {
            Direction::East => Directive::East(amount),
            Direction::West => Directive::West(amount),
            Direction::North => Directive::North(amount),
            Direction::South => Directive::South(amount),
        }
    }
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub enum Directive {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl Directive {
    pub fn from_string(line: &str) -> Self {
        let name = &line[0..1];
        let directive = match name {
            "N" => Directive::North,
            "E" => Directive::East,
            "S" => Directive::South,
            "W" => Directive::West,
            "R" => Directive::Right,
            "L" => Directive::Left,
            "F" => Directive::Forward,
            d => panic!("Unknown directive: {}", d),
        };

        let amount = &line[1..].parse::<i32>().expect("Expect numerical amount");

        directive(*amount)
    }
}

pub struct Turtle {
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

    pub fn go(&mut self, directive: Directive) {
        match directive {
            Directive::South(n) => self.position.y += n,
            Directive::North(n) => self.position.y -= n,
            Directive::East(n) => self.position.x += n,
            Directive::West(n) => self.position.x -= n,
            Directive::Right(n) => self.rotate(n),
            Directive::Left(n) => self.rotate(360 - n),
            Directive::Forward(n) => {
                let forward = self.direction.as_directive(n);
                self.go(forward);
            }
        }
    }

    pub fn distance(&self) -> i32 {
        self.position.distance()
    }

    fn rotate(&mut self, angle: i32) {
        self.direction = self.direction.rotate(angle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_at_destination() {
        let mut turtle = Turtle::new();
        let directives = vec![
            Directive::Forward(10),
            Directive::North(3),
            Directive::Forward(7),
            Directive::Right(90),
            Directive::Forward(11),
        ];

        for directive in directives {
            turtle.go(directive);
        }

        assert_eq!(turtle.distance(), 25);
    }
}
