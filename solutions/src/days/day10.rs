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

struct PipeLoop {
    the_loop: HashSet<Point>,
    depth: usize,
    min: (isize, isize),
    max: (isize, isize),
    start_pipe: Pipe,
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
    fn get_loop(&self) -> PipeLoop {
        let mut depth = 0;
        let mut the_loop: HashSet<Point> = HashSet::new();
        let mut positions = vec![];
        let mut offsets = vec![];
        let mut min_x = self.start.x;
        let mut min_y = self.start.y;
        let mut max_x = self.start.x;
        let mut max_y = self.start.y;

        // Get starting branches
        the_loop.insert(self.start);
        for offset in CARDINALS {
            let test = self.start + offset;
            if let Some(Tile::Pipe(p)) = self.tiles.get(&test) {
                let (a, b) = p.get_connecting(test);
                if a == self.start || b == self.start {
                    positions.push((test, *p));
                    offsets.push(offset);
                }
            }
        }

        let start_pipe = match (offsets[0], offsets[1]) {
            (Point { x: 0, y: -1 }, Point { x: 0, y: 1 }) => Pipe::Vertical,
            (Point { x: 0, y: -1 }, Point { x: -1, y: 0 }) => Pipe::BendNW,
            (Point { x: 0, y: -1 }, Point { x: 1, y: 0 }) => Pipe::BendNE,
            (Point { x: 1, y: 0 }, Point { x: 0, y: 1 }) => Pipe::BendSE,
            (Point { x: 1, y: 0 }, Point { x: -1, y: 0 }) => Pipe::Horizontal,
            (Point { x: 0, y: 1 }, Point { x: -1, y: 0 }) => Pipe::BendSW,
            _ => Pipe::Horizontal,
        };

        if positions.len() != 2 {
            panic!("Starting point must have two connecting pipes!");
        }
        loop {
            depth += 1;
            let mut new_pos = vec![];
            for &(pos, pipe) in &positions {
                the_loop.insert(pos);
                if pos.x < min_x {
                    min_x = pos.x;
                }
                if pos.y < min_y {
                    min_y = pos.y;
                }
                if pos.x > max_x {
                    max_x = pos.x;
                }
                if pos.y > max_y {
                    max_y = pos.y;
                }
                let (a, b) = pipe.get_connecting(pos);
                for next in [a, b] {
                    if the_loop.contains(&next) {
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
        PipeLoop {
            the_loop,
            depth,
            min: (min_x, min_y),
            max: (max_x, max_y),
            start_pipe,
        }
    }

    fn get_max_depth(&self) -> usize {
        self.get_loop().depth
    }

    fn get_interior_tiles(&self) -> usize {
        let PipeLoop {
            depth: _,
            the_loop,
            min: (min_x, min_y),
            max: (max_x, max_y),
            start_pipe,
        } = self.get_loop();

        let mut interior = 0;
        for y in 0..(self.height as isize) {
            let mut out = true;
            for x in 0..(self.width as isize) {
                let pos = Point::new(x, y);
                if the_loop.contains(&pos) {
                    let pipe = if pos == self.start {
                        start_pipe
                    } else {
                        // we need to treat the starting tile as its equivalent pipe to count correctly
                        match self.tiles.get(&pos) {
                            Some(&Tile::Pipe(p)) => p,
                            _ => panic!("wtf"),
                        }
                    };
                    match pipe {
                        // only count connections from above to elsewhere (below/side)
                        Pipe::Vertical | Pipe::BendNE | Pipe::BendNW => out = !out,
                        _ => (),
                    }
                } else if !out && min_x < x && x < max_x && min_y < y && y < max_y {
                    interior += 1;
                }
            }
        }
        interior
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
    const EXAMPLE_INPUT_PART2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

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
        assert_eq!(10, maze.get_interior_tiles());
        Ok(())
    }
}
