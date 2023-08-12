use super::input;

pub fn solve() -> usize {
    let rows = input::get_data();

    let count = rows
        .iter()
        .filter(|(password, rule)| rule.check(password))
        .collect::<Vec<&input::Row>>()
        .len();

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 500);
    }
}
