use std::{collections::HashMap, str::FromStr};

use crate::{common::lcm, solver::Solver};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 8;
    const TITLE: &'static str = "Haunted Wasteland";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        Ok(map.travel())
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        Ok(map.spooky_travel())
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    left: String,
    right: String,
}

lazy_static! {
    static ref NODE_RE: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}

struct Map {
    directions: Vec<Direction>,
    network: HashMap<String, Node>,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split("\n\n").collect_vec();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid map"));
        }
        let directions = parts[0]
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect_vec();

        let mut network = HashMap::new();
        for line in parts[1].lines() {
            for (_, [id, left, right]) in NODE_RE.captures_iter(line).map(|cap| cap.extract()) {
                network.insert(
                    id.to_string(),
                    Node {
                        left: left.to_string(),
                        right: right.to_string(),
                    },
                );
            }
        }

        Ok(Map {
            directions,
            network,
        })
    }
}
impl Map {
    fn travel(&self) -> usize {
        let mut step_count = 0;
        let mut instr = 0;
        let mut cur = self.network.get("AAA").unwrap();
        loop {
            step_count += 1;
            let id = match &self.directions[instr] {
                Direction::Left => &cur.left,
                Direction::Right => &cur.right,
            };
            if id == "ZZZ" {
                break;
            }
            cur = self.network.get(id).unwrap();
            instr = (instr + 1) % self.directions.len();
        }
        step_count
    }

    // Not as spooky as the flashbacks to the bus-timetable problem...
    fn spooky_travel(&self) -> usize {
        let mut step_count = 0;
        let mut instr = 0;
        let mut cur = self
            .network
            .iter()
            .filter_map(|(k, v)| if k.ends_with('A') { Some(v) } else { None })
            .collect_vec();
        let mut dists = vec![];
        loop {
            step_count += 1;
            let dir = &self.directions[instr];
            let ids = cur
                .iter()
                .map(|n| match dir {
                    Direction::Left => &n.left,
                    Direction::Right => &n.right,
                })
                .collect_vec();
            cur.clear();
            for id in ids {
                if id.ends_with('Z') {
                    dists.push(step_count);
                } else {
                    cur.push(self.network.get(id).unwrap());
                }
            }
            if cur.len() == 0 {
                break;
            }
            instr = (instr + 1) % self.directions.len();
        }
        lcm(&dists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_PART1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    const EXAMPLE_INPUT_PART2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "RL\n\nAAA = (BBB, CCC)".parse::<Map>()?;
        let nodes: HashMap<String, Node> = HashMap::from([(
            "AAA".to_string(),
            Node {
                left: "BBB".to_string(),
                right: "CCC".to_string(),
            },
        )]);
        assert_eq!(vec![Direction::Right, Direction::Left], test.directions);
        assert_eq!(nodes, test.network);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = EXAMPLE_INPUT_PART1.parse::<Map>()?;
        let steps = test.travel();
        assert_eq!(6, steps);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let test = EXAMPLE_INPUT_PART2.parse::<Map>()?;
        let steps = test.spooky_travel();
        assert_eq!(6, steps);
        Ok(())
    }
}
