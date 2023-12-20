use std::str::FromStr;

use crate::{
    common::{Direction, IPoint},
    solver::Solver,
};
use anyhow::{anyhow, Result};
use itertools::Itertools;

type Point = IPoint;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 18;
    const TITLE: &'static str = "Lavaduct Lagoon";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let plan = self.input().get_as::<DigPlan>()?;
        Ok(plan.get_volume())
    }

    fn part_two(&self) -> Result<isize> {
        let plan = DigPlan::from_bad_str(&self.input().get().unwrap())?;
        Ok(plan.get_volume())
    }
}

struct Instruction {
    direction: Direction,
    meters: isize,
    #[allow(dead_code)] // keeping for posterity
    color: (u8, u8, u8),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split(' ').collect_vec();
        if parts.len() != 3 {
            return Err(anyhow!("Invalid instruction: {s}"));
        }
        let direction = parts[0].parse()?;
        let meters = parts[1].parse()?;
        let r = u8::from_str_radix(&parts[2][2..4], 16)?;
        let g = u8::from_str_radix(&parts[2][4..6], 16)?;
        let b = u8::from_str_radix(&parts[2][6..8], 16)?;

        Ok(Instruction {
            direction,
            meters,
            color: (r, g, b),
        })
    }
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction: {s}")),
        }
    }
}
impl Instruction {
    fn from_bad_str(s: &str) -> Result<Self> {
        let i = match s.find('#') {
            Some(idx) => idx,
            None => return Err(anyhow!("Invalid instruction: {s}")),
        };
        let meters = isize::from_str_radix(&s[i + 1..i + 6], 16)?;
        let direction = match &s[i + 6..i + 7] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => return Err(anyhow!("Invalid direction: {s}")),
        };
        Ok(Instruction {
            direction,
            meters,
            color: (0, 0, 0),
        })
    }
}

struct DigPlan {
    instructions: Vec<Instruction>,
}
impl FromStr for DigPlan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let instructions = s
            .lines()
            .filter_map(|l| l.parse::<Instruction>().ok())
            .collect_vec();
        Ok(DigPlan { instructions })
    }
}
impl DigPlan {
    fn from_bad_str(s: &str) -> Result<Self> {
        let instructions = s
            .lines()
            .filter_map(|l| Instruction::from_bad_str(l).ok())
            .collect_vec();
        Ok(DigPlan { instructions })
    }

    fn dig(&self) -> Vec<Point> {
        let mut vertices = vec![];
        let mut cur = Point::new(0, 0);
        for ins in &self.instructions {
            cur = match ins.direction {
                Direction::Up => cur + (0, -ins.meters),
                Direction::Down => cur + (0, ins.meters),
                Direction::Left => cur + (-ins.meters, 0),
                Direction::Right => cur + (ins.meters, 0),
            };
            vertices.push(cur);
        }
        vertices
    }

    fn get_volume(&self) -> isize {
        let vertices = self.dig();
        let mut area = 0;
        let mut perim = 0;
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            area += vertices[i].x * vertices[j].y;
            area -= vertices[i].y * vertices[j].x;
            perim += (vertices[i].x - vertices[j].x).abs() + (vertices[i].y - vertices[j].y).abs();
        }
        perim = (perim / 2) + 1;
        area = (area / 2).abs();
        area + perim
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "R 3 (#042069)".parse::<Instruction>()?;
        assert_eq!(Direction::Right, test.direction);
        assert_eq!(3, test.meters);
        assert_eq!((4, 32, 105), test.color);
        Ok(())
    }

    #[test]
    fn should_get_volume() -> Result<()> {
        let test = "R 3 (#000000)\nD 2 (#000000)\nR 1 (#000000)\nD 2 (#000000)\nL 4 (#000000)\nU 4 (#000000)".parse::<DigPlan>()?;
        let volume = test.get_volume();
        assert_eq!(23, volume);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let plan = EXAMPLE_INPUT.parse::<DigPlan>()?;
        assert_eq!(62, plan.get_volume());
        Ok(())
    }

    #[test]
    fn should_parse_part2() -> Result<()> {
        let test = Instruction::from_bad_str("U 2 (#069420)")?;
        assert_eq!(Direction::Right, test.direction);
        assert_eq!(26946, test.meters);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let plan = DigPlan::from_bad_str(EXAMPLE_INPUT)?;
        assert_eq!(952408144115, plan.get_volume());
        Ok(())
    }
}
