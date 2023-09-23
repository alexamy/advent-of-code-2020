use crate::{
    reader,
    turtle::{Directive, Turtle},
};

pub fn solve() -> i32 {
    let input = reader::read_input();
    let directives = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(Directive::from_string);

    let mut turtle = Turtle::new();
    for directive in directives {
        turtle.go(directive);
    }

    turtle.distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1294);
    }
}
