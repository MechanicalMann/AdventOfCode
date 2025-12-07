use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::Point, solver::Solver};
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 07;
    const TITLE: &'static str = "Laboratories";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let manifold = self.input().get_as::<Manifold>()?;
        Ok(manifold.split())
    }

    fn part_two(&self) -> Result<usize> {
        let manifold = self.input().get_as::<Manifold>()?;
        Ok(manifold.timelines())
    }
}

#[derive(Debug)]
struct Manifold {
    height: usize,
    start: usize,
    splitters: HashSet<Point>,
}
impl FromStr for Manifold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut height = 0;
        let mut start = 0;
        let mut splitters = HashSet::new();
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    'S' => start = x,
                    '^' => {
                        splitters.insert(Point::new(x, y));
                    }
                    _ => (),
                }
            }
            height = y + 1;
        }
        Ok(Self {
            height,
            start,
            splitters,
        })
    }
}
impl Manifold {
    fn split(&self) -> usize {
        let mut splits = 0;
        let mut beams = HashSet::from([self.start]);
        for y in 1..self.height {
            let mut next = HashSet::new();
            for &b in &beams {
                if self.splitters.contains(&Point::new(b, y)) {
                    next.insert(b - 1);
                    next.insert(b + 1);
                    splits += 1;
                } else {
                    next.insert(b);
                }
            }
            beams = next;
        }
        splits
    }

    fn timelines(&self) -> usize {
        let mut beams = HashMap::from([(self.start, 1usize)]);
        for y in 1..self.height {
            let mut next = HashMap::new();
            for (x, count) in beams {
                if self.splitters.contains(&Point::new(x, y)) {
                    *next.entry(x - 1).or_insert(0) += count;
                    *next.entry(x + 1).or_insert(0) += count;
                } else {
                    *next.entry(x).or_insert(0) += count;
                }
            }
            beams = next;
        }
        beams.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn should_parse() -> Result<()> {
        let test = ".S.\n^.^".parse::<Manifold>()?;
        assert_eq!(2, test.height);
        assert_eq!(1, test.start);
        assert_eq!(
            HashSet::from([Point::new(0, 1), Point::new(2, 1)]),
            test.splitters
        );
        Ok(())
    }

    #[test]
    fn should_split() -> Result<()> {
        let test = "...S...\n...^...\n..^.^..\n.^.....".parse::<Manifold>()?;
        assert_eq!(4, test.split());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<Manifold>()?;
        assert_eq!(21, test.split());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<Manifold>()?;
        assert_eq!(40, test.timelines());
        Ok(())
    }
}
