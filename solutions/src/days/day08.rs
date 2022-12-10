use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 8;
    const TITLE: &'static str = "Treetop Tree House";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let grove = self.input().get_as::<Grove>()?;
        Ok(grove.find_visible().len())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}
impl From<(usize, usize)> for Point {
    fn from(source: (usize, usize)) -> Self {
        Point {
            x: source.0,
            y: source.1,
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

#[derive(Debug, PartialEq)]
struct Grove {
    trees: HashMap<Point, u32>,
    size: Point,
}
impl Grove {
    fn find_visible(&self) -> HashSet<Point> {
        let mut visible = HashSet::new();
        let (max_x, max_y) = (self.size.x - 1, self.size.y - 1);
        // Rows
        for y in 0..self.size.y {
            let (mut fwd_max, mut rev_max) = (0, 0);
            for fwd_x in 0..self.size.x {
                let rev_x = max_x - fwd_x;
                let (fwd_pt, rev_pt): (Point, Point) = ((fwd_x, y).into(), (rev_x, y).into());
                let fwd_height = self.trees.get(&fwd_pt).unwrap();
                let rev_height = self.trees.get(&rev_pt).unwrap();
                // Edges are always visible
                let mut edge = false;
                if fwd_x == 0 || y == 0 {
                    visible.insert(fwd_pt);
                    fwd_max = *fwd_height;
                    edge = true;
                }
                if rev_x == max_x || y == max_y {
                    visible.insert(rev_pt);
                    rev_max = *rev_height;
                    edge = true;
                }
                if edge {
                    continue;
                }
                if fwd_height > &fwd_max {
                    visible.insert(fwd_pt);
                    fwd_max = *fwd_height;
                }
                if rev_height > &rev_max {
                    visible.insert(rev_pt);
                    rev_max = *rev_height;
                }
            }
        }
        // Cols
        for x in 0..self.size.x {
            let (mut fwd_max, mut rev_max) = (0, 0);
            for fwd_y in 0..self.size.y {
                let rev_y = max_y - fwd_y;
                let (fwd_pt, rev_pt) = ((x, fwd_y), (x, rev_y));
                let fwd_height = self.trees.get(&fwd_pt.into()).unwrap();
                let rev_height = self.trees.get(&rev_pt.into()).unwrap();
                if fwd_height > &fwd_max {
                    visible.insert((x, fwd_y).into());
                    fwd_max = *fwd_height;
                }
                if rev_height > &rev_max {
                    visible.insert((x, rev_y).into());
                    rev_max = *rev_height;
                }
            }
        }
        visible
    }
}
impl FromStr for Grove {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = HashMap::new();
        let (mut max_x, mut max_y) = (0, 0);
        for (y, l) in s.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                trees.insert((x, y).into(), c.to_digit(10).unwrap());
                if y == 0 {
                    max_x += 1;
                }
            }
            max_y += 1;
        }
        Ok(Grove {
            trees,
            size: (max_x, max_y).into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn should_parse() {
        let test = "123
456
789";
        let expected: HashMap<Point, u32> = HashMap::from_iter([
            ((0, 0).into(), 1),
            ((1, 0).into(), 2),
            ((2, 0).into(), 3),
            ((0, 1).into(), 4),
            ((1, 1).into(), 5),
            ((2, 1).into(), 6),
            ((0, 2).into(), 7),
            ((1, 2).into(), 8),
            ((2, 2).into(), 9),
        ]);

        let actual = test.parse::<Grove>().unwrap();
        assert_eq!(expected, actual.trees);
    }

    #[test]
    fn should_get_visible() {
        let test = "11111
10001
10201
10001
11111";
        let expected = HashSet::from_iter([
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 4, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 4, y: 2 },
            Point { x: 0, y: 3 },
            Point { x: 4, y: 3 },
            Point { x: 0, y: 4 },
            Point { x: 1, y: 4 },
            Point { x: 2, y: 4 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 4 },
        ]);
        let actual = test.parse::<Grove>().unwrap().find_visible();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_solve_part_1() {
        let actual = EXAMPLE_INPUT.parse::<Grove>().unwrap();
        let visible = actual.find_visible();
        assert_eq!(21, visible.len());
    }
}
