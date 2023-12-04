use std::{collections::HashSet, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 4;
    const TITLE: &'static str = "Scratchcards";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let scratchers = self.input().get_lines_as::<Scratcher>()?;
        Ok(scratchers.iter().map(|s| s.score()).sum())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

struct Scratcher {
    numbers: HashSet<usize>,
    winners: HashSet<usize>,
}
impl FromStr for Scratcher {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut numbers = HashSet::new();
        let mut winners = HashSet::new();
        let win_start = s.find(':').ok_or(anyhow!("Invalid scratch card!"))?;
        let num_start = s.find('|').ok_or(anyhow!("Invalid scratch card!"))?;
        for winner in s[win_start + 1..num_start - 1].split_whitespace() {
            winners.insert(winner.parse()?);
        }
        for number in s[num_start + 1..].split_whitespace() {
            numbers.insert(number.parse()?);
        }
        Ok(Scratcher { numbers, winners })
    }
}
impl Scratcher {
    fn score(&self) -> usize {
        let wins = self.winners.intersection(&self.numbers).count();
        if wins == 0 {
            return 0;
        }
        2usize.pow(wins as u32 - 1)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "Game 1: 1 2 3 |  4  5  6";
        let scratcher = test.parse::<Scratcher>()?;
        let expected_win: HashSet<usize> = HashSet::from_iter(vec![1, 2, 3].iter().cloned());
        let expected_num: HashSet<usize> = HashSet::from_iter(vec![4, 5, 6].iter().cloned());
        assert_eq!(expected_win, scratcher.winners);
        assert_eq!(expected_num, scratcher.numbers);
        Ok(())
    }

    #[test]
    fn should_score() -> Result<()> {
        let test = "Game 1: 1 2 3 4 5 6 | 2 3 4 5";
        let scratcher = test.parse::<Scratcher>()?;
        assert_eq!(8, scratcher.score());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let scratchers = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Scratcher>().unwrap());
        let scores = scratchers.map(|s| s.score()).collect_vec();
        let total: usize = scores.iter().sum();
        assert_eq!(13, total);
        Ok(())
    }
}
