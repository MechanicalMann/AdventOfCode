use crate::{common::IPoint, solver::Solver};
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 09;
    const TITLE: &'static str = "Movie Theater";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let rects = parse(&self.input().get()?)?;
        Ok(biggest_rectangle(&rects))
    }

    fn part_two(&self) -> Result<isize> {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
struct Rect {
    a: IPoint,
    b: IPoint,
    area: isize,
}
impl Rect {
    fn from(a: IPoint, b: IPoint) -> Self {
        let diff = a - b;
        let area = (diff.x.abs() + 1) * (diff.y.abs() + 1);
        Self { a, b, area }
    }
}

fn parse(s: &str) -> Result<Vec<Rect>> {
    let mut rects = vec![];
    let lines = s.lines().collect_vec();
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            rects.push(Rect::from(lines[i].parse()?, lines[j].parse()?));
        }
    }
    Ok(rects)
}

fn biggest_rectangle(rects: &[Rect]) -> isize {
    rects
        .iter()
        .map(|r| r.area)
        .max()
        .expect("Expected more than zero rectangles")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn should_get_area() {
        let rect = Rect::from(IPoint::new(0, 3), IPoint::new(9, 0));
        assert_eq!(40, rect.area);
    }

    #[test]
    fn should_parse() -> Result<()> {
        let rects = parse("1,2\n3,4")?;
        assert_eq!(1, rects.len());
        assert_eq!(Rect::from(IPoint::new(1, 2), IPoint::new(3, 4)), rects[0]);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = parse(EXAMPLE_INPUT)?;
        assert_eq!(50, biggest_rectangle(&test));
        Ok(())
    }
}
