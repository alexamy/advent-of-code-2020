#[derive(PartialEq, Debug, Clone)]
pub struct Interval {
    pub min: u32,
    pub max: u32,
}

struct Input<'a> {
    code: &'a str,
    left: char,
    right: char,
}

impl Interval {
    pub fn left(&self) -> Self {
        Interval {
            min: self.min,
            max: self.max / 2,
        }
    }

    pub fn right(&self) -> Self {
        Interval {
            min: self.max / 2 + 1,
            max: self.max,
        }
    }

    pub fn converge(&self, input: &Input) -> u32 {
        let mut interval = self.clone();

        for character in input.code.chars() {
            if character == input.left {
                interval = interval.left();
            } else if character == input.right {
                interval = interval.right();
            }
        }

        interval.converged().expect("Interval must be converged")
    }

    fn converged(&self) -> Result<u32, &str> {
        if self.min == self.max {
            Ok(self.min)
        } else {
            Err("Interval is not converged")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_calculate_left() {
        let interval = Interval { min: 0, max: 127 };
        assert_eq!(interval.left(), Interval { min: 0, max: 63 });
    }

    #[test]
    fn is_calculate_right() {
        let interval = Interval { min: 0, max: 127 };
        assert_eq!(interval.right(), Interval { min: 64, max: 127 });
    }

    #[test]
    fn get_converged_value() {
        let interval = Interval { min: 1, max: 1 };
        assert_eq!(interval.converged(), Ok(1));
    }

    #[test]
    fn converges_row() {
        let interval = Interval { min: 0, max: 127 };
        let input = Input {
            code: "BFFFBBF",
            left: 'F',
            right: 'B',
        };

        assert_eq!(interval.converge(&input), 70)
    }
}