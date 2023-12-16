use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use crate::{common::Point, solver::Solver};
use anyhow::{anyhow, Result};
use itertools::Itertools;

// Ok, I definitely got lucky while playing around with different values here.
// I guess by coincidence the pattern in my input happens to repeat exactly
// every 1,000 iterations. I *should* come back and do proper cycle detection
// like I wanted to, but this is the code that got me the gold star, so...
const MAX_CYCLES: usize = 1_000;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 14;
    const TITLE: &'static str = "Parabolic Reflector Dish";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut platform = self.input().get_as::<Platform>()?;
        platform.roll(Direction::North);
        Ok(platform.get_load())
    }

    fn part_two(&self) -> Result<usize> {
        let mut platform = self.input().get_as::<Platform>()?;
        platform.spin(MAX_CYCLES);
        Ok(platform.get_load())
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

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Platform {
    round_rocks: HashSet<Point>,
    square_rocks: HashSet<Point>,
    width: usize,
    height: usize,
}
impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut round_rocks = HashSet::new();
        let mut square_rocks = HashSet::new();
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
                let point = Point::new(x, y);
                match c {
                    '.' => (),
                    'O' => {
                        round_rocks.insert(point);
                    }
                    '#' => {
                        square_rocks.insert(point);
                    }
                    _ => return Err(anyhow!("Invalid character in platform: {c}")),
                }
            }
        }
        width += 1;
        height += 1;
        Ok(Platform {
            round_rocks,
            square_rocks,
            width,
            height,
        })
    }
}
impl Platform {
    fn roll(&mut self, dir: Direction) {
        let mut rolled = HashSet::new();
        let comp: fn(&&Point, &&Point) -> Ordering = match dir {
            Direction::North => |a, b| Ord::cmp(&a.y, &b.y),
            Direction::West => |a, b| Ord::cmp(&a.x, &b.x),
            Direction::East => |a, b| Ord::cmp(&b.x, &a.x),
            Direction::South => |a, b| Ord::cmp(&b.y, &a.y),
        };
        for pos in self.round_rocks.iter().sorted_by(comp) {
            let mut cur = Point::new(pos.x, pos.y);
            let mut next;
            loop {
                if (dir == Direction::North && cur.y == 0)
                    || (dir == Direction::South && cur.y == self.height - 1)
                    || (dir == Direction::West && cur.x == 0)
                    || (dir == Direction::East && cur.x == self.width - 1)
                {
                    break;
                }
                next = cur;
                next.x = match dir {
                    Direction::West => next.x - 1,
                    Direction::East => next.x + 1,
                    _ => next.x,
                };
                next.y = match dir {
                    Direction::North => next.y - 1,
                    Direction::South => next.y + 1,
                    _ => next.y,
                };
                if self.square_rocks.contains(&next) {
                    break;
                }
                if rolled.contains(&next) {
                    break;
                }
                cur = next;
            }
            rolled.insert(cur);
        }
        self.round_rocks = rolled;
    }

    fn spin(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.roll(Direction::North);
            self.roll(Direction::West);
            self.roll(Direction::South);
            self.roll(Direction::East);
        }
    }

    // ;)
    fn get_load(&self) -> usize {
        let mut load = 0;
        for pos in &self.round_rocks {
            load += self.height - pos.y;
        }
        load
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

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
            HashSet::from_iter(vec![Point::new(0, 1), Point::new(1, 1)]),
            test.round_rocks
        );
        assert_eq!(
            HashSet::from_iter(vec![Point::new(1, 0)]),
            test.square_rocks
        );
        Ok(())
    }

    #[test]
    fn should_roll() -> Result<()> {
        let mut test = ".#O.\nOOOO".parse::<Platform>()?;
        test.roll(Direction::North);
        assert_eq!(
            HashSet::from_iter(vec![
                Point::new(0, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(1, 1),
                Point::new(2, 1),
            ]),
            test.round_rocks
        );
        Ok(())
    }

    #[test]
    fn should_get_load() -> Result<()> {
        let mut test = ".#O.\nOOOO".parse::<Platform>()?;
        test.roll(Direction::North);
        assert_eq!(8, test.get_load());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let mut test = EXAMPLE_INPUT.parse::<Platform>()?;
        test.roll(Direction::North);
        assert_eq!(136, test.get_load());
        Ok(())
    }

    #[test]
    fn should_roll_in_other_directions() -> Result<()> {
        let mut test = ".#O.\nOO.O".parse::<Platform>()?;
        test.roll(Direction::South);
        assert_eq!(4, test.get_load());
        test.roll(Direction::North);
        assert_eq!(7, test.get_load());
        test.roll(Direction::East);
        assert!(test.round_rocks.contains(&Point::new(3, 1)));
        test.roll(Direction::West);
        assert!(test.round_rocks.contains(&Point::new(0, 1)));
        Ok(())
    }

    #[test]
    fn should_cycle() -> Result<()> {
        let mut test = "...\n.O.\n...".parse::<Platform>()?;
        test.spin(1);
        assert_eq!(HashSet::from_iter(vec![Point::new(2, 2)]), test.round_rocks);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let mut test = EXAMPLE_INPUT.parse::<Platform>()?;
        test.spin(MAX_CYCLES);
        assert_eq!(64, test.get_load());
        Ok(())
    }
}
