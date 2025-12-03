use std::{collections::HashSet, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 02;
    const TITLE: &'static str = "Gift Shop";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let ranges = self.input().get_csv_as::<Range>()?;
        Ok(get_total_invalid(&ranges))
    }

    fn part_two(&self) -> Result<usize> {
        let ranges = self.input().get_csv_as::<Range>()?;
        Ok(get_total_invalid_2(&ranges))
    }
}

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}
impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let split = s.split('-').collect_vec();
        if split.len() != 2 {
            return Err(anyhow!("Invalid range specification"));
        }
        let (min, max) = (split[0].parse()?, split[1].parse()?);
        Ok(Self { min, max })
    }
}
impl Range {
    fn get_invalid_ids(&self) -> HashSet<usize> {
        let mut res = HashSet::new();
        let (d_min, d_max) = (self.min.ilog10() + 1, self.max.ilog10() + 1);
        let (upper_min, upper_max) = (
            self.min / 10usize.pow((d_min + 1) / 2),
            self.max / 10usize.pow(d_max / 2),
        );
        for d in d_min..=d_max {
            if d % 2 != 0 {
                continue;
            }
            let range_min = std::cmp::max(1 * 10usize.pow((d / 2) - 1), upper_min);
            let upper = std::cmp::min(get_max_for_digits(d / 2), upper_max);
            for i in range_min..=upper {
                let id = (i * 10usize.pow(d / 2)) + i;
                if id < self.min {
                    continue;
                }
                if id > self.max {
                    break;
                }
                res.insert(id);
            }
        }
        res
    }

    // There's probably a way to generalize this solution to work for parts 1 & 2
    // However I am very tired today so fuck it
    fn get_invalid_ids_2(&self) -> HashSet<usize> {
        let mut res = HashSet::new();
        let (d_min, d_max) = (self.min.ilog10() + 1, self.max.ilog10() + 1);
        let upper_max = self.max / 10usize.pow(d_max / 2);
        for d in d_min..=d_max {
            if d < 2 {
                continue;
            }
            let upper = std::cmp::min(get_max_for_digits(d / 2), upper_max);
            for i in 1..=upper {
                let di = i.ilog10() + 1;
                let mut id = 0;
                for j in 0..(d / di) {
                    id += i * 10usize.pow(di * j);
                }
                if id < self.min {
                    continue;
                }
                if id > self.max {
                    continue;
                }
                res.insert(id);
            }
        }
        res
    }
}

fn get_max_for_digits(d: u32) -> usize {
    10usize.pow(d) - 1
}

fn get_total_invalid(ranges: &[Range]) -> usize {
    let mut ids: HashSet<usize> = HashSet::new();
    for r in ranges {
        ids.extend(&r.get_invalid_ids());
    }
    ids.iter().sum()
}

fn get_total_invalid_2(ranges: &[Range]) -> usize {
    let mut ids: HashSet<usize> = HashSet::new();
    for r in ranges {
        ids.extend(&r.get_invalid_ids_2());
    }
    ids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "123-456".parse::<Range>()?;
        assert_eq!(123, test.min);
        assert_eq!(456, test.max);
        Ok(())
    }

    #[test]
    fn should_get_max_for_digits() {
        assert_eq!(0, get_max_for_digits(0));
        assert_eq!(9, get_max_for_digits(1));
        assert_eq!(99, get_max_for_digits(2));
        assert_eq!(999, get_max_for_digits(3));
        assert_eq!(9999, get_max_for_digits(4));
        assert_eq!(99999, get_max_for_digits(5));
        assert_eq!(999999, get_max_for_digits(6));
        assert_eq!(9999999, get_max_for_digits(7));
        assert_eq!(99999999, get_max_for_digits(8));
        assert_eq!(999999999, get_max_for_digits(9));
    }

    #[test]
    fn should_sum_invalid_ids() -> Result<()> {
        let range = "1-1001".parse::<Range>()?;
        assert_eq!(495, get_total_invalid(&[range]));
        Ok(())
    }

    #[test]
    fn should_get_invalid_ids() -> Result<()> {
        let range = "1-1001".parse::<Range>()?;
        assert_eq!(
            HashSet::from([11, 22, 33, 44, 55, 66, 77, 88, 99]),
            range.get_invalid_ids()
        );
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let ranges = EXAMPLE_INPUT
            .split(',')
            .filter_map(|r| r.parse::<Range>().ok())
            .collect_vec();
        assert_eq!(1227775554, get_total_invalid(&ranges));
        Ok(())
    }

    #[test]
    fn should_get_invalid_ids_2() -> Result<()> {
        let range = "2121212118-2121212124".parse::<Range>()?;
        assert_eq!(HashSet::from([2121212121]), range.get_invalid_ids_2());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let ranges = EXAMPLE_INPUT
            .split(',')
            .filter_map(|r| r.parse::<Range>().ok())
            .collect_vec();
        assert_eq!(4174379265, get_total_invalid_2(&ranges));
        Ok(())
    }
}
