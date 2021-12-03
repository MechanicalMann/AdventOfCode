use anyhow::Result;
use std::num::ParseIntError;

use crate::input::AdventInput;

const DAY: u8 = 3;

pub mod part1 {
    use super::*;
    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get_lines()?;
        get_power_consumption(&data)
    }
}

pub mod part2 {
    use super::*;
    pub fn solve() -> Result<isize> {
        Ok(0)
    }
}

// This is one of those where it feels like there's a really mathy solution
// based on fun properties of boolean logic.  But fuck that noise.
fn count_the_ones(input: &Vec<String>) -> Result<Vec<u32>> {
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
    let counts = count_the_ones(&bits)?;
    println!("{:?}", counts);
    Ok(counts
        .iter()
        .map(|&c| if c as usize > total / 2 { '1' } else { '0' })
        .collect())
}

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
}
