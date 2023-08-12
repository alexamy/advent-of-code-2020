use super::input;

pub fn solve() -> u32 {
    let mut numbers = input::get_data();
    numbers.sort();

    for number in &numbers {
        let candidates = find_candidates(number, &numbers);
        if let None = candidates {
            continue;
        }

        let candidates = candidates.unwrap();
        let target = find_target(number, &candidates);
        if let Some(result) = target {
            return result;
        }
    };

    panic!("No solution found");
}

fn find_candidates(number: &u32, numbers: &Vec<u32>) -> Option<Vec<u32>> {
    let candidates: Vec<u32> = numbers
        .to_vec()
        .iter()
        .map(|n| *n)
        .filter(|n| number + n < 2020)
        .collect();

    if candidates.len() < 2 {
        return None;
    }

    return Some(candidates);
}

fn find_target(number: &u32, candidates: &Vec<u32>) -> Option<u32> {
    for c1 in candidates {
        for c2 in candidates {
            let target = number + c1 + c2;
            if target == 2020 {
                let result = number * c1 * c2;
                return Some(result);
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_solved() {
        assert_eq!(solve(), 85555470);
    }
}
