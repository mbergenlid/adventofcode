use regex::Regex;
use std::collections::HashMap;

pub fn solve_part_1(_: &str) -> usize {
    include_str!("../inputs/prob4")
        .split("\n\n")
        .map(|l| Passport::parse(l))
        .filter(|p| p.has_all_required_fields())
        .count()
}

pub fn solve_part_2(_: &str) -> usize {
    include_str!("../inputs/prob4")
        .split("\n\n")
        .map(|l| Passport::parse(l))
        .filter(|p| p.is_valid())
        .count()
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\w+):([^\s]+)").unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> Passport<'a> {
    fn parse(s: &'a str) -> Passport<'a> {
        Passport(
            REGEX
                .captures_iter(s)
                .map(|c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()))
                .collect(),
        )
    }

    fn has_all_required_fields(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .all(|&field| self.0.contains_key(field))
    }

    fn is_valid(&self) -> bool {
        self.has_all_required_fields()
            && self.0["byr"]
                .parse::<u32>()
                .map_or(false, |year| year >= 1920 && year <= 2002)
            && self.0["iyr"]
                .parse::<u32>()
                .map_or(false, |year| year >= 2010 && year <= 2020)
            && self.0["eyr"]
                .parse::<u32>()
                .map_or(false, |year| year >= 2020 && year <= 2030)
            && {
                let height = self.0["hgt"];
                let height_value = height[0..height.len() - 2].parse::<u32>();
                match &height[height.len() - 2..] {
                    "in" => height_value.map_or(false, |h| h >= 59 && h <= 76),
                    "cm" => height_value.map_or(false, |h| h >= 150 && h <= 193),
                    _ => false,
                }
            }
            && HCL_REGEX.is_match(self.0["hcl"])
            && ECL_REGEX.is_match(self.0["ecl"])
            && PID_REGEX.is_match(self.0["pid"])
    }
}

#[cfg(test)]
mod test {
    use crate::prob4::Passport;

    #[test]
    fn test_parse() {
        let passport = Passport::parse(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm",
        );

        println!("{:?}", passport.0);
        assert!(passport.is_valid());
    }
}
