use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 13;
    const TITLE: &'static str = "Point of Incidence";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let patches = self.input().get_grouped_as::<Patch>()?;
        Ok(summarize(&patches))
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ground {
    Ash,
    Rock,
}
impl TryFrom<char> for Ground {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(Ground::Ash),
            '#' => Ok(Ground::Rock),
            _ => Err(anyhow!("Invalid ground cover: {value}")),
        }
    }
}

struct Patch {
    points: HashMap<Point, Ground>,
    max_x: usize,
    max_y: usize,
}
impl FromStr for Patch {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut points = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in s.lines().enumerate() {
            max_y = y;
            for (x, char) in line.chars().enumerate() {
                max_x = x;
                let g = char.try_into()?;
                points.insert(Point::new(x, y), g);
            }
        }
        Ok(Patch {
            points,
            max_x,
            max_y,
        })
    }
}
impl Patch {
    fn get_reflection(&self) -> Option<(Orientation, usize)> {
        // Vertical reflection check
        if let Some(x) = reflection_check(self, self.max_x, Self::compare_cols) {
            return Some((Orientation::Vertical, x));
        }

        // Horizontal reflection check
        if let Some(y) = reflection_check(self, self.max_y, Self::compare_rows) {
            return Some((Orientation::Horizontal, y));
        }
        None
    }

    fn compare_cols(&self, xl: usize, xr: usize) -> bool {
        self.compare_ranges(xl, xr, true)
    }

    fn compare_rows(&self, yt: usize, yb: usize) -> bool {
        self.compare_ranges(yt, yb, false)
    }

    fn compare_ranges(&self, a: usize, b: usize, vertical: bool) -> bool {
        let get_points = |c: usize| match vertical {
            true => (Point::new(a, c), Point::new(b, c)),
            false => (Point::new(c, a), Point::new(c, b)),
        };
        let max = match vertical {
            true => self.max_y,
            false => self.max_x,
        };
        for z in 0..=max {
            let (pos_a, pos_b) = get_points(z);
            let left = self.points.get(&pos_a).unwrap();
            let right = self.points.get(&pos_b).unwrap();
            if left != right {
                return false;
            }
        }
        true
    }
}

fn reflection_check(
    patch: &Patch,
    max: usize,
    comparator: fn(&Patch, usize, usize) -> bool,
) -> Option<usize> {
    let mut reflection: Option<usize> = None;
    let comp = |a, b| comparator(patch, a, b);
    for a in 0..max {
        let b = a + 1;
        let mut reflected = true;
        if !comp(a, b) {
            reflected = false;
        }
        if reflected {
            let (mut exa, mut exb) = (a, b);
            loop {
                if exa == 0 || exb == max {
                    break;
                }
                exa -= 1;
                exb += 1;
                if !comp(exa, exb) {
                    reflected = false;
                    break;
                }
            }
            if reflected {
                reflection = Some(a + 1);
                break;
            }
        }
    }
    reflection
}

fn summarize(patches: &[Patch]) -> usize {
    patches
        .iter()
        .filter_map(|p| p.get_reflection())
        .map(|(o, v)| match o {
            Orientation::Horizontal => 100 * v,
            Orientation::Vertical => v,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn should_parse() -> Result<()> {
        let test = ".#.".parse::<Patch>()?;
        let expected = HashMap::from_iter(vec![
            (Point::new(0, 0), Ground::Ash),
            (Point::new(1, 0), Ground::Rock),
            (Point::new(2, 0), Ground::Ash),
        ]);
        assert_eq!(expected, test.points);
        Ok(())
    }

    #[test]
    fn should_get_reflection() -> Result<()> {
        let vertical = "#.##.\n..##.".parse::<Patch>()?;
        assert_eq!(Some((Orientation::Vertical, 3)), vertical.get_reflection());
        let horizontal = "#.#.\n###.\n###.\n#.#.".parse::<Patch>()?;
        assert_eq!(
            Some((Orientation::Horizontal, 2)),
            horizontal.get_reflection()
        );
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let patches = EXAMPLE_INPUT
            .split("\n\n")
            .filter_map(|s| s.parse::<Patch>().ok())
            .collect_vec();
        let summary = summarize(&patches);
        assert_eq!(405, summary);
        Ok(())
    }
}
