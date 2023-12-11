use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::{anyhow, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 10;
    const TITLE: &'static str = "Pipe Maze";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let maze = self.input().get_as::<Maze>()?;
        Ok(maze.get_max_depth())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl From<(isize, isize)> for Point {
    fn from(value: (isize, isize)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

const CARDINALS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
}
impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::BendNE),
            'J' => Ok(Pipe::BendNW),
            '7' => Ok(Pipe::BendSW),
            'F' => Ok(Pipe::BendSE),
            _ => Err(anyhow!("Invalid pipe: {value}")),
        }
    }
}
impl Pipe {
    fn get_offsets(&self) -> (Point, Point) {
        match self {
            Pipe::Vertical => (Point::new(0, -1), Point::new(0, 1)),
            Pipe::Horizontal => (Point::new(-1, 0), Point::new(1, 0)),
            Pipe::BendNE => (Point::new(0, -1), Point::new(1, 0)),
            Pipe::BendNW => (Point::new(0, -1), Point::new(-1, 0)),
            Pipe::BendSE => (Point::new(0, 1), Point::new(1, 0)),
            Pipe::BendSW => (Point::new(0, 1), Point::new(-1, 0)),
        }
    }

    fn get_connecting(&self, start: Point) -> (Point, Point) {
        let (a, b) = self.get_offsets();
        (a + start, b + start)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}
impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::Start),
            _ => {
                let pipe = value.try_into()?;
                Ok(Tile::Pipe(pipe))
            }
        }
    }
}

struct Maze {
    tiles: HashMap<Point, Tile>,
    start: Point,
}
impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = HashMap::new();
        let mut start = Point::new(0, 0);
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::try_from(c)?;
                let coord = Point::new(x as isize, y as isize);
                tiles.insert(coord, tile);
                if tile == Tile::Start {
                    start = coord;
                }
            }
        }
        Ok(Maze { tiles, start })
    }
}
impl Maze {
    fn get_max_depth(&self) -> usize {
        let mut depth = 0;
        let mut seen: HashSet<Point> = HashSet::new();
        let mut positions = vec![];
        // Get starting branches
        for offset in CARDINALS {
            let test = self.start + offset;
            if let Some(Tile::Pipe(p)) = self.tiles.get(&test) {
                let (a, b) = p.get_connecting(test);
                if a == self.start || b == self.start {
                    positions.push((test, *p));
                }
            }
        }
        if positions.len() != 2 {
            panic!("Starting point must have two connecting pipes!");
        }
        loop {
            depth += 1;
            let mut new_pos = vec![];
            for &(pos, pipe) in &positions {
                seen.insert(pos);
                let (a, b) = pipe.get_connecting(pos);
                for next in [a, b] {
                    if seen.contains(&next) {
                        continue;
                    }
                    if let Some(Tile::Pipe(p)) = self.tiles.get(&next) {
                        new_pos.push((next, *p));
                    }
                }
            }
            if new_pos.len() == 0 {
                break;
            }
            positions.clear();
            positions.append(&mut new_pos);
        }
        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn should_parse() -> Result<()> {
        let test = ".S-.".parse::<Maze>()?;
        let expected = HashMap::from_iter(vec![
            (Point::new(0, 0), Tile::Ground),
            (Point::new(1, 0), Tile::Start),
            (Point::new(2, 0), Tile::Pipe(Pipe::Horizontal)),
            (Point::new(3, 0), Tile::Ground),
        ]);
        assert_eq!(expected, test.tiles);
        assert_eq!(Point::new(1, 0), test.start);
        Ok(())
    }

    #[test]
    fn should_get_depth() -> Result<()> {
        let test = ".-S--".parse::<Maze>()?;
        assert_eq!(2, test.get_max_depth());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let maze = EXAMPLE_INPUT.parse::<Maze>()?;
        assert_eq!(8, maze.get_max_depth());
        Ok(())
    }
}
