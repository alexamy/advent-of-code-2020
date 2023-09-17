pub fn find_sum_sequence(numbers: Vec<u64>, target: u64) -> Option<u64> {
    let sequence = find_sequence(numbers, target)?;
    let min = sequence.iter().min()?;
    let max = sequence.iter().max()?;

    Some(min + max)
}

fn find_sequence(numbers: Vec<u64>, target: u64) -> Option<Vec<u64>> {
    let mut result = Vec::new();
    let mut sum = 0;
    for (i, _) in numbers.iter().enumerate() {
        for number in &numbers[i..] {
            result.push(*number);
            sum += number;

            if sum == target {
                return Some(result);
            } else if sum > target {
                break;
            }
        }

        result = Vec::new();
        sum = 0;
    }

    None
}

pub fn find_breaking(numbers: Vec<u64>, preamble_length: u64) -> Option<u64> {
    let checks = &numbers[preamble_length as usize..];

    for (i, number) in checks.iter().enumerate() {
        let end_index = i + preamble_length as usize;
        if end_index > numbers.len() {
            return None;
        }

        let candidates = &numbers[i..end_index];
        let is_proper = is_contained(candidates, *number);
        if !is_proper {
            return Some(*number);
        }
    }

    None
}

fn is_contained(numbers: &[u64], target: u64) -> bool {
    for (ci, c1) in numbers.iter().enumerate() {
        if ci == numbers.len() - 1 {
            return false;
        }

        for c2 in &numbers[ci + 1..] {
            if c1 + c2 == target {
                return true;
            }
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_found_sum_sequnce() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_sum_sequence(numbers, 127), Some(62));
    }

    #[test]
    fn is_found_sequence() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_sequence(numbers, 127), Some(vec![15, 25, 47, 40]))
    }

    #[test]
    fn is_found_breaking() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_breaking(numbers, 5), Some(127))
    }
}
