use crate::solver::Solver;
use anyhow::{anyhow, Ok, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use pcre2::bytes::Regex;
use phf_macros::phf_map;

pub struct Solution;
impl Solver<u32, u32> for Solution {
    const DAY: u8 = 1;
    const TITLE: &'static str = "Trebuchet?!";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<u32> {
        let input = self.input().get()?;
        get_total(&input, false)
    }

    fn part_two(&self) -> Result<u32> {
        let input = self.input().get()?;
        get_total(&input, true)
    }
}

fn get_calibration_value(line: &str) -> Result<u32> {
    let chars = line.chars().collect_vec();
    let first = line.find(|c: char| c.is_digit(10));
    let last = line.rfind(|c: char| c.is_digit(10));

    if first == None || last == None {
        return Err(anyhow!("Could not find any digits!"));
    }

    let calibration = chars[first.unwrap()].to_digit(10).unwrap() * 10
        + chars[last.unwrap()].to_digit(10).unwrap();

    Ok(calibration)
}

static DIGITS: phf::Map<&'static str, u32> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
};
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?=(\d|zero|one|two|three|four|five|six|seven|eight|nine))").unwrap();
}
fn get_stringy_calibration(line: &str) -> Result<u32> {
    let digits = RE
        .captures_iter(line.as_bytes())
        .map(|c| {
            let m = c.unwrap();
            let v = std::str::from_utf8(&m[1]).unwrap();
            DIGITS[&v]
        })
        .collect_vec();
    let val = digits[0] * 10 + *digits.last().unwrap();
    Ok(val)
}

fn get_total(input: &str, stringy: bool) -> Result<u32> {
    // Yeah, this could be a one-liner, but then we wouldn't propagate the
    // Result::Err easily, now would we?
    let mut sum: u32 = 0;
    for line in input.lines() {
        sum += match stringy {
            true => get_stringy_calibration(&line)?,
            false => get_calibration_value(&line)?,
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART_ONE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const EXAMPLE_INPUT_PART_TWO: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn should_get_calibration_value() -> Result<()> {
        let line = "1abc2";
        let calibration = get_calibration_value(&line)?;
        assert_eq!(12, calibration);
        Ok(())
    }

    #[test]
    fn should_get_single_digit() -> Result<()> {
        let line = "ab1cd";
        let cal = get_calibration_value(&line)?;
        assert_eq!(11, cal);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let res = get_total(EXAMPLE_INPUT_PART_ONE, false)?;
        assert_eq!(142, res);
        Ok(())
    }

    #[test]
    fn should_get_strings_too() -> Result<()> {
        let test = "eightjzqzhrllg1oneightfck";
        let val = get_stringy_calibration(&test)?;
        assert_eq!(88, val);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let res = get_total(EXAMPLE_INPUT_PART_TWO, true)?;
        assert_eq!(281, res);
        Ok(())
    }
}
