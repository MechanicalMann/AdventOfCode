use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 03;
    const TITLE: &'static str = "Lobby";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        joltage_part1(&self.input().get()?)
    }

    fn part_two(&self) -> Result<usize> {
        joltage_part2(&self.input().get()?)
    }
}

fn joltage_part1(input: &str) -> Result<usize> {
    Ok(input.lines().filter_map(|s| get_joltage(s, 2).ok()).sum())
}

fn joltage_part2(input: &str) -> Result<usize> {
    Ok(input.lines().filter_map(|s| get_joltage(s, 12).ok()).sum())
}

fn get_joltage(s: &str, digits: usize) -> Result<usize> {
    let chars = s.chars().collect_vec();
    if chars.len() < digits {
        return Err(anyhow!("Cannot get {digits} digit number from string {s}"));
    }
    let mut stack = vec![0usize; digits];
    for i in 0..s.len() {
        let d: usize = chars[i].to_digit(10).unwrap().try_into()?;
        let mut found = false;
        for j in 0..digits {
            if found {
                stack[j] = 0;
            } else if i < s.len() - (digits - j - 1) && d > stack[j] {
                stack[j] = d.try_into()?;
                found = true;
            }
        }
    }
    let (mut res, d): (usize, u32) = (0, digits.try_into()?);
    for i in 0..digits {
        res += stack[i] * 10usize.pow(d - (i as u32) - 1);
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn should_get_joltage() {}

    #[test]
    fn should_get_joltage_for_arbitrary_digits() -> Result<()> {
        assert_eq!(45, get_joltage("12345", 2)?);
        assert_eq!(543, get_joltage("54321", 3)?);
        assert_eq!(1213, get_joltage("111213", 4)?);
        assert_eq!(12345, get_joltage("111112345", 5)?);
        assert_eq!(543211, get_joltage("543211111", 6)?);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let total: usize = joltage_part1(&EXAMPLE_INPUT)?;
        assert_eq!(357, total);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let total: usize = joltage_part2(&EXAMPLE_INPUT)?;
        assert_eq!(3121910778619, total);
        Ok(())
    }
}
