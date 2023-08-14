use std::collections::HashMap;

fn split_entities() -> Vec<HashMap<String, String>> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_hashes() {
        let input = "
ecl:gry pid:860
byr:1937

iyr:2013
";

        let entities = vec![
            HashMap::from([
                (String::from("ecl"), String::from("gry")),
                (String::from("pid"), String::from("860")),
                (String::from("byr"), String::from("1937")),
            ]),
            HashMap::from([(String::from("iyr"), String::from("2013"))]),
        ];

        assert_eq!(split_entities(input), entities);
    }
}
