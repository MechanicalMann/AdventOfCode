use anyhow::Result;
use std::num::ParseIntError;

use crate::solver::Solver;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 3;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let data = self.input().get_lines()?;
        get_power_consumption(&data)
    }

    fn part_two(&self) -> Result<usize> {
        let data = self.input().get_lines()?;
        get_life_support(&data)
    }
}

// This is one of those where it feels like there's a really mathy solution
// based on fun properties of boolean logic.  But fuck that noise.
fn count_the_ones(input: &Vec<&String>) -> Result<Vec<u32>> {
    Ok(input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .reduce(|acc, e| acc.iter().zip(e.iter()).map(|z| z.0 + z.1).collect())
        .unwrap())
}

fn get_average_bitmask(bits: &Vec<String>) -> Result<String> {
    let total = bits.len();
    let counts = count_the_ones(&bits.iter().collect())?;
    Ok(counts
        .iter()
        .map(|&c| if c as usize > total / 2 { '1' } else { '0' })
        .collect())
}

// Well this part would've been easier if I'd used bits/bools, huh
fn get_gamma_from_bits(bits: &String) -> Result<usize, ParseIntError> {
    usize::from_str_radix(bits, 2)
}

fn get_epsilon_from_bits(bits: &String) -> Result<usize> {
    let flipped: String = bits
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();
    let epsilon = get_gamma_from_bits(&flipped)?;
    Ok(epsilon)
}

fn get_power_consumption(data: &Vec<String>) -> Result<usize> {
    let bits = get_average_bitmask(&data)?;
    let gamma = get_gamma_from_bits(&bits)?;
    let epsilon = get_epsilon_from_bits(&bits)?;
    Ok(gamma * epsilon)
}

enum Direction {
    MoreCommon,
    LessCommon,
}

fn get_rating(data: &Vec<String>, dir: Direction) -> Result<usize> {
    let digits = data[0].len();
    let mut values: Vec<_> = data.iter().collect();
    for i in 0..digits {
        let counts = count_the_ones(&values)?;
        let midpoint = values.len() as f32 / 2.0;
        let bit = match (&dir, counts[i] as f32 >= midpoint) {
            (Direction::MoreCommon, true) => '1',
            (Direction::MoreCommon, false) => '0',
            (Direction::LessCommon, true) => '0',
            (Direction::LessCommon, false) => '1',
        };
        values = values
            .iter()
            .filter(|s| s.chars().nth(i).unwrap() == bit)
            .map(|&s| s)
            .collect();
        if values.len() == 1 {
            break;
        }
    }
    let val = usize::from_str_radix(values[0], 2)?;
    Ok(val)
}

fn get_oxygen(data: &Vec<String>) -> Result<usize> {
    get_rating(&data, Direction::MoreCommon)
}

fn get_co2(data: &Vec<String>) -> Result<usize> {
    get_rating(&data, Direction::LessCommon)
}

fn get_life_support(data: &Vec<String>) -> Result<usize> {
    Ok(get_oxygen(&data)? * get_co2(&data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_get_one() {
        let data = vec!["1".to_owned(), "0".to_owned(), "1".to_owned()];
        let bits = get_average_bitmask(&data).unwrap();
        assert_eq!(String::from("1"), bits)
    }

    #[test]
    fn should_get_zero() {
        let data = vec!["0".to_owned(), "1".to_owned(), "0".to_owned()];
        let bits = get_average_bitmask(&data).unwrap();
        assert_eq!(String::from("0"), bits)
    }

    #[test]
    fn should_handle_multiple_digits() {
        let data = vec!["01".to_owned(), "10".to_owned(), "01".to_owned()];
        let bits = get_average_bitmask(&data).unwrap();
        assert_eq!(String::from("01"), bits)
    }

    #[test]
    fn should_get_gamma_bits() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let bits = get_average_bitmask(&data).unwrap();
        assert_eq!(String::from("10110"), bits)
    }

    #[test]
    fn should_get_power_consumption() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let power = get_power_consumption(&data).unwrap();
        assert_eq!(198, power)
    }

    #[test]
    fn should_filter_more_common() {
        let data = vec!["01".to_owned(), "10".to_owned(), "00".to_owned()];
        let rating = get_rating(&data, Direction::MoreCommon).unwrap();
        assert_eq!(1, rating)
    }

    #[test]
    fn should_filter_less_common() {
        let data = vec!["01".to_owned(), "10".to_owned(), "00".to_owned()];
        let rating = get_rating(&data, Direction::LessCommon).unwrap();
        assert_eq!(2, rating)
    }

    #[test]
    fn should_get_life_support_rating() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
        let rating = get_life_support(&data).unwrap();
        assert_eq!(230, rating)
    }
}
