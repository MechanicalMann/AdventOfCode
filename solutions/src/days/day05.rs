use std::str::FromStr;

use crate::{common::Range, solver::Solver};
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 05;
    const TITLE: &'static str = "Cafeteria";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let inventory = self.input().get()?.parse::<Inventory>()?;
        Ok(inventory.count_fresh())
    }

    fn part_two(&self) -> Result<usize> {
        let inventory = self.input().get()?.parse::<Inventory>()?;
        Ok(inventory.count_all_fresh())
    }
}

struct Inventory {
    fresh: Vec<Range>,
    ingredients: Vec<usize>,
}
impl FromStr for Inventory {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let halves = s.split("\n\n").collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!("Invalid inventory"));
        }
        let fresh = halves[0]
            .lines()
            .filter_map(|l| l.parse::<Range>().ok())
            .collect();
        let ingredients = halves[1]
            .lines()
            .filter_map(|l| l.parse::<usize>().ok())
            .collect();
        Ok(Self { fresh, ingredients })
    }
}
impl Inventory {
    fn is_fresh(&self, id: usize) -> bool {
        self.fresh.iter().any(|r| r.contains(id))
    }

    fn count_fresh(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|&&i| self.is_fresh(i))
            .count()
    }

    fn collapse_ranges(&self) -> Vec<Range> {
        let sorted = self
            .fresh
            .iter()
            .sorted_by(|a, b| a.min.cmp(&b.min))
            .collect_vec();
        let mut collapsed = vec![];
        let mut visited = vec![false; sorted.len()];
        for i in 0..sorted.len() {
            if visited[i] {
                continue;
            }
            let mut r = *sorted[i];
            for n in 0..sorted.len() {
                if i == n || visited[n] {
                    continue;
                }

                let next = sorted[n];
                if next.min >= r.min && next.max <= r.max {
                    visited[n] = true;
                    continue;
                }
                if next.min < r.min && next.max >= r.min {
                    r.min = next.min;
                    visited[n] = true;
                }
                if next.max > r.max && next.min <= r.max {
                    r.max = next.max;
                    visited[n] = true;
                }
            }
            collapsed.push(r);
            visited[i] = true;
        }
        collapsed
    }

    fn count_all_fresh(&self) -> usize {
        let mut total = 0;
        for r in &self.collapse_ranges() {
            total += r.max - r.min + 1;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "1-2\n3-4\n\n5\n6\n7\n8".parse::<Inventory>()?;
        assert_eq!(vec![Range::new(1, 2), Range::new(3, 4)], test.fresh);
        assert_eq!(vec![5, 6, 7, 8], test.ingredients);
        Ok(())
    }

    #[test]
    fn should_identify_fresh() -> Result<()> {
        let test = "1-2\n3-4\n\n5".parse::<Inventory>()?;
        assert!(test.is_fresh(2));
        Ok(())
    }

    #[test]
    fn should_count_fresh() -> Result<()> {
        let test = "1-2\n3-4\n\n2\n3\n4\n5".parse::<Inventory>()?;
        assert_eq!(3, test.count_fresh());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<Inventory>()?;
        assert_eq!(3, test.count_fresh());
        Ok(())
    }

    #[test]
    fn should_collapse_ranges() -> Result<()> {
        let test = "1-3\n2-5\n7-10\n8-12\n\n4".parse::<Inventory>()?;
        let collapsed = test.collapse_ranges();
        assert_eq!(vec![Range::new(1, 5), Range::new(7, 12)], collapsed);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let test = EXAMPLE_INPUT.parse::<Inventory>()?;
        assert_eq!(14, test.count_all_fresh());
        Ok(())
    }
}
