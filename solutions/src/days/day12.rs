use std::str::FromStr;

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 12;
    const TITLE: &'static str = "Hot Springs";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let records = self.input().get_lines_as::<Record>()?;
        Ok(records.iter().map(|r| r.get_valid_permutations()).sum())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}
impl TryFrom<char> for Spring {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(anyhow!("Invalid spring: {value}")),
        }
    }
}

struct Record {
    springs: Vec<Spring>,
    damaged_groups: Vec<usize>,
}
impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(' ').collect_vec();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid spring record: {s}"));
        }
        let springs = parts[0]
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect_vec();
        let damaged_groups = parts[1]
            .split(',')
            .filter_map(|n| n.parse().ok())
            .collect_vec();
        Ok(Record {
            springs,
            damaged_groups,
        })
    }
}
impl Record {
    fn is_valid(&self) -> bool {
        is_valid_record(&self.springs, &self.damaged_groups)
    }

    fn get_valid_permutations(&self) -> usize {
        if self.is_valid() {
            return 1;
        }
        let mut valid = 0;
        let permutations = get_permutations(&self.springs);
        match permutations {
            Some(perms) => {
                for perm in &perms {
                    if is_valid_record(&perm, &self.damaged_groups) {
                        valid += 1;
                    }
                }
            }
            None => (),
        }
        valid
    }
}

fn get_permutations(springs: &[Spring]) -> Option<Vec<Vec<Spring>>> {
    let mut unknown: Option<usize> = None;
    for (i, s) in springs.iter().enumerate() {
        match s {
            Spring::Unknown => {
                unknown = Some(i);
                break;
            }
            _ => (),
        }
    }
    match unknown {
        Some(idx) => {
            let after = idx + 1;
            let base = &springs[..idx];
            let mut ret = vec![];
            let mut left = Vec::from_iter(base.iter().cloned());
            let mut right = Vec::from_iter(base.iter().cloned());
            left.push(Spring::Operational);
            right.push(Spring::Damaged);
            if after < springs.len() {
                for next in [left, right] {
                    let perms = get_permutations(&springs[after..]);
                    match perms {
                        Some(others) => {
                            for o in others {
                                let mut permutation = next.clone();
                                permutation.extend(o.iter());
                                ret.push(permutation);
                            }
                        }
                        None => {
                            ret.push(next.iter().chain(&springs[after..]).cloned().collect_vec());
                        }
                    }
                }
            } else {
                ret.push(left);
                ret.push(right);
            }
            Some(ret)
        }
        None => None,
    }
}

fn is_valid_record(springs: &[Spring], damaged_groups: &[usize]) -> bool {
    let mut damaged = vec![];
    let mut group: Option<usize> = None;
    for s in springs {
        match s {
            Spring::Unknown => return false,
            Spring::Operational => match group {
                Some(x) => {
                    damaged.push(x);
                    group = None;
                }
                None => (),
            },
            Spring::Damaged => match group {
                Some(x) => group = Some(x + 1),
                None => group = Some(1),
            },
        }
    }
    match group {
        Some(x) => {
            damaged.push(x);
        }
        None => (),
    }
    damaged == damaged_groups
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "#.?# 1,2".parse::<Record>()?;
        assert_eq!(
            vec![
                Spring::Damaged,
                Spring::Operational,
                Spring::Unknown,
                Spring::Damaged
            ],
            test.springs
        );
        assert_eq!(vec![1, 2], test.damaged_groups);
        Ok(())
    }

    #[test]
    fn should_validate() -> Result<()> {
        let one = "#.##.### 1,2,3".parse::<Record>()?;
        assert!(one.is_valid());
        let two = "#.?#.### 1,2,3".parse::<Record>()?;
        assert!(!two.is_valid());
        let three = ".###.### 1,2,3".parse::<Record>()?;
        assert!(!three.is_valid());
        Ok(())
    }

    #[test]
    fn should_get_valid_perms() -> Result<()> {
        let test = "#.?#.### 1,2,3".parse::<Record>()?;
        assert_eq!(1, test.get_valid_permutations());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let records = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Record>().ok())
            .collect_vec();
        let valid: usize = records.iter().map(|r| r.get_valid_permutations()).sum();
        assert_eq!(21, valid);
        Ok(())
    }
}
