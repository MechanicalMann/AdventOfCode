use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::{anyhow, Result};

type Point = IPoint;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 17;
    const TITLE: &'static str = "Clumsy Crucible";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<CityMap>()?;
        map.find_path(
            Point::new(0, 0),
            Point::new(map.max_x, map.max_y),
            Direction::Down,
        )
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Down,
    Right,
    Left,
    Up,
}
impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }
}

const ADJACENTS: [((isize, isize), Direction); 4] = [
    ((0, 1), Direction::Down),
    ((1, 0), Direction::Right),
    ((-1, 0), Direction::Left),
    ((0, -1), Direction::Up),
];

struct CityMap {
    blocks: HashMap<Point, u8>,
    max_x: isize,
    max_y: isize,
}
impl FromStr for CityMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut blocks = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, line) in s.lines().enumerate() {
            max_y = y as isize;
            for (x, c) in line.chars().enumerate() {
                max_x = x as isize;
                blocks.insert(Point::new(x as isize, y as isize), (c as u8) % 48);
            }
        }
        Ok(CityMap {
            blocks,
            max_x,
            max_y,
        })
    }
}
impl CityMap {
    fn find_path(&self, start: Point, end: Point, dir: Direction) -> Result<usize> {
        let mut frontier = BinaryHeap::from([Reverse((0, 0, dir, start))]);
        let mut visited: HashMap<(Point, Direction, usize), usize> = HashMap::new();

        while let Some(Reverse((cur_loss, cur_steps, cur_dir, cur_pos))) = frontier.pop() {
            if cur_pos == end {
                return Ok(cur_loss);
            }
            for (next, next_loss, next_dir, next_steps) in
                self.get_next_nodes(cur_pos, cur_loss, cur_dir, cur_steps)
            {
                if !visited.contains_key(&(next, next_dir, next_steps)) {
                    frontier.push(Reverse((next_loss, next_steps, next_dir, next)));
                    visited.insert((next, next_dir, next_steps), next_loss);
                }
            }
        }
        Err(anyhow!("No valid path found!"))
    }

    fn get_next_nodes(
        &self,
        pos: Point,
        loss: usize,
        dir: Direction,
        steps: usize,
    ) -> Vec<(Point, usize, Direction, usize)> {
        let mut ret = vec![];
        for (offset, next_direction) in ADJACENTS {
            if next_direction == dir.opposite() {
                continue;
            }
            if next_direction == dir && steps == 3 {
                continue;
            }
            let next = pos + offset;
            let next_loss = match self.blocks.get(&next) {
                Some(&x) => x as usize,
                None => continue,
            };
            let next_steps = match dir == next_direction {
                true => steps + 1,
                false => 1,
            };
            ret.push((next, loss + next_loss, next_direction, next_steps));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "12\n34".parse::<CityMap>()?;
        let expected = HashMap::from_iter(vec![
            (Point::new(0, 0), 1),
            (Point::new(1, 0), 2),
            (Point::new(0, 1), 3),
            (Point::new(1, 1), 4),
        ]);
        assert_eq!(expected, test.blocks);
        Ok(())
    }

    #[test]
    fn should_navigate() -> Result<()> {
        let test = "14999\n23111\n99991".parse::<CityMap>()?;
        let heat_loss = test.find_path(Point::new(0, 0), Point::new(4, 2), Direction::Down)?;
        assert_eq!(11, heat_loss);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let city = EXAMPLE_INPUT.parse::<CityMap>()?;
        let heat_loss = city.find_path(
            Point::new(0, 0),
            Point::new(city.max_x, city.max_y),
            Direction::Right,
        )?;
        assert_eq!(102, heat_loss);
        Ok(())
    }
}
