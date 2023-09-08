use crate::{processor::seat::Seat, reader};

pub fn solve() -> u32 {
    let input = reader::read_input();
    let lines = reader::as_lines(&input);
    let mut ids: Vec<_> = lines.into_iter().map(Seat::parse).map(|s| s.id).collect();

    ids.sort();
    for i in 0..ids.len() {
        if let [current, next] = ids[i..i + 2] {
            if next - current == 2 {
                return next + 1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 0);
    }
}
