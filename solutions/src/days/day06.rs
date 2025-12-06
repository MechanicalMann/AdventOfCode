use std::str::FromStr;

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 06;
    const TITLE: &'static str = "Trash Compactor";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let homework = self.input().get_as::<Homework>()?;
        Ok(homework.grand_total())
    }

    fn part_two(&self) -> Result<usize> {
        let homework = self.input().get_as::<CephaloHomework>()?;
        Ok(homework.grand_total())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mult,
}
impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mult),
            _ => Err(anyhow!("Unknown operator")),
        }
    }
}
impl TryFrom<char> for Op {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mult),
            _ => Err(anyhow!("Unknown operator")),
        }
    }
}
impl Op {
    fn op(&self, l: usize, r: usize) -> usize {
        match self {
            Op::Add => l + r,
            Op::Mult => l * r,
        }
    }
}

#[derive(Debug)]
struct Homework {
    matrix: Vec<Vec<usize>>,
    operators: Vec<Op>,
    width: usize,
}
impl FromStr for Homework {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut matrix = vec![];
        let mut operators = vec![];
        let mut width = 0;
        for l in s.lines() {
            let split = l.split_whitespace().collect_vec();
            if width == 0 {
                width = split.len();
            } else if width != split.len() {
                return Err(anyhow!("Lines must contain the same number of columns"));
            }
            match split[0] {
                "+" | "*" => {
                    operators = split.iter().filter_map(|s| s.parse::<Op>().ok()).collect()
                }
                _ => matrix.push(
                    split
                        .iter()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect(),
                ),
            }
        }
        Ok(Self {
            matrix,
            operators,
            width,
        })
    }
}
impl Homework {
    fn solve(&self, col: usize) -> usize {
        if col >= self.width {
            panic!("Col {col} is outside column width");
        }
        let mut res = self.matrix[0][col];
        let op = self.operators[col];
        for row in 1..self.matrix.len() {
            res = op.op(res, self.matrix[row][col]);
        }
        res
    }

    fn grand_total(&self) -> usize {
        let mut total = 0;
        for i in 0..self.width {
            total += self.solve(i);
        }
        total
    }
}

#[derive(Debug)]
struct CephaloHomework {
    matrix: Vec<Vec<usize>>,
    operators: Vec<Op>,
}
impl FromStr for CephaloHomework {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut matrix = vec![];
        let mut operators = vec![];
        let lines = s
            .lines()
            .map(|s| s.chars().rev().collect_vec())
            .rev()
            .collect_vec();
        let mut values = vec![];
        for c in 0..lines[0].len() {
            let mut num = 0;
            let mut digit_count = 0;
            for r in 0..lines.len() {
                let d = lines[r][c];
                if r == 0 && d != ' ' {
                    operators.push(d.try_into()?);
                }
                if d.is_digit(10) {
                    num += (d.to_digit(10).unwrap() as usize) * 10usize.pow(digit_count);
                    digit_count += 1;
                }
            }
            if num == 0 {
                matrix.push(values.clone());
                values.clear();
            } else {
                values.push(num);
            }
        }
        matrix.push(values);
        Ok(Self { matrix, operators })
    }
}
impl CephaloHomework {
    fn solve(&self, row: usize) -> usize {
        if row >= self.matrix.len() {
            panic!("Row {row} is outside row height");
        }
        let mut res = self.matrix[row][0];
        let op = self.operators[row];
        for col in 1..self.matrix[row].len() {
            res = op.op(res, self.matrix[row][col]);
        }
        res
    }

    fn grand_total(&self) -> usize {
        let mut total = 0;
        for i in 0..self.matrix.len() {
            total += self.solve(i);
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "1 2  3\n* +  *".parse::<Homework>()?;
        assert_eq!(vec![vec![1, 2, 3]], test.matrix);
        assert_eq!(vec![Op::Mult, Op::Add, Op::Mult], test.operators);
        assert_eq!(3, test.width);
        Ok(())
    }

    #[test]
    fn should_solve() -> Result<()> {
        let test = "1 2  3\n4 5  6\n* +  *".parse::<Homework>()?;
        assert_eq!(4, test.solve(0));
        assert_eq!(7, test.solve(1));
        assert_eq!(18, test.solve(2));
        Ok(())
    }

    #[test]
    fn should_get_grand_total() -> Result<()> {
        let test = "1 2  3\n4 5  6\n* +  *".parse::<Homework>()?;
        assert_eq!(29, test.grand_total());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<Homework>()?;
        assert_eq!(4277556, test.grand_total());
        Ok(())
    }

    #[test]
    fn should_parse_cephalo() -> Result<()> {
        let test = "12 34\n56 7 \n*  + ".parse::<CephaloHomework>()?;
        assert_eq!(vec![vec![4, 37], vec![26, 15]], test.matrix);
        assert_eq!(vec![Op::Add, Op::Mult], test.operators);
        Ok(())
    }

    #[test]
    fn should_solve_cephalo() -> Result<()> {
        let test = "12 34\n56 7 \n*  + ".parse::<CephaloHomework>()?;
        assert_eq!(41, test.solve(0));
        assert_eq!(390, test.solve(1));
        Ok(())
    }

    #[test]
    fn should_get_grand_total_cephalo() -> Result<()> {
        let test = "12 34\n56 7 \n*  + ".parse::<CephaloHomework>()?;
        assert_eq!(431, test.grand_total());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<CephaloHomework>()?;
        assert_eq!(3263827, test.grand_total());
        Ok(())
    }
}
