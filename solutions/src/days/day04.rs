use std::{collections::HashSet, str::FromStr};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 04;
    const TITLE: &'static str = "Printing Department";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        Ok(map.count_accessible())
    }

    fn part_two(&self) -> Result<usize> {
        let mut map = self.input().get_as::<Map>()?;
        Ok(map.clean())
    }
}

const ADJACENTS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug)]
struct Map {
    rolls: HashSet<IPoint>,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rolls = HashSet::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '@' {
                    rolls.insert(IPoint::new(x as isize, y as isize));
                }
            }
        }
        Ok(Self { rolls })
    }
}
impl Map {
    fn get_accessible(&self) -> HashSet<IPoint> {
        let mut acc = HashSet::new();
        for &r in self.rolls.iter() {
            let mut a_count = 0;
            for a in ADJACENTS.iter().map(|a| r + a) {
                if self.rolls.contains(&a) {
                    a_count += 1;
                }
            }
            if a_count < 4 {
                acc.insert(r);
            }
        }
        acc
    }

    fn count_accessible(&self) -> usize {
        self.get_accessible().len()
    }

    fn clear_accessible(&mut self) {
        for a in &self.get_accessible() {
            self.rolls.remove(a);
        }
    }

    fn clean(&mut self) -> usize {
        let mut removed = 0;
        loop {
            let before = self.rolls.len();
            self.clear_accessible();
            let after = self.rolls.len();
            removed += before - after;
            if after == before {
                break;
            }
        }
        removed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn should_parse() -> Result<()> {
        let actual = "@..\n.@.\n@.@".parse::<Map>()?;
        assert_eq!(
            HashSet::from([
                IPoint::new(0, 0),
                IPoint::new(1, 1),
                IPoint::new(0, 2),
                IPoint::new(2, 2)
            ]),
            actual.rolls
        );
        Ok(())
    }

    #[test]
    fn should_count_accessible_rolls() -> Result<()> {
        let map = "@@@\n@@@\n".parse::<Map>()?;
        assert_eq!(4, map.count_accessible());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<Map>()?;
        assert_eq!(13, map.count_accessible());
        Ok(())
    }

    #[test]
    fn should_remove_accessible() -> Result<()> {
        let mut map = "@@@\n@@@\n".parse::<Map>()?;
        map.clear_accessible();
        assert_eq!(
            HashSet::from([IPoint::new(1, 0), IPoint::new(1, 1),]),
            map.rolls
        );
        Ok(())
    }

    #[test]
    fn should_clean() -> Result<()> {
        let mut map = "@@@\n@@@\n".parse::<Map>()?;
        let cleaned = map.clean();
        assert_eq!(HashSet::new(), map.rolls);
        assert_eq!(6, cleaned);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let mut map = EXAMPLE_INPUT.parse::<Map>()?;
        let cleaned = map.clean();
        assert_eq!(43, cleaned);
        Ok(())
    }
}
