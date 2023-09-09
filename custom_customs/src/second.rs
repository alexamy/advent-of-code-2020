use crate::reader;

pub fn solve() -> u32 {
    let input = reader::read_input();

    sum_counts(&input)
}

fn sum_counts(input: &str) -> u32 {
    input.split("\n\n").map(sum_characters).sum()
}

fn sum_characters(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let mut result = lines[0].clone();
    for line in &lines[1..] {
        result = result.into_iter().filter(|ch| line.contains(ch)).collect();
    }

    result.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 3229);
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

        assert_eq!(sum_counts(input), 6);
    }

    #[test]
    fn sums_characters() {
        let input = "\
ab
ac
ab";

        assert_eq!(sum_characters(&input), 1);
    }
}
