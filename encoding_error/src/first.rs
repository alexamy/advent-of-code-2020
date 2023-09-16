use core::num;

fn find_breaking(numbers: Vec<u32>, preamble_length: u32) -> Option<u32> {
    for (i, number) in numbers.iter().enumerate() {
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

fn is_contained(numbers: &[u32], target: u32) -> bool {
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
    fn is_found_breaking() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_breaking(numbers, 5), Some(127))
    }
}
