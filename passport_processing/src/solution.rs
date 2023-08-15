use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

impl Passport {
    pub fn new(entries: HashMap<&str, &str>) -> Option<Self> {
        Some(Passport {
            birth_year: entries.get("byr")?.to_string(),
            issue_year: entries.get("iyr")?.to_string(),
            expiration_year: entries.get("eyr")?.to_string(),
            height: entries.get("hgt")?.to_string(),
            hair_color: entries.get("hcl")?.to_string(),
            eye_color: entries.get("ecl")?.to_string(),
            passport_id: entries.get("pid")?.to_string(),
            country_id: entries.get("cid").map(|v| v.to_string()),
        })
    }

    pub fn is_valid(&self) -> bool {
        validators::birth_year(&self.birth_year)
            && validators::issue_year(&self.issue_year)
            && validators::expiration_year(&self.expiration_year)
            && validators::hair_color(&self.hair_color)
            && validators::eye_color(&self.eye_color)
    }
}

mod validators {
    pub fn birth_year(input: &str) -> bool {
        year(input, 1920, 2002)
    }

    pub fn issue_year(input: &str) -> bool {
        year(input, 2010, 2020)
    }

    pub fn expiration_year(input: &str) -> bool {
        year(input, 2020, 2030)
    }

    fn year(input: &str, low: u32, high: u32) -> bool {
        input
            .parse::<u32>()
            .map(|year| year >= low && year <= high)
            .unwrap_or(false)
    }

    pub fn hair_color(input: &str) -> bool {
        const CHARS: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        if input.len() != 7 {
            return false;
        }

        let mut chars = input.chars();
        let start = chars.next().filter(|s| *s == '#');
        if start.is_none() {
            return false;
        }

        let is_not_hex = chars.any(|chr| !CHARS.contains(&chr));
        if is_not_hex {
            return false;
        }

        true
    }

    pub fn eye_color(input: &str) -> bool {
        const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        COLORS.contains(&input)
    }

    pub fn passport_id(input: &str) -> bool {
        const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        if input.len() != 9 {
            return false;
        }

        !input.chars().any(|chr| !DIGITS.contains(&chr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validating_birth_year() {
        assert_eq!(validators::birth_year("2000"), true);

        assert_eq!(validators::birth_year("xxx"), false);
        assert_eq!(validators::birth_year("100"), false);
        assert_eq!(validators::birth_year("1919"), false);
        assert_eq!(validators::birth_year("2003"), false);
    }

    #[test]
    fn validating_issue_year() {
        assert_eq!(validators::issue_year("2015"), true);

        assert_eq!(validators::issue_year("xxx"), false);
        assert_eq!(validators::issue_year("100"), false);
        assert_eq!(validators::issue_year("2009"), false);
        assert_eq!(validators::issue_year("2021"), false);
    }

    #[test]
    fn validating_expiration_year() {
        assert_eq!(validators::expiration_year("2025"), true);

        assert_eq!(validators::expiration_year("xxx"), false);
        assert_eq!(validators::expiration_year("100"), false);
        assert_eq!(validators::expiration_year("2019"), false);
        assert_eq!(validators::expiration_year("2031"), false);
    }

    #[test]
    fn validating_hair_color() {
        assert_eq!(validators::hair_color("#ff0011"), true);

        assert_eq!(validators::hair_color("#FF0011"), false);
        assert_eq!(validators::hair_color("#ff"), false);
        assert_eq!(validators::hair_color("ff0011"), false);
        assert_eq!(validators::hair_color("~ff0011"), false);
        assert_eq!(validators::hair_color("zx"), false);
    }

    #[test]
    fn validating_eye_color() {
        assert_eq!(validators::eye_color("amb"), true);
        assert_eq!(validators::eye_color("blu"), true);
        assert_eq!(validators::eye_color("oth"), true);

        assert_eq!(validators::eye_color("othx"), false);
        assert_eq!(validators::eye_color("orange"), false);
    }

    #[test]
    fn validating_passport_id() {
        assert_eq!(validators::passport_id("000111222"), true);
        assert_eq!(validators::passport_id("333111222"), true);

        assert_eq!(validators::passport_id("33311122"), false);
        assert_eq!(validators::passport_id("aaabbbccc"), false);
    }
}
