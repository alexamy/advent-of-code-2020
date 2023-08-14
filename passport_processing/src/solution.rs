use std::collections::HashMap;

pub struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: u32,
    hair_color: String,
    eye_color: String,
    passport_id: u32,
    country_id: Option<u32>,
}

impl Passport {
    pub fn new(entries: HashMap<&str, &str>) -> Option<Self> {
        Some(Passport {
            birth_year: entries.get("byr")?.parse().ok()?,
            issue_year: entries.get("iyr")?.parse().ok()?,
            expiration_year: entries.get("eyr")?.parse().ok()?,
            height: entries.get("hgt")?.parse().ok()?,
            hair_color: entries.get("hcl")?.to_string(),
            eye_color: entries.get("ecl")?.to_string(),
            passport_id: entries.get("pid")?.parse().ok()?,
            country_id: entries.get("cid").map(|v| v.parse().unwrap()),
        })
    }
}
