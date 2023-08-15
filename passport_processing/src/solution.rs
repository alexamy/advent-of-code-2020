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
}
