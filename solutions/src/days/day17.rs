use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

use crate::solver::Solver;

pub struct Solution;
impl Solver<isize, usize> for Solution {
    const DAY: u8 = 17;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<isize> {
        let target = self.input().get_as::<Target>()?;
        Ok(hit_with_style(&target).unwrap())
    }

    fn part_two(&self) -> anyhow::Result<usize> {
        let target = self.input().get_as::<Target>()?;
        Ok(hit_with_coverage(&target).unwrap())
    }
}

struct Target {
    bl: (isize, isize),
    tr: (isize, isize),
}
impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
        let m = re.captures(s).unwrap();
        let bl = (
            m.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            m.get(3).unwrap().as_str().parse::<isize>().unwrap(),
        );
        let tr = (
            m.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            m.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        );
        Ok(Target { bl, tr })
    }
}
impl Target {
    fn is_hit(&self, point: &(isize, isize)) -> bool {
        let &(x, y) = point;
        x >= self.bl.0 && x <= self.tr.0 && y >= self.bl.1 && y <= self.tr.1
    }

    fn is_miss(&self, point: &(isize, isize)) -> bool {
        let &(x, y) = point;
        x > self.tr.0 || y < self.bl.1
    }
}

fn launch_probe(vx: isize, vy: isize, at: &Target) -> Option<Vec<(isize, isize)>> {
    // Just like my least favorite part of ME2!
    // Launch a shitload of probes and see what sticks!
    let mut trajectory = vec![(0, 0)];
    let mut point = (0, 0);
    let mut vx = vx;
    let mut vy = vy;
    loop {
        let nx = point.0 + vx;
        let ny = point.1 + vy;
        point = (nx, ny);
        trajectory.push(point);

        if at.is_miss(&point) {
            return None;
        } else if at.is_hit(&point) {
            break;
        }

        vy -= 1;
        vx += if vx > 0 {
            -1
        } else if vx < 0 {
            1
        } else {
            0
        };
    }
    Some(trajectory)
}

fn hit_with_style(target: &Target) -> Option<isize> {
    let mut hits: Vec<Vec<(isize, isize)>> = vec![];
    // Some guesses
    let startx = 0;
    let starty = 0;
    let maxx = target.tr.0;
    let maxy = target.bl.1.abs();

    for vx in (startx..maxx).rev() {
        for vy in starty..maxy {
            if let Some(traj) = launch_probe(vx, vy, target) {
                hits.push(traj);
            }
        }
    }
    if hits.len() == 0 {
        None
    } else {
        Some(
            hits.iter()
                .map(|t| t.iter().map(|p| p.1).max().unwrap())
                .max()?,
        )
    }
}

// I *could* combine these two functions...
// ...
// or I could go to bed...
fn hit_with_coverage(target: &Target) -> Option<usize> {
    let mut hits: Vec<(isize, isize)> = vec![];

    let startx = 0;
    let starty = target.bl.1;
    let maxx = target.tr.0;
    let maxy = target.bl.1.abs();

    for vx in (startx..=maxx).rev() {
        for vy in starty..=maxy {
            if let Some(_) = launch_probe(vx, vy, target) {
                hits.push((vx, vy));
            }
        }
    }

    if hits.len() == 0 {
        None
    } else {
        Some(hits.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = input.parse::<Target>().unwrap();
        assert_eq!((20, -10), target.bl);
        assert_eq!((30, -5), target.tr);
    }

    #[test]
    fn should_detect_hit() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = input.parse::<Target>().unwrap();
        assert_eq!(true, target.is_hit(&(25, -7)));
        assert_ne!(true, target.is_hit(&(15, -4)));
    }

    #[test]
    fn should_detect_miss() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = input.parse::<Target>().unwrap();
        assert_eq!(true, target.is_miss(&(32, -12)));
        assert_ne!(true, target.is_miss(&(25, -7)));
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = input.parse::<Target>().unwrap();

        let max_y = hit_with_style(&target).unwrap();
        assert_eq!(45, max_y);
    }

    #[test]
    fn should_solve_part2_example() {
        let input = "target area: x=20..30, y=-10..-5";
        let target = input.parse::<Target>().unwrap();

        let max_y = hit_with_coverage(&target).unwrap();
        assert_eq!(112, max_y);
    }
}
