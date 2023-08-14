use std::collections::HashMap;

fn split_passports(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .into_iter()
        .map(|s| s.replace("\n", " ").trim().to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(split_passports(input), entities);
    }
}
