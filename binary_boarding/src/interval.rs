#[derive(PartialEq, Debug)]
struct Interval {
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
}
