use itertools::Itertools;

use crate::solver::Solver;
use std::str::FromStr;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 4;
    const TITLE: &'static str = "Camp Cleanup";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<usize> {
        let pairs = self.input().get_lines_as::<Pair>()?;
        Ok(count_fully_contained(&pairs))
    }

    fn part_two(&self) -> anyhow::Result<usize> {
        let pairs = self.input().get_lines_as::<Pair>()?;
        Ok(count_overlaps(&pairs))
    }
}

fn count_fully_contained(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|Pair { left, right }| left.contains(right) || right.contains(left))
        .count()
}

fn count_overlaps(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|Pair { left, right }| left.overlaps(right))
        .count()
}

#[derive(Debug, PartialEq)]
struct Range {
    start: usize,
    end: usize,
}
impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (self.start <= other.start && self.end >= other.start)
            || (self.start <= other.end && self.end >= other.end)
            || other.contains(self)
    }
}
impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let halves = s.split('-').collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!("Invalid range specification"));
        }
        let (start, end) = (halves[0].parse::<usize>()?, halves[1].parse::<usize>()?);
        Ok(Range { start, end })
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    left: Range,
    right: Range,
}
impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let halves = s.split(',').collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!("Invalid pair specification"));
        }
        let (left, right) = (halves[0].parse::<Range>()?, halves[1].parse::<Range>()?);
        Ok(Pair { left, right })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn should_parse() {
        let input = "1-9,2-8";
        let expected = Pair {
            left: Range { start: 1, end: 9 },
            right: Range { start: 2, end: 8 },
        };
        let actual = input.parse::<Pair>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_detect_contains() {
        let left = Range { start: 1, end: 9 };
        let right = Range { start: 2, end: 4 };
        assert!(left.contains(&right));
    }

    #[test]
    fn should_count_contained() {
        let pairs = vec![
            Pair {
                left: Range { start: 1, end: 9 },
                right: Range { start: 2, end: 4 },
            },
            Pair {
                left: Range { start: 2, end: 4 },
                right: Range { start: 6, end: 8 },
            },
            Pair {
                left: Range { start: 2, end: 4 },
                right: Range { start: 2, end: 3 },
            },
        ];
        let count = count_fully_contained(&pairs);
        assert_eq!(2, count);
    }

    #[test]
    fn should_solve_part_1() {
        let pairs = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .collect_vec();
        let count = count_fully_contained(&pairs);
        assert_eq!(2, count);
    }

    #[test]
    fn should_count_overlaps() {
        let pairs = vec![
            Pair {
                left: Range { start: 1, end: 9 },
                right: Range { start: 2, end: 4 },
            },
            Pair {
                left: Range { start: 2, end: 4 },
                right: Range { start: 6, end: 8 },
            },
            Pair {
                left: Range { start: 2, end: 2 },
                right: Range { start: 1, end: 5 },
            },
        ];
        let count = count_overlaps(&pairs);
        assert_eq!(2, count);
    }

    #[test]
    fn should_solve_part_2() {
        let pairs = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .collect_vec();
        let count = count_overlaps(&pairs);
        assert_eq!(4, count);
    }
}
