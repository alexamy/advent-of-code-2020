use crate::reader;

pub fn solve() {
    let input = reader::read_input();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 245);
    }
}
