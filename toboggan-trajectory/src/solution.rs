pub struct Offset {
    pub x: usize,
    pub y: usize,
}

pub fn count_trees(map: &Vec<String>, offset: Offset) -> usize {
    let tree = '#';

    let mut count = 0;
    let mut x = 0;

    for row in map {
        if let Some(cell) = row.chars().nth(x) {
            if cell == tree {
                count += 1;
            }
        }

        x = (x + offset.x) % row.len();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_trees() {
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

        let offset = Offset { x: 3, y: 1 };
        assert_eq!(count_trees(&map, offset), 7);
    }
}
