use crate::{processor::seat::Seat, reader};

pub fn solve() -> u32 {
    let input = reader::read_input();
    let lines = reader::as_lines(&input);
    let max_id = lines.into_iter().map(Seat::parse).map(|s| s.id).max();

    max_id.expect("Expect max value to be found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 915);
    }
}
