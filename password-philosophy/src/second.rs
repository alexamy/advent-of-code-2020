use super::input;

trait Check {
    fn check(&self, password: &str) -> bool;
}

impl Check for input::Rule {
    fn check(&self, password: &str) -> bool {
        let chars: Vec<char> = password.chars().collect();

        let first = chars[self.low_bound - 1] == self.letter;
        let second = chars[self.high_bound - 1] == self.letter;

        first ^ second
    }
}

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
        assert_eq!(solve(), 313);
    }

    #[test]
    fn rule_checks_password() {
        let rule = input::Rule {
            low_bound: 2,
            high_bound: 4,
            letter: 'p',
        };

        assert_eq!(rule.check("apwwx"), true);
        assert_eq!(rule.check("awwppx"), true);

        assert_eq!(rule.check("axpx"), false);
        assert_eq!(rule.check("axxx"), false);
        assert_eq!(rule.check("pxxx"), false);
    }
}
