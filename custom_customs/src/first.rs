use crate::reader;

pub fn solve() -> u32 {
    0
}

fn sum_counts(input: &str) -> u32 {
    let lines = reader::as_lines(input);

    let mut count = 0;
    let mut acc: Vec<&str> = Vec::new();

    for line in lines {}

    count
}

fn sum_characters(input: &str) -> u32 {
    let mut result = String::new();

    for line in input.lines() {
        result += line;
    }

    let mut characters: Vec<_> = result.chars().collect();

    characters.sort();
    characters.dedup();

    characters.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 0);
    }

    #[test]
    fn sums_counts() {
        let input = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

        assert_eq!(sum_counts(input), 11);
    }

    #[test]
    fn sums_characters() {
        let input = "\
ab
ac
ab
";

        assert_eq!(sum_characters(input), 3);
    }
}
