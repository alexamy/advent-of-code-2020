pub struct Slope {
    pub x: u8,
    pub y: u8,
}

pub fn count_trees(map: &Vec<String>, slope: Slope) -> usize {
    let tree = '#';

    let mut count = 0;
    let mut x = 0;

    for row in map {
        if let Some(cell) = row.chars().nth(x) {
            if cell == tree {
                count += 1;
            }
        }

        x = (x + slope.x as usize) % row.len();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_trees_with_y_1() {
        let map = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];

        let slope = Slope { x: 3, y: 1 };
        assert_eq!(count_trees(&map, slope), 7);
    }

    #[test]
    fn counting_trees_with_y_2() {
        let map = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
        ];

        let slope = Slope { x: 1, y: 2 };
        assert_eq!(count_trees(&map, slope), 1);
    }
}
