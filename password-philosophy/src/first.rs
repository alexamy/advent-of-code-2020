use super::input;

trait Check {
    fn check(&self, password: &str) -> bool;
}

impl Check for input::Rule {
    fn check(&self, password: &str) -> bool {
      let count = password
        .chars()
        .filter(|letter| *letter == self.letter)
        .collect::<Vec<char>>()
        .len();

      count >= self.low_bound && count <= self.high_bound
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
        assert_eq!(solve(), 500);
    }

    #[test]
    fn rule_allows_password() {
        let rule = input::Rule {
        low_bound: 2,
        high_bound: 4,
        letter: 'p',
        };

        assert_eq!(rule.check("appx"), true);
        assert_eq!(rule.check("apppx"), true);
        assert_eq!(rule.check("appppx"), true);
    }

    #[test]
    fn rule_disallows_password() {
        let rule = input::Rule {
        low_bound: 2,
        high_bound: 4,
        letter: 'p',
        };

        assert_eq!(rule.check("ax"), false);
        assert_eq!(rule.check("apx"), false);
        assert_eq!(rule.check("apppppx"), false);
    }
}
