use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

use crate::input::AdventInput;

const DAY: u8 = 5;

pub mod part1 {
    use super::*;
    pub fn solve() -> Result<usize> {
        let input = AdventInput::for_day(DAY).get_lines()?;
        let data: Vec<_> = input
            .iter()
            .map(|l| l.parse::<Line>().unwrap())
            .filter(|&l| l.0 .0 == l.1 .0 || l.0 .1 == l.1 .1)
            .collect();
        let overlaps = get_intersections(&data)?;
        Ok(overlaps.len())
    }
}

pub mod part2 {
    use super::*;
    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get_lines_as::<Line>()?;
        let overlaps = get_intersections(&data)?;
        Ok(overlaps.len())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point(usize, usize);
impl FromStr for Point {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = input
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Ok(Point(coords[0], coords[1]))
    }
}

#[derive(Clone, Copy, Debug)]
struct Line(Point, Point);
impl FromStr for Line {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points: Vec<_> = input
            .split(" -> ")
            .map(|x| x.parse::<Point>().unwrap())
            .collect();
        Ok(Line(points[0], points[1]))
    }
}
impl Line {
    fn iter_points(self) -> LineIterator {
        LineIterator {
            line: self,
            cur: self.0,
            done: false,
        }
    }
}

struct LineIterator {
    line: Line,
    cur: Point,
    done: bool,
}
impl Iterator for LineIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.cur == self.line.1 {
            self.done = true;
            Some(self.cur)
        } else {
            let result = Some(self.cur);
            if self.line.0 .0 != self.line.1 .0 {
                if self.line.0 .0 > self.line.1 .0 {
                    self.cur.0 -= 1;
                } else {
                    self.cur.0 += 1;
                }
            }
            if self.line.0 .1 != self.line.1 .1 {
                if self.line.0 .1 > self.line.1 .1 {
                    self.cur.1 -= 1;
                } else {
                    self.cur.1 += 1;
                }
            }
            result
        }
    }
}

fn get_intersections(lines: &[Line]) -> Result<Vec<Point>> {
    let total = lines.len();
    if total < 2 {
        return Ok(vec![]);
    }
    let mut points: HashSet<Point> = HashSet::new();
    let mut overlaps: HashSet<Point> = HashSet::new();
    for l in lines {
        for p in l.iter_points() {
            if !points.insert(p) {
                overlaps.insert(p);
            }
        }
    }
    Ok(Vec::from_iter(overlaps))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse_point() -> Result<()> {
        let input = "1,2";
        let point = input.parse::<Point>()?;
        assert_eq!(1, point.0);
        assert_eq!(2, point.1);
        Ok(())
    }

    #[test]
    fn should_parse_line() -> Result<()> {
        let input = "1,2 -> 3,4";
        let line = input.parse::<Line>()?;
        assert_eq!(Point(1, 2), line.0);
        assert_eq!(Point(3, 4), line.1);
        Ok(())
    }

    #[test]
    fn should_get_points_in_line() {
        let input = "4,0 -> 0,0";
        let line = input.parse::<Line>().unwrap();
        let expected = vec![
            Point(4, 0),
            Point(3, 0),
            Point(2, 0),
            Point(1, 0),
            Point(0, 0),
        ];
        let actual: Vec<_> = line.iter_points().collect();
        assert_eq!(expected.len(), actual.len());
        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn should_get_intersection() {
        let line1 = "0,1 -> 2,1".parse::<Line>().unwrap();
        let line2 = "1,0 -> 1,2".parse::<Line>().unwrap();
        let intersections = get_intersections(&vec![line1, line2]).unwrap();
        assert_eq!(1, intersections.len());
        assert_eq!(Point(1, 1), intersections[0]);
    }

    #[test]
    fn should_get_overlap() {
        let line1 = "1,1 -> 5,1".parse::<Line>().unwrap();
        let line2 = "0,1 -> 3,1".parse::<Line>().unwrap();
        let expected = vec![Point(1, 1), Point(2, 1), Point(3, 1)];
        let mut actual = get_intersections(&vec![line1, line2]).unwrap();
        assert_eq!(expected.len(), actual.len());
        actual.sort_by(|a, b| a.0.cmp(&b.0)); // Hashsets don't preserve order
        for (e, a) in expected.iter().zip(actual.iter()) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn should_solve_example() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines: Vec<_> = input
            .lines()
            .map(|l| l.parse::<Line>().unwrap())
            .filter(|l| l.0 .0 == l.1 .0 || l.0 .1 == l.1 .1) // Example specifies only horizontal and vertical lines
            .collect();
        let intersections = get_intersections(&lines).unwrap();
        assert_eq!(5, intersections.len());
    }
}
