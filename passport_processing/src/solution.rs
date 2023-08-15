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
    }
}

mod validators {
    pub fn birth_year(input: &str) -> bool {
        let year: u32 = input.parse().unwrap_or(0);

        input.len() == 4 && year >= 1920 && year <= 2002
    }

    pub fn issue_year(input: &str) -> bool {
        let year: u32 = input.parse().unwrap_or(0);

        input.len() == 4 && year >= 2010 && year <= 2020
    }

    pub fn expiration_year(input: &str) -> bool {
        let year: u32 = input.parse().unwrap_or(0);

        input.len() == 4 && year >= 2020 && year <= 2030
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
}
