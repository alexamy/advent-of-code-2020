use std::collections::HashMap;

fn main() {
    let input = get_input();
    let vec = split_whitespace(&input);
    let parts = make_hash(vec);

    println!("{}", input);
    dbg!(parts);

    let strs = vec![String::from("gh:1"), String::from("xs:2")];
    let refs = get_splits(&strs);
}

fn get_splits(strs: &Vec<String>) -> Vec<Vec<&str>> {
    strs.iter().map(|s| split_whitespace(s)).collect()
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

// String -> ref
fn test_str_map() {
    let inputs = vec![String::from("1"), String::from("2")];

    let inputs = inputs.iter().map(str_identity);
}

fn str_identity(input: &str) -> &str {
    input
}
