pub fn count_trees(map: &Vec<String>, x_offset: usize) -> usize {
    let tree = '#';

    let mut count = 0;
    let mut x = 0;

    for row in map {
        if let Some(cell) = row.chars().nth(x) {
            if cell == tree {
                count += 1;
            }
        }

        x = (x + x_offset) % row.len();
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

        assert_eq!(count_trees(&map, 3), 7);
    }
}
