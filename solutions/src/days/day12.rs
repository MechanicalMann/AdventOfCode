use crate::solver::Solver;
use anyhow::Result;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 12;

    const TITLE: &'static str = "Hill Climbing Algorithm";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<HeightMap>()?;
        map.find_easiest_path(&map.start)
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<HeightMap>()?;
        map.find_shortest_of_all_paths()
    }
}

struct HeightMap {
    nodes: HashMap<(isize, isize), u8>,
    start: (isize, isize),
    target: (isize, isize),
}
impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let mut start = (0, 0);
        let mut target = (0, 0);
        let mut max_height = 0;
        for (uy, line) in input.lines().enumerate() {
            let y = isize::try_from(uy)?;
            for (ux, height) in line.chars().enumerate() {
                let x = isize::try_from(ux)?;
                let val = match height {
                    'S' => {
                        start = (x, y);
                        1
                    }
                    'E' => {
                        target = (x, y);
                        u8::MAX
                    }
                    x => {
                        let h = (u8::try_from(x.to_digit(36).unwrap_or(9))?) - 9;
                        if h > max_height {
                            max_height = h;
                        }
                        h
                    }
                };
                nodes.insert((x, y), val);
            }
        }
        nodes.entry(target).and_modify(|e| *e = max_height + 1);
        Ok(HeightMap {
            nodes,
            start,
            target,
        })
    }
}
impl HeightMap {
    fn get_adjacent(&self, point: &(isize, isize)) -> Vec<&(isize, isize)> {
        let mut adjacent = vec![];
        let cur_height = self.nodes.get(point).unwrap();
        for (xa, ya) in [(0isize, -1isize), (1, 0), (0, 1), (-1, -0)] {
            let (x, y) = (point.0 - xa, point.1 - ya);
            if let Some((pos, height)) = self.nodes.get_key_value(&(x, y)) {
                if height <= &(cur_height + 1) {
                    adjacent.push(pos);
                }
            }
        }
        adjacent
    }

    fn find_easiest_path(&self, start: &(isize, isize)) -> Result<usize> {
        let mut frontier = BinaryHeap::from([Reverse((0, 0, start))]);
        let mut visited: HashMap<&(isize, isize), usize> = HashMap::new();

        while let Some(Reverse((_, cur_steps, pos))) = frontier.pop() {
            if pos == &self.target {
                return Ok(cur_steps);
            }
            if let Some(&seen) = visited.get(&pos) {
                if cur_steps > seen {
                    continue;
                }
            }
            for next in self.get_adjacent(&pos) {
                let total_steps = cur_steps + 1;
                let seen_steps = *visited.get(next).or(Some(&usize::MAX)).unwrap();
                if total_steps < seen_steps {
                    frontier.push(Reverse((
                        total_steps + distance_to(next, &self.target),
                        total_steps,
                        next,
                    )));
                    visited.insert(next, total_steps);
                }
            }
        }
        Err(anyhow!("No path found!"))
    }

    fn find_shortest_of_all_paths(&self) -> Result<usize> {
        let mut shortest = usize::MAX;

        for pos in self.nodes.iter().filter_map(|(k, h)| match &h {
            1 => Some(k),
            _ => None,
        }) {
            match self.find_easiest_path(pos) {
                Ok(steps) => {
                    if steps < shortest {
                        shortest = steps;
                    }
                }
                _ => (),
            }
        }
        match shortest == usize::MAX {
            true => Err(anyhow!("No viable path!")),
            false => Ok(shortest),
        }
    }
}

fn distance_to(source: &(isize, isize), target: &(isize, isize)) -> usize {
    ((target.0 - source.0).abs() + (target.1 - source.1).abs()) as usize
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    const EXAMPLE_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn should_parse() -> Result<()> {
        let input = "baS
cde
Egf";
        let map = input.parse::<HeightMap>()?;
        assert_eq!((2, 0), map.start);
        assert_eq!((0, 2), map.target);
        assert_eq!(9, map.nodes.len());
        Ok(())
    }

    #[test]
    fn should_find_shortest_path() -> Result<()> {
        let input = "baS
cde
Egf";
        let map = input.parse::<HeightMap>()?;
        let steps = map.find_easiest_path(&map.start)?;
        assert_eq!(8, steps);
        Ok(())
    }

    #[test]
    fn should_solve_part_1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<HeightMap>()?;
        let steps = map.find_easiest_path(&map.start)?;
        assert_eq!(31, steps);
        Ok(())
    }

    #[test]
    fn should_solve_part_2() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<HeightMap>()?;
        let steps = map.find_shortest_of_all_paths()?;
        assert_eq!(29, steps);
        Ok(())
    }
}
