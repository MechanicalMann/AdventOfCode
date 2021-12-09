use std::{collections::HashSet, str::FromStr};

use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 9;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let map = AdventInput::for_day(DAY).get_as::<Map>()?;
        Ok(map.get_total_risk())
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let map = AdventInput::for_day(DAY).get_as::<Map>()?;
        Ok(0)
    }
}

struct Map {
    points: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Vec<u8>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect();
        let width = points[0].len();
        let height = points.len();
        Ok(Map {
            points,
            width,
            height,
        })
    }
}
impl Map {
    fn get_adjacent(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut adjacent: Vec<(usize, usize)> = vec![];
        for (x, y) in [(0isize, -1isize), (1, 0), (0, 1), (-1, 0)] {
            if point.0 == 0 && x == -1 || point.1 == 0 && y == -1 {
                continue;
            }
            if point.0 == self.width - 1 && x == 1 || point.1 == self.height - 1 && y == 1 {
                continue;
            }
            let a = (
                (point.0 as isize + x) as usize,
                (point.1 as isize + y) as usize,
            );
            adjacent.push(a);
        }
        adjacent
    }

    fn get_low_points(&self) -> Vec<(usize, usize)> {
        let mut lows: Vec<(usize, usize)> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                let point = (x, y);
                if self.is_low(&point) {
                    lows.push(point);
                }
            }
        }
        lows
    }

    fn is_low(&self, point: &(usize, usize)) -> bool {
        let &(x, y) = point;
        let val = self.points[y][x];
        for (ax, ay) in self.get_adjacent(point) {
            if self.points[ay][ax] <= val {
                return false;
            }
        }
        true
    }

    fn get_total_risk(&self) -> usize {
        let mut risk = 0;
        for (x, y) in self.get_low_points() {
            risk += 1 + self.points[y][x] as usize;
        }
        risk
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "111
101
111";
        let map = input.parse::<Map>().unwrap();
        assert_eq!(3, map.width);
        assert_eq!(3, map.height);
        assert_eq!(
            vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]],
            map.points
        )
    }

    #[test]
    fn should_get_all_adjacent() {
        let input = "111
101
111";
        let map = input.parse::<Map>().unwrap();
        let adjacent = map.get_adjacent(&(1, 1));
        assert_eq!(4, adjacent.len());
        assert_eq!(vec![(1, 0), (2, 1), (1, 2), (0, 1)], adjacent)
    }

    #[test]
    fn should_get_corner_adjacent() {
        let input = "111
101
111";
        let map = input.parse::<Map>().unwrap();
        let adjacent = map.get_adjacent(&(0, 0));
        assert_eq!(2, adjacent.len());
        assert_eq!(vec![(1, 0), (0, 1)], adjacent)
    }

    #[test]
    fn should_get_edge_adjacent() {
        let input = "111
101
111";
        let map = input.parse::<Map>().unwrap();
        let adjacent = map.get_adjacent(&(0, 1));
        assert_eq!(3, adjacent.len());
        assert_eq!(vec![(0, 0), (1, 1), (0, 2)], adjacent)
    }

    #[test]
    fn should_get_low_points() {
        let input = "111
102
121";
        let map = input.parse::<Map>().unwrap();
        let lows = map.get_low_points();
        assert_eq!(vec![(1, 1), (2, 2)], lows);
    }

    #[test]
    fn should_get_total_risk() {
        let input = "111
102
121";
        let map = input.parse::<Map>().unwrap();
        let risk = map.get_total_risk();
        assert_eq!(3, risk);
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = input.parse::<Map>().unwrap();
        let risk = map.get_total_risk();
        assert_eq!(15, risk);
    }
}
