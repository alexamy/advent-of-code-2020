use crate::{processor, reader, solution::Passport};

pub fn solve() -> u32 {
    let input = reader::read_input();
    let entries = processor::split(&input);

    let passports: Vec<_> = entries
        .into_iter()
        .map(|entry| Passport::new(entry.entries))
        .filter(|p| p.is_some())
        .collect();

    passports.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 1);
    }
}
