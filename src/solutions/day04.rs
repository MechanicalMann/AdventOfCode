use regex::Regex;
use std::fs;

#[derive(Default)]
struct Passport {
    passport_id: String,
    country_id: String,
    birth_year: String,
    issue_year: String,
    expiry_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
}

const FIELD_MATCHER: &str = r"(pid|cid|byr|iyr|eyr|hgt|hcl|ecl):([\w#]+)";

struct PassportParser<'a, T: Iterator<Item = &'a str>> {
    source: T,
    pattern: Regex,
}

impl<'a, T: Iterator<Item = &'a str>> PassportParser<'a, T> {
    fn new(source: T) -> PassportParser<'a, T> {
        let pattern = Regex::new(FIELD_MATCHER).unwrap();
        PassportParser { source, pattern }
    }
}

trait Validator {
    fn is_valid(&self, passport: &Passport) -> bool;
}

struct SimpleValidator;
impl Validator for SimpleValidator {
    fn is_valid(&self, passport: &Passport) -> bool {
        let required = &[
            &passport.passport_id,
            /*passport.country_id,*/
            &passport.birth_year,
            &passport.issue_year,
            &passport.expiry_year,
            &passport.height,
            &passport.hair_color,
            &passport.eye_color,
        ];
        if required.iter().any(|x| x.is_empty()) {
            return false;
        }
        true
    }
}

struct PassportValidator {
    passport_id: Regex,
    /*country_id: Regex,*/
    birth_year: Regex,
    issue_year: Regex,
    expiry_year: Regex,
    height: Regex,
    hair_color: Regex,
    eye_color: Regex,
}
impl PassportValidator {
    fn new() -> PassportValidator {
        PassportValidator {
            passport_id: Regex::new(r"^\d{9}$").unwrap(),
            birth_year: Regex::new(r"^(19[2-9]\d|200[0-2])$").unwrap(),
            issue_year: Regex::new(r"^201\d|2020$").unwrap(),
            expiry_year: Regex::new(r"^202\d|2030$").unwrap(),
            height: Regex::new(r"^((59|6\d|7[0-6])in|1([5-8]\d|9[0-3])cm)$").unwrap(),
            hair_color: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
            eye_color: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
        }
    }
}
impl Validator for PassportValidator {
    fn is_valid(&self, passport: &Passport) -> bool {
        self.passport_id.is_match(&passport.passport_id)
            && self.birth_year.is_match(&passport.birth_year)
            && self.issue_year.is_match(&passport.issue_year)
            && self.expiry_year.is_match(&passport.expiry_year)
            && self.height.is_match(&passport.height)
            && self.hair_color.is_match(&passport.hair_color)
            && self.eye_color.is_match(&passport.eye_color)
    }
}

impl<'a, T: Iterator<Item = &'a str>> Iterator for PassportParser<'a, T> {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        // let's hand-roll a generator function because generators are "unsafe"
        let mut passport = Passport {
            ..Default::default()
        };
        let mut has_data = false;
        loop {
            let line = match self.source.next() {
                Some(x) => x,
                None => break,
            };
            if line.trim().is_empty() {
                return match has_data {
                    true => Some(passport),
                    false => None,
                };
            }
            if !has_data {
                passport = Passport {
                    ..Default::default()
                };
            }
            for m in self.pattern.captures_iter(line) {
                has_data = true;
                populate_field(&mut passport, &m[1], &m[2]);
            }
        }
        match has_data {
            true => Some(passport),
            false => None,
        }
    }
}

fn populate_field(passport: &mut Passport, field: &str, value: &str) {
    let v = value.to_owned();
    match field {
        "pid" => passport.passport_id = v,
        "cid" => passport.country_id = v,
        "byr" => passport.birth_year = v,
        "iyr" => passport.issue_year = v,
        "eyr" => passport.expiry_year = v,
        "hgt" => passport.height = v,
        "hcl" => passport.hair_color = v,
        "ecl" => passport.eye_color = v,
        _ => (),
    }
}

pub fn part1() {
    let datafile = "data/day4.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let parser = PassportParser::new(read.lines());
    let validator = SimpleValidator;

    let total: usize = parser
        .map(|x| match validator.is_valid(&x) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Answer: {}", total);
}

pub fn part2() {
    let datafile = "data/day4.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let parser = PassportParser::new(read.lines());
    let validator = PassportValidator::new();

    let total: usize = parser
        .map(|x| match validator.is_valid(&x) {
            true => 1,
            false => 0,
        })
        .sum();
    println!("Answer: {}", total);
}
