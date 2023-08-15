use crate::{processor, reader, solution::Passport};

pub fn solve() -> u32 {
    let input = reader::read_input();
    let passports = processor::split_passports(&input);

    let entries: Vec<_> = passports
        .iter()
        .map(|p| processor::split_entries(p))
        .map(Passport::new)
        .filter(Option::is_some)
        .collect();

    entries.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 245);
    }
}
