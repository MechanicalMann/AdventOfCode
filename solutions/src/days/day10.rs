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
        let maze = self.input().get_as::<Maze>()?;
        Ok(maze.get_interior_tiles())
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
    width: usize,
    height: usize,
}
impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = HashMap::new();
        let mut start = Point::new(0, 0);
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                let tile = Tile::try_from(c)?;
                let coord = Point::new(x as isize, y as isize);
                tiles.insert(coord, tile);
                if tile == Tile::Start {
                    start = coord;
                }
            }
        }
        Ok(Maze {
            tiles,
            start,
            width,
            height,
        })
    }
}
impl Maze {
    fn get_loop_and_depth(&self) -> (usize, HashSet<Point>) {
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
        (depth, seen)
    }

    fn get_max_depth(&self) -> usize {
        self.get_loop_and_depth().0
    }

    fn get_interior_tiles(&self) -> usize {
        let the_loop = self.get_loop_and_depth().1;
        let mut top_x: HashMap<usize, usize> = HashMap::new();
        let mut left_y: HashMap<usize, usize> = HashMap::new();
        let mut bot_x: HashMap<usize, usize> = HashMap::new();
        let mut right_y: HashMap<usize, usize> = HashMap::new();

        let mut potentials: HashMap<Point, (usize, usize, usize, usize)> = HashMap::new();

        for ya in 0..self.height {
            let yb = self.height - 1 - ya;
            let cyl = left_y.entry(ya).or_insert(0);
            let cyr = right_y.entry(yb).or_insert(0);
            for xa in 0..self.width {
                let xb = self.width - 1 - xa;
                let cxt = top_x.entry(xa).or_insert(0);
                let cxb = bot_x.entry(xb).or_insert(0);

                let pos_a = Point::new(xa as isize, ya as isize);
                if the_loop.contains(&pos_a) {
                    let tile = self.tiles.get(&pos_a).unwrap();
                    match tile {
                        Tile::Pipe(p) => match p {
                            Pipe::Horizontal => *cxt += 1,
                            Pipe::Vertical => *cyl += 1,
                            _ => (),
                        },
                        _ => (),
                    }
                } else {
                    let counts = potentials.entry(pos_a).or_insert((0, 0, 0, 0));
                    counts.0 = *cxt;
                    counts.1 = *cyl;
                }

                let pos_b = Point::new(xb as isize, yb as isize);
                if the_loop.contains(&pos_b) {
                    match self.tiles.get(&pos_b).unwrap() {
                        Tile::Pipe(p) => match p {
                            Pipe::Horizontal => *cxb += 1,
                            Pipe::Vertical => *cyr += 1,
                            _ => (),
                        },
                        _ => (),
                    }
                } else {
                    let counts = potentials.entry(pos_b).or_insert((0, 0, 0, 0));
                    counts.2 = *cxb;
                    counts.3 = *cyr;
                }
            }
        }
        potentials
            .values()
            .filter(|(a, b, c, d)| a > &0 && b > &0 && c > &0 && d > &0)
            .filter(|(a, b, c, d)| a % 2 == 1 || b % 2 == 1 || c % 2 == 1 || d % 2 == 1)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    const EXAMPLE_INPUT_PART2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

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
        let maze = EXAMPLE_INPUT_PART1.parse::<Maze>()?;
        assert_eq!(8, maze.get_max_depth());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let maze = EXAMPLE_INPUT_PART2.parse::<Maze>()?;
        assert_eq!(4, maze.get_interior_tiles());
        Ok(())
    }
}
