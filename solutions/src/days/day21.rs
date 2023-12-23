use std::{collections::HashSet, str::FromStr};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

type Point = IPoint;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 21;
    const TITLE: &'static str = "Step Counter";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let garden = self.input().get_as::<Garden>()?;
        let steps = garden.walk(64);
        Ok(steps.len())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

const OFFSETS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

struct Garden {
    rocks: HashSet<Point>,
    start: Point,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}
impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rocks = HashSet::new();
        let (mut max_x, mut max_y) = (0, 0);
        let mut start = Point::new(0, 0);

        for (y, line) in s.lines().enumerate() {
            max_y = y as isize;
            for (x, c) in line.chars().enumerate() {
                max_x = x as isize;
                match c {
                    '#' => {
                        rocks.insert(Point::new(max_x, max_y));
                    }
                    'S' => start = Point::new(max_x, max_y),
                    _ => (),
                }
            }
        }

        Ok(Self {
            rocks,
            start,
            min_x: 0,
            min_y: 0,
            max_x,
            max_y,
        })
    }
}
impl Garden {
    fn step(&self, start: &HashSet<Point>) -> HashSet<Point> {
        let mut steps = HashSet::new();
        for point in start {
            for offset in OFFSETS {
                let next = point + offset;
                if self.rocks.contains(&next)
                    || next.x > self.max_x
                    || next.x < self.min_x
                    || next.y > self.max_y
                    || next.y < self.min_y
                {
                    continue;
                }
                steps.insert(next);
            }
        }
        steps
    }

    fn get_start(&self) -> HashSet<Point> {
        HashSet::from_iter([self.start])
    }

    fn walk(&self, to: usize) -> HashSet<Point> {
        let mut steps = self.get_start();
        for _ in 0..to {
            steps = self.step(&steps);
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "..#.#\n#.S..".parse::<Garden>()?;
        let expected_rocks =
            HashSet::from_iter([Point::new(2, 0), Point::new(4, 0), Point::new(0, 1)]);
        let expected_start = Point::new(2, 1);
        assert_eq!(expected_start, test.start);
        assert_eq!(expected_rocks, test.rocks);
        assert_eq!(4, test.max_x);
        assert_eq!(1, test.max_y);
        Ok(())
    }

    #[test]
    fn should_step() -> Result<()> {
        let test = "..#.#\n#.S..".parse::<Garden>()?;
        let steps = test.step(&test.get_start());
        let expected_steps = HashSet::from_iter([Point::new(1, 1), Point::new(3, 1)]);
        assert_eq!(expected_steps, steps);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let garden = EXAMPLE_INPUT.parse::<Garden>()?;
        let steps = garden.walk(6);
        assert_eq!(16, steps.len());
        Ok(())
    }
}
