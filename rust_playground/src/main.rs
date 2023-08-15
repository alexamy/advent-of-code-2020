use std::collections::HashMap;

fn main() {
    let input = get_input();
    let vec = split_whitespace(&input);
    let parts = make_hash(vec);

    println!("{}", input);

    dbg!(parts);
}

fn get_input() -> String {
    String::from("Hello:from here:there")
}

fn split_whitespace(input: &str) -> Vec<&str> {
    input.split(" ").collect()
}

fn make_hash(vec: Vec<&str>) -> HashMap<&str, &str> {
    vec.iter()
        .map(|s| {
            let mut entries = s.split(':');
            let key = entries.next().unwrap();
            let val = entries.next().unwrap();
            (key, val)
        })
        .collect()
}
