use std::collections::HashMap;

pub fn split_entries(row: &str) -> HashMap<&str, &str> {
    let mut result = HashMap::new();

    for part in row.split(" ").collect::<Vec<_>>() {
        let data: Vec<_> = part.split(":").collect();

        result.entry(data[0]).or_insert(data[1]);
    }

    result
}

pub fn split_passports(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| s.trim().replace("\n", " "))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting_entries() {
        let input = "ecl:gry pid:860 byr:1937";
        let entities = HashMap::from([("ecl", "gry"), ("pid", "860"), ("byr", "1937")]);

        assert_eq!(split_entries(&input), entities);
    }

    #[test]
    fn splitting_passports() {
        let input = "\
ecl:gry pid:860
byr:1937

iyr:2013
";

        let entities = vec![
            String::from("ecl:gry pid:860 byr:1937"),
            String::from("iyr:2013"),
        ];

        assert_eq!(split_passports(&input), entities);
    }
}
