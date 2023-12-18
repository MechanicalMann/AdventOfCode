use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::{anyhow, Ok, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 16;
    const TITLE: &'static str = "The Floor Will Be Lava";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let contraption = self.input().get_as::<Contraption>()?;
        let illuminated = contraption.illuminate(IPoint::new(0, 0), Direction::Right);
        Ok(illuminated.len())
    }

    fn part_two(&self) -> Result<usize> {
        let contraption = self.input().get_as::<Contraption>()?;
        Ok(contraption.find_max_illumination())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    MirrorUp,
    MirrorDown,
    SplitVert,
    SplitHoriz,
}
impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Empty),
            '/' => Ok(Tile::MirrorUp),
            '\\' => Ok(Tile::MirrorDown),
            '|' => Ok(Tile::SplitVert),
            '-' => Ok(Tile::SplitHoriz),
            _ => Err(anyhow!("Invalid tile: {value}")),
        }
    }
}
impl Tile {
    const OFFSET_UP: (isize, isize) = (0, -1);
    const OFFSET_DOWN: (isize, isize) = (0, 1);
    const OFFSET_LEFT: (isize, isize) = (-1, 0);
    const OFFSET_RIGHT: (isize, isize) = (1, 0);

    fn get_next_offset(&self, direction: Direction) -> Vec<((isize, isize), Direction)> {
        let mut offsets = vec![];
        match (self, direction) {
            (Tile::Empty, Direction::Up) => offsets.push((Tile::OFFSET_UP, direction)),
            (Tile::Empty, Direction::Down) => offsets.push((Tile::OFFSET_DOWN, direction)),
            (Tile::Empty, Direction::Left) => offsets.push((Tile::OFFSET_LEFT, direction)),
            (Tile::Empty, Direction::Right) => offsets.push((Tile::OFFSET_RIGHT, direction)),
            (Tile::MirrorUp, Direction::Up) => offsets.push((Tile::OFFSET_RIGHT, Direction::Right)),
            (Tile::MirrorUp, Direction::Down) => offsets.push((Tile::OFFSET_LEFT, Direction::Left)),
            (Tile::MirrorUp, Direction::Left) => offsets.push((Tile::OFFSET_DOWN, Direction::Down)),
            (Tile::MirrorUp, Direction::Right) => offsets.push((Tile::OFFSET_UP, Direction::Up)),
            (Tile::MirrorDown, Direction::Up) => offsets.push((Tile::OFFSET_LEFT, Direction::Left)),
            (Tile::MirrorDown, Direction::Down) => {
                offsets.push((Tile::OFFSET_RIGHT, Direction::Right))
            }
            (Tile::MirrorDown, Direction::Left) => offsets.push((Tile::OFFSET_UP, Direction::Up)),
            (Tile::MirrorDown, Direction::Right) => {
                offsets.push((Tile::OFFSET_DOWN, Direction::Down))
            }
            (Tile::SplitVert, Direction::Up) => offsets.push((Tile::OFFSET_UP, direction)),
            (Tile::SplitVert, Direction::Down) => offsets.push((Tile::OFFSET_DOWN, direction)),
            (Tile::SplitVert, _) => offsets.extend([
                (Tile::OFFSET_UP, Direction::Up),
                (Tile::OFFSET_DOWN, Direction::Down),
            ]),
            (Tile::SplitHoriz, Direction::Left) => offsets.push((Tile::OFFSET_LEFT, direction)),
            (Tile::SplitHoriz, Direction::Right) => offsets.push((Tile::OFFSET_RIGHT, direction)),
            (Tile::SplitHoriz, _) => offsets.extend([
                (Tile::OFFSET_LEFT, Direction::Left),
                (Tile::OFFSET_RIGHT, Direction::Right),
            ]),
        }
        offsets
    }
}

struct Contraption {
    tiles: HashMap<IPoint, Tile>,
    width: usize,
    height: usize,
}
impl FromStr for Contraption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = HashMap::new();
        let (mut width, mut height) = (0, 0);
        for (y, line) in s.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                let tile = c.try_into()?;
                tiles.insert(IPoint::new(x as isize, y as isize), tile);
            }
        }
        Ok(Contraption {
            tiles,
            width,
            height,
        })
    }
}
impl Contraption {
    fn illuminate(&self, start: IPoint, direction: Direction) -> HashSet<IPoint> {
        let mut visited = HashSet::new();
        let beam = self.get_next(start, direction, &mut visited);
        HashSet::from_iter(beam)
    }

    fn get_next(
        &self,
        start: IPoint,
        direction: Direction,
        visited: &mut HashSet<(IPoint, Direction)>,
    ) -> Vec<IPoint> {
        let mut illuminated = vec![];
        let tile;
        match self.tiles.get(&start) {
            Some(t) => tile = t,
            None => return illuminated,
        }
        visited.insert((start, direction));
        illuminated.push(start);
        for (offset, next_dir) in tile.get_next_offset(direction) {
            let next = start + offset;
            if visited.contains(&(next, next_dir)) {
                continue;
            }
            illuminated.extend(self.get_next(next, next_dir, visited));
        }
        illuminated
    }

    fn find_max_illumination(&self) -> usize {
        let mut max = 0;
        for x in 0..self.width {
            for y in [0, self.height - 1] {
                let dir = match y {
                    0 => Direction::Down,
                    _ => Direction::Up,
                };
                let beam = self.illuminate(IPoint::new(x as isize, y as isize), dir);
                if beam.len() > max {
                    max = beam.len();
                }
            }
        }
        for y in 0..self.height {
            for x in [0, self.width - 1] {
                let dir = match x {
                    0 => Direction::Right,
                    _ => Direction::Left,
                };
                let beam = self.illuminate(IPoint::new(x as isize, y as isize), dir);
                if beam.len() > max {
                    max = beam.len();
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "./\\\n|-.".parse::<Contraption>()?;
        let expected = HashMap::from_iter(vec![
            (IPoint::new(0, 0), Tile::Empty),
            (IPoint::new(1, 0), Tile::MirrorUp),
            (IPoint::new(2, 0), Tile::MirrorDown),
            (IPoint::new(0, 1), Tile::SplitVert),
            (IPoint::new(1, 1), Tile::SplitHoriz),
            (IPoint::new(2, 1), Tile::Empty),
        ]);
        assert_eq!(expected, test.tiles);
        assert_eq!(3, test.width);
        assert_eq!(2, test.height);
        Ok(())
    }

    #[test]
    fn should_illuminate() -> Result<()> {
        let test = ".\\/-.\n.-/..".parse::<Contraption>()?;
        let expected = HashSet::from_iter(vec![
            IPoint::new(0, 0),
            IPoint::new(1, 0),
            IPoint::new(1, 1),
            IPoint::new(0, 1),
            IPoint::new(2, 1),
            IPoint::new(2, 0),
            IPoint::new(3, 0),
            IPoint::new(4, 0),
        ]);
        let beam = test.illuminate(IPoint::new(0, 0), Direction::Right);
        assert_eq!(expected, beam);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let contraption = EXAMPLE_INPUT.parse::<Contraption>()?;
        let beam = contraption.illuminate(IPoint::new(0, 0), Direction::Right);
        assert_eq!(46, beam.len());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let contraption = EXAMPLE_INPUT.parse::<Contraption>()?;
        let max = contraption.find_max_illumination();
        assert_eq!(51, max);
        Ok(())
    }
}
