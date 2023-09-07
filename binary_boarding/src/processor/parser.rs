use super::interval::Interval;

struct Input<'a> {
    code: &'a str,
    left: &'a char,
    right: &'a char,
}

impl Input<'_> {
    pub fn converge(&self, interval: &Interval) -> Interval {
        let mut interval = interval.clone();

        for character in self.code.chars() {
            if character == *self.left {
                interval = interval.left();
            } else if character == *self.right {
                interval = interval.right();
            }
        }

        interval
    }
}
