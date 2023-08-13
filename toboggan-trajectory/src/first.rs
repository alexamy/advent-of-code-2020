use crate::reader;

pub fn solve() -> usize {
    let map = reader::read_lines();

    count_trees(&map)
}

fn count_trees(map: &Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::first::count_trees;

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

        assert_eq!(count_trees(&map), 7);
    }
}
