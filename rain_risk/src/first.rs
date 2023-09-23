use crate::{
    input::Input,
    turtle::{Directive, Turtle},
};

pub fn solve() -> i32 {
    let input = Input::read();
    let directives = input.lines().map(Directive::from_string);

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
