use std::collections::HashMap;

fn main() {
    let input = get_input();
    let vec = split_whitespace(input);
    let parts = make_hash(vec);

    dbg!(parts);
}

fn get_input() -> String {
    String::from("Hello:from here:there")
}

fn split_whitespace(input: String) -> Vec<String> {
    input.split(" ").map(str::to_string).collect()
}

fn make_hash(vec: Vec<String>) -> HashMap<String, String> {
    vec.iter()
        .map(|s| {
            let mut entries = s.split(':').map(str::to_string);
            let key = entries.next().unwrap();
            let val = entries.next().unwrap();
            (key, val)
        })
        .collect()
}
