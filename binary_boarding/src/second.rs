use crate::{processor::seat::Seat, reader};

pub fn solve() -> u32 {
    let input = reader::read_input();
    let lines = reader::as_lines(&input);
    let mut ids: Vec<_> = lines.into_iter().map(Seat::parse).map(|s| s.id).collect();

    ids.sort();

    for i in 0..ids.len() {
        if let [current, next] = ids[i..i + 2] {
            let target = current + 1;
            if target == next {
                continue;
            }

            return target;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 699);
    }
}
