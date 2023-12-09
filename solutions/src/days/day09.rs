use std::str::FromStr;

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 9;
    const TITLE: &'static str = "Mirage Maintenance";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let oases = self.input().get_lines_as::<History>()?;
        let sum = oases.iter().map(|o| o.predict_next()).sum();
        Ok(sum)
    }

    fn part_two(&self) -> Result<isize> {
        let oases = self.input().get_lines_as::<History>()?;
        let sum = oases.iter().map(|o| o.predict_prev()).sum();
        Ok(sum)
    }
}

struct History {
    value: Vec<isize>,
}
impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let value = s.split(' ').filter_map(|x| x.parse().ok()).collect_vec();
        Ok(History { value })
    }
}
impl History {
    fn predict_next(&self) -> isize {
        predict_next(&self.value)
    }

    fn predict_prev(&self) -> isize {
        predict_next(&self.value.iter().rev().cloned().collect_vec())
    }
}

fn predict_next(value: &[isize]) -> isize {
    if value.len() == 1 {
        return value[0];
    }
    let mut diffs: Vec<isize> = vec![];
    let mut next_diffs: Vec<isize> = vec![];
    let mut increments: Vec<isize> = vec![];
    let mut latest = 0;
    let mut all_zero = true;
    diffs.extend(value.iter());
    loop {
        for i in 0..diffs.len() - 1 {
            let j = i + 1;
            latest = diffs[j] - diffs[i];
            next_diffs.push(latest);
            if latest != 0 {
                all_zero = false;
            }
        }
        increments.push(latest);
        diffs.clear();
        diffs.append(&mut next_diffs);
        if all_zero {
            break;
        }
        all_zero = true;
    }
    let max = value[value.len() - 1];
    let incr: isize = increments.iter().sum();
    max + incr
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "-1 2 3".parse::<History>()?;
        assert_eq!(vec![-1, 2, 3], test.value);
        Ok(())
    }

    #[test]
    fn should_get_next() -> Result<()> {
        let test = "1 2 3".parse::<History>()?;
        assert_eq!(4, test.predict_next());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let hists = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<History>().ok())
            .collect_vec();
        let total: isize = hists.iter().map(|h| h.predict_next()).sum();
        assert_eq!(114, total);
        Ok(())
    }

    #[test]
    fn should_get_prev() -> Result<()> {
        let test = "1 2 3".parse::<History>()?;
        assert_eq!(0, test.predict_prev());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let hists = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<History>().ok())
            .collect_vec();
        let total: isize = hists.iter().map(|h| h.predict_prev()).sum();
        assert_eq!(2, total);
        Ok(())
    }
}
