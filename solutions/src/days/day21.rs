use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{common::IPoint, solver::Solver};
use anyhow::Result;

type Point = IPoint;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 21;
    const TITLE: &'static str = "Step Counter";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let garden = self.input().get_as::<Garden>()?;
        Ok(garden.walk(64))
    }

    fn part_two(&self) -> Result<usize> {
        let garden = self.input().get_as::<Garden>()?;
        Ok(garden.walk(26501365))
    }
}

const OFFSETS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 0 },
];

struct Garden {
    rocks: HashSet<Point>,
    start: Point,
    width: isize,
    height: isize,
}
impl FromStr for Garden {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut rocks = HashSet::new();
        let (mut width, mut height) = (0, 0);
        let mut start = Point::new(0, 0);

        for (y, line) in s.lines().enumerate() {
            height = y as isize;
            for (x, c) in line.chars().enumerate() {
                width = x as isize;
                match c {
                    '#' => {
                        rocks.insert(Point::new(width, height));
                    }
                    'S' => start = Point::new(width, height),
                    _ => (),
                }
            }
        }

        width += 1;
        height += 1;

        Ok(Self {
            rocks,
            start,
            width,
            height,
        })
    }
}
impl Garden {
    fn step(
        &self,
        start: &HashSet<Point>,
        visited: &mut HashMap<Point, HashSet<Point>>,
    ) -> HashSet<Point> {
        let mut steps = HashSet::new();
        for point in start {
            if let Some(pts) = visited.get(point) {
                steps.extend(pts);
                continue;
            }
            let mut history = HashSet::new();
            for offset in OFFSETS {
                let next = point + offset;
                if self.has_rock(next) {
                    continue;
                }
                history.insert(next);
            }
            steps.extend(&history);
            visited.insert(*point, history);
        }
        steps
    }

    fn has_rock(&self, point: Point) -> bool {
        let mut check = point;
        if check.x < 0 {
            check.x = (self.width + (check.x % self.width)) % self.width;
        } else {
            check.x = check.x % self.width;
        }
        if check.y < 0 {
            check.y = (self.height + (check.y % self.height)) % self.height;
        } else {
            check.y = check.y % self.height;
        }
        self.rocks.contains(&check)
    }

    fn get_start(&self) -> HashSet<Point> {
        HashSet::from_iter([self.start])
    }

    fn walk(&self, to: usize) -> usize {
        let mut steps = self.get_start();
        let mut visited = HashMap::new();
        let w = self.width as usize;
        let z = w / 2 + w % 2;
        let threshold = w + z;
        if to > threshold {
            let mut points = vec![];
            for i in 0..to {
                steps = self.step(&steps, &mut visited);
                if (i + z) % w == 0 {
                    let p = steps.len() as isize;
                    points.push(p);
                }
                if points.len() == 3 {
                    break;
                }
            }
            let (a, b, c) = solve_quadratic(points[0], points[1], points[2]);
            let x = ((to - z) / w) as isize;
            ((a * x.pow(2)) - (b * x) + c) as usize
        } else {
            for _ in 1..=to {
                steps = self.step(&steps, &mut visited);
            }
            steps.len()
        }
    }
}

fn solve_quadratic(x: isize, y: isize, z: isize) -> (isize, isize, isize) {
    let (mut a, mut b, mut c) = (x, y, z);
    b = ((-4 * a) + b) / -2;
    c = ((-9 * a) + 6 * b) + c;
    b += -c + (-c / 2);
    a += -b + -c;
    println!("{a}, {b}, {c}");
    (a as isize, b as isize, c as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "..#.#\n#.S..".parse::<Garden>()?;
        let expected_rocks =
            HashSet::from_iter([Point::new(2, 0), Point::new(4, 0), Point::new(0, 1)]);
        let expected_start = Point::new(2, 1);
        assert_eq!(expected_start, test.start);
        assert_eq!(expected_rocks, test.rocks);
        assert_eq!(5, test.width);
        assert_eq!(2, test.height);
        Ok(())
    }

    #[test]
    fn should_step() -> Result<()> {
        let test = "..#.#\n#.S..".parse::<Garden>()?;
        let steps = test.step(&test.get_start(), &mut HashMap::new());
        let expected_steps = HashSet::from_iter([Point::new(1, 1), Point::new(3, 1)]);
        assert_eq!(expected_steps, steps);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let garden = EXAMPLE_INPUT.parse::<Garden>()?;
        let steps = garden.walk(6);
        assert_eq!(16, steps);
        Ok(())
    }

    #[test]
    fn should_check_rocks() -> Result<()> {
        let test = "..#.#\n#.S..".parse::<Garden>()?;
        assert_eq!(true, test.has_rock(Point::new(2, 0)));
        assert_eq!(false, test.has_rock(Point::new(0, 0)));
        assert_eq!(true, test.has_rock(Point::new(2, 2)));
        assert_eq!(false, test.has_rock(Point::new(-2, 0)));
        assert_eq!(true, test.has_rock(Point::new(2, -4)));
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let garden = EXAMPLE_INPUT.parse::<Garden>()?;
        let steps = garden.walk(50);
        assert_eq!(1594, steps);
        Ok(())
    }
}
