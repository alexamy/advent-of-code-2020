#[derive(PartialEq, Debug)]
pub struct Interval {
    pub min: u32,
    pub max: u32,
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

    pub fn converged(&self) -> Result<u32, &str> {
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
}
