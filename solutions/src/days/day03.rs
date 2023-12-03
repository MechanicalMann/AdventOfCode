use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 3;
    const TITLE: &'static str = "Gear Ratios";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let schematic = self.input().get_as::<Schematic>()?;
        let parts = schematic.get_part_numbers();
        Ok(parts.iter().sum())
    }

    fn part_two(&self) -> Result<usize> {
        let schematic = self.input().get_as::<Schematic>()?;
        let ratios = schematic.get_gear_ratios();
        Ok(ratios.iter().sum())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

struct Schematic {
    numbers: Vec<(Point, Point, usize)>,
    symbols: HashSet<Point>,
    gears: HashSet<Point>,
}
impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut gears: HashSet<Point> = HashSet::new();
        let mut symbols: HashSet<Point> = HashSet::new();
        let mut numbers: Vec<(Point, Point, usize)> = vec![];
        let mut cur_num: Vec<usize> = vec![];
        let mut cur_start: Option<Point> = None;
        let mut cur_stop: Option<Point> = None;

        for (_y, row) in s.lines().enumerate() {
            let y = isize::try_from(_y)?;
            if matches!((cur_start, cur_stop), (Some(_), Some(_))) {
                let num = digits_to_number(&cur_num);
                numbers.push((cur_start.unwrap(), cur_stop.unwrap(), num));
                cur_num.clear();
                cur_start = None;
                cur_stop = None;
            }
            for (_x, col) in row.chars().enumerate() {
                let x = isize::try_from(_x)?;
                match col {
                    '0'..='9' => {
                        cur_num.push(col.to_digit(10).unwrap().try_into()?);
                        if cur_start.is_none() {
                            cur_start = Some(Point { x, y });
                        }
                        cur_stop = Some(Point { x, y });
                    }
                    _ => {
                        match (cur_start, cur_stop) {
                            (Some(start), Some(stop)) => {
                                let num = digits_to_number(&cur_num);
                                numbers.push((start, stop, num));
                                cur_num.clear();
                                cur_start = None;
                                cur_stop = None;
                            }
                            (None, None) => (),
                            _ => return Err(anyhow!("Invalid number")),
                        }
                        match col {
                            '.' => continue,
                            '*' => {
                                gears.insert(Point { x, y });
                            }
                            _ => (),
                        }
                        symbols.insert(Point { x, y });
                    }
                }
            }
        }
        Ok(Schematic {
            numbers,
            symbols,
            gears,
        })
    }
}
impl Schematic {
    fn get_part_numbers(&self) -> Vec<usize> {
        let mut parts = vec![];
        for &(start, stop, n) in self.numbers.iter() {
            for a in get_adjacencies(start, stop).iter() {
                if self.symbols.contains(&a) {
                    parts.push(n);
                    break;
                }
            }
        }
        parts
    }

    fn get_gear_ratios(&self) -> Vec<usize> {
        let mut geared: HashMap<Point, (usize, usize)> = HashMap::new();
        for &(start, stop, n) in self.numbers.iter() {
            for &a in get_adjacencies(start, stop).iter() {
                if self.gears.contains(&a) {
                    let (count, ratio) = geared.entry(a).or_insert((0, 1));
                    *count += 1;
                    *ratio *= n;
                }
            }
        }
        geared
            .iter()
            .filter_map(|(_, &(c, v))| if c == 2 { Some(v) } else { None })
            .collect_vec()
    }
}

fn digits_to_number(digits: &[usize]) -> usize {
    let mut num = 0;
    for i in 0..digits.len() {
        num += digits[i] * 10usize.pow((digits.len() - 1 - i).try_into().unwrap());
    }
    num
}

fn get_adjacencies(start: Point, stop: Point) -> Vec<Point> {
    let mut ret = vec![];
    // Since all numbers are one row, we only need to loop for top & bottom
    for x in start.x - 1..=stop.x + 1 {
        ret.push(Point::new(x, start.y - 1));
        ret.push(Point::new(x, start.y + 1));
    }
    ret.push(Point::new(start.x - 1, start.y));
    ret.push(Point::new(stop.x + 1, stop.y));
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "..123..*..";
        let expected_num = vec![(Point::new(2, 0), Point::new(4, 0), 123)];
        let mut expected_sym = HashSet::new();
        expected_sym.insert(Point::new(7, 0));
        let schematic = test.parse::<Schematic>()?;
        assert_eq!(1, schematic.numbers.len());
        assert_eq!(1, schematic.symbols.len());
        assert_eq!(expected_num, schematic.numbers);
        assert_eq!(expected_sym, schematic.symbols);
        Ok(())
    }

    #[test]
    fn should_get_adjacent() -> Result<()> {
        let expected = vec![
            Point::new(0, 0),
            Point::new(0, 2),
            Point::new(1, 0),
            Point::new(1, 2),
            Point::new(2, 0),
            Point::new(2, 2),
            Point::new(3, 0),
            Point::new(3, 2),
            Point::new(4, 0),
            Point::new(4, 2),
            Point::new(0, 1),
            Point::new(4, 1),
        ];
        let actual = get_adjacencies(Point::new(1, 1), Point::new(3, 1));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn should_get_part_numbers() -> Result<()> {
        let test = "..$123..45";
        let schematic = test.parse::<Schematic>()?;
        let expected = vec![123];
        assert_eq!(expected, schematic.get_part_numbers());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let schematic = EXAMPLE_INPUT.parse::<Schematic>()?;
        let parts = schematic.get_part_numbers();
        assert_eq!(4361usize, parts.iter().sum());
        Ok(())
    }

    #[test]
    fn should_get_ratios() -> Result<()> {
        let test = "..12*3..";
        let schematic = test.parse::<Schematic>()?;
        assert_eq!(vec![36], schematic.get_gear_ratios());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let schematic = EXAMPLE_INPUT.parse::<Schematic>()?;
        let ratios = schematic.get_gear_ratios();
        assert_eq!(467835usize, ratios.iter().sum());
        Ok(())
    }
}
