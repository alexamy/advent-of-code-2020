use super::interval::{Input, Interval};

pub fn find_row(code: &str) -> u32 {
    let input = Input {
        code,
        left: 'F',
        right: 'B',
    };

    let interval = Interval { min: 0, max: 127 };

    interval.converge(&input)
}

pub fn find_column(code: &str) -> u32 {
    let input = Input {
        code,
        left: 'L',
        right: 'R',
    };

    let interval = Interval { min: 0, max: 7 };

    interval.converge(&input)
}
