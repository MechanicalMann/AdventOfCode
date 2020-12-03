use regex::Regex;
use std::convert::TryInto;
use std::fs;

struct Password {
    required: char,
    min_req: u8,
    max_req: u8,
    password: String,
}

impl Password {
    fn is_valid<F>(&self, validator: F) -> bool
    where
        F: Fn(char, u8, u8, &str) -> bool,
    {
        return validator(self.required, self.min_req, self.max_req, &self.password);
    }
}

fn range_validator(c: char, min: u8, max: u8, s: &str) -> bool {
    let num: u8 = s.chars().filter(|x| *x == c).count().try_into().unwrap();
    return num >= min && num <= max;
}

fn pos_validator(c: char, left: u8, right: u8, s: &str) -> bool {
    let mut chars = s.chars();
    let has_left = match chars.nth((left - 1).into()) {
        Some(x) => x == c,
        None => false,
    };
    let has_right = match chars.nth((right - left - 1).into()) {
        Some(x) => x == c,
        None => false,
    };
    return has_left ^ has_right;
}

fn parse_password(input: &str) -> Password {
    let rm = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.+)$").unwrap();
    if !rm.is_match(input) {
        panic!("Invalid input: {}", input);
    }
    let cap = rm.captures_iter(input).next().unwrap();
    Password {
        min_req: cap[1].parse().unwrap(),
        max_req: cap[2].parse().unwrap(),
        required: cap[3].chars().next().unwrap(),
        password: cap[4].to_string(),
    }
}

pub fn part1() {
    let datafile = "data/day2.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let lines = read.lines();

    let num_valid = lines
        .map(|x| parse_password(x))
        .filter(|y| y.is_valid(&range_validator))
        .count();

    println!("Answer: {}", num_valid);
}

pub fn part2() {
    let datafile = "data/day2.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let lines = read.lines();

    let num_valid = lines
        .map(|x| parse_password(x))
        .filter(|y| y.is_valid(&pos_validator))
        .count();

    println!("Answer: {}", num_valid);
}
