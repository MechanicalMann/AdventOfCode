use std::{collections::HashSet, str::FromStr};

use crate::solver::Solver;
use anyhow::{bail, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 13;

    fn part_one(&self) -> Result<usize> {
        let mut input = self.input().get_as::<Input>().unwrap();
        input.sheet.fold(input.folds[0]);
        Ok(input.sheet.dots.len())
    }

    fn part_two(&self) -> Result<usize> {
        let mut input = self.input().get_as::<Input>().unwrap();
        for f in input.folds {
            input.sheet.fold(f);
        }
        println!("Day 13 Part 02:");
        input.sheet.print();
        Ok(0)
    }

    fn print_part_two<T: std::fmt::Display>(&self, _: T) {}

    fn new() -> Self {
        Solution {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}
impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let dir = &split.next().expect("Invalid fold, expected x or y")[11..];
        let amt = split
            .next()
            .expect("Invalid fold, expected position")
            .parse::<usize>()
            .expect("Invalid fold, expected numeric position");
        match dir {
            "x" => Ok(Fold::Vertical(amt)),
            "y" => Ok(Fold::Horizontal(amt)),
            _ => bail!("Invalid fold"),
        }
    }
}

struct Input {
    sheet: Sheet,
    folds: Vec<Fold>,
}
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sections = s.split("\n\n").collect::<Vec<_>>();
        let dots = sections[0]
            .lines()
            .map(|l| l.split(',').collect::<Vec<_>>())
            .map(|s| {
                (
                    s[0].parse::<usize>().unwrap(),
                    s[1].parse::<usize>().unwrap(),
                )
            });
        let folds = sections[1].lines().map(|l| l.parse::<Fold>().unwrap());
        Ok(Input {
            sheet: Sheet {
                dots: HashSet::from_iter(dots),
            },
            folds: folds.collect(),
        })
    }
}

struct Sheet {
    dots: HashSet<(usize, usize)>,
}
impl Sheet {
    fn fold(&mut self, fold: Fold) {
        let mut folded = HashSet::new();
        for &(x, y) in self.dots.iter() {
            let (dim, amt) = match fold {
                Fold::Horizontal(a) => (y, a),
                Fold::Vertical(a) => (x, a),
            };
            if dim > amt {
                let newpos = amt - (dim - amt);
                match fold {
                    Fold::Horizontal(_) => folded.insert((x, newpos)),
                    Fold::Vertical(_) => folded.insert((newpos, y)),
                };
            } else if dim < amt {
                folded.insert((x, y));
            }
        }
        self.dots = folded;
    }

    fn print(&self) {
        let max_x = self.dots.iter().map(|&d| d.0).max().unwrap();
        let max_y = self.dots.iter().map(|&d| d.1).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                print!(
                    "{}",
                    match self.dots.get(&(x, y)) {
                        Some(_) => '#',
                        None => ' ',
                    }
                );
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn should_parse() {
        let test = "1,1
2,2
3,3

fold along x=2";
        let input = test.parse::<Input>().unwrap();
        assert_eq!(3, input.sheet.dots.len());
        assert_eq!(1, input.folds.len());
        assert_eq!(
            vec![(1, 1), (2, 2), (3, 3)],
            input.sheet.dots.into_iter().sorted().collect::<Vec<_>>()
        );
        assert_eq!(vec![Fold::Vertical(2)], input.folds);
    }

    #[test]
    fn should_fold_y() {
        let test = "1,1
2,2
3,3

fold along y=2";
        let mut input = test.parse::<Input>().unwrap();
        input.sheet.fold(input.folds[0]);
        assert_eq!(2, input.sheet.dots.len());
        assert_eq!(
            vec![(1, 1), (3, 1)],
            input.sheet.dots.into_iter().sorted().collect::<Vec<_>>()
        );
    }

    #[test]
    fn should_fold_x() {
        let test = "1,1
2,2
3,3

fold along x=2";
        let mut input = test.parse::<Input>().unwrap();
        input.sheet.fold(input.folds[0]);
        assert_eq!(2, input.sheet.dots.len());
        assert_eq!(
            vec![(1, 1), (1, 3)],
            input.sheet.dots.into_iter().sorted().collect::<Vec<_>>()
        );
    }

    #[test]
    fn should_solve_part1_example() {
        let test = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let mut input = test.parse::<Input>().unwrap();
        input.sheet.fold(input.folds[0]);
        assert_eq!(17, input.sheet.dots.len());
    }
}
