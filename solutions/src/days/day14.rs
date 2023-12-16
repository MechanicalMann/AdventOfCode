use std::{collections::HashMap, str::FromStr};

use crate::{common::Point, solver::Solver};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 14;
    const TITLE: &'static str = "Parabolic Reflector Dish";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut platform = self.input().get_as::<Platform>()?;
        platform.roll();
        Ok(platform.get_load())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}
impl TryFrom<char> for Rock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'O' => Ok(Rock::Round),
            '#' => Ok(Rock::Square),
            _ => Err(anyhow!("Invalid rock: {value}")),
        }
    }
}

struct Platform {
    rocks: HashMap<Point, Rock>,
    width: usize,
    height: usize,
}
impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rocks = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            if y > height {
                height = y;
            }
            for (x, c) in line.chars().enumerate() {
                if x > width {
                    width = x;
                }
                if c == '.' {
                    continue;
                }
                let rock = c.try_into()?;
                rocks.insert(Point::new(x, y), rock);
            }
        }
        width += 1;
        height += 1;
        Ok(Platform {
            rocks,
            width,
            height,
        })
    }
}
impl Platform {
    fn roll(&mut self) {
        let mut rolled = HashMap::new();
        let mut rolling = vec![];
        for (&pos, &rock) in self
            .rocks
            .iter()
            .sorted_by(|(a, _), (b, __)| Ord::cmp(&a.y, &b.y))
        {
            match rock {
                Rock::Round => rolling.push(pos),
                Rock::Square => {
                    rolled.insert(pos, rock);
                }
            }
        }
        for pos in rolling {
            let mut cur = Point::new(pos.x, pos.y);
            let mut next;
            loop {
                if cur.y == 0 {
                    break;
                }
                next = cur;
                next.y -= 1;
                match rolled.get(&next) {
                    Some(_) => break, // any kind of rock above stops the roll
                    None => cur = next,
                }
            }
            rolled.insert(cur, Rock::Round);
        }
        self.rocks = rolled;
    }

    // ;)
    fn get_load(&self) -> usize {
        let mut load = 0;
        for y in 0..self.height {
            let row_load = self.height - y;
            for x in 0..self.width {
                match self.rocks.get(&Point::new(x, y)) {
                    Some(rock) => match rock {
                        Rock::Round => load += row_load,
                        Rock::Square => (),
                    },
                    None => (),
                }
            }
        }
        load
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn should_parse() -> Result<()> {
        let test = ".#..\nOO..".parse::<Platform>()?;
        assert_eq!(4, test.width);
        assert_eq!(2, test.height);
        assert_eq!(
            HashMap::from_iter(vec![
                (Point::new(1, 0), Rock::Square),
                (Point::new(0, 1), Rock::Round),
                (Point::new(1, 1), Rock::Round),
            ]),
            test.rocks
        );
        Ok(())
    }

    #[test]
    fn should_roll() -> Result<()> {
        let mut test = ".#O.\nOOOO".parse::<Platform>()?;
        test.roll();
        assert_eq!(
            HashMap::from_iter(vec![
                (Point::new(0, 0), Rock::Round),
                (Point::new(1, 0), Rock::Square),
                (Point::new(2, 0), Rock::Round),
                (Point::new(3, 0), Rock::Round),
                (Point::new(1, 1), Rock::Round),
                (Point::new(2, 1), Rock::Round),
            ]),
            test.rocks
        );
        Ok(())
    }

    #[test]
    fn should_get_load() -> Result<()> {
        let mut test = ".#O.\nOOOO".parse::<Platform>()?;
        test.roll();
        assert_eq!(8, test.get_load());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let mut test = EXAMPLE_INPUT.parse::<Platform>()?;
        test.roll();
        assert_eq!(136, test.get_load());
        Ok(())
    }
}
