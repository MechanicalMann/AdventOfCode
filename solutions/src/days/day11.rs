use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 11;
    const TITLE: &'static str = "Cosmic Expansion";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        let expanded = map.expand(2);
        Ok(expanded.get_min_distances())
    }

    fn part_two(&self) -> Result<usize> {
        let map = self.input().get_as::<Map>()?;
        let expanded = map.expand(1_000_000);
        Ok(expanded.get_min_distances())
    }
}

struct Map {
    galaxies: Vec<(isize, isize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}
impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut galaxies = vec![];
        let mut rows_empty = HashMap::new();
        let mut cols_empty = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut empty_space = true;
                if c == '#' {
                    galaxies.push((x as isize, y as isize));
                    empty_space = false;
                }
                rows_empty
                    .entry(y)
                    .and_modify(|a| *a = *a && empty_space)
                    .or_insert(empty_space);
                cols_empty
                    .entry(x)
                    .and_modify(|a| *a = *a && empty_space)
                    .or_insert(empty_space);
            }
        }
        let empty_rows = rows_empty
            .iter()
            .filter(|(_, &v)| v)
            .map(|(&k, _)| k)
            .sorted()
            .collect_vec();
        let empty_cols = cols_empty
            .iter()
            .filter(|(_, &v)| v)
            .map(|(&k, _)| k)
            .sorted()
            .collect_vec();
        Ok(Map {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}
impl Map {
    fn expand(&self, factor: usize) -> Map {
        let mut galaxies = vec![];
        let mut row_expansions = HashMap::new();
        let mut col_expansions = HashMap::new();
        for g in &self.galaxies {
            let expand_x = *col_expansions.entry(g.0).or_insert(
                (self
                    .empty_cols
                    .iter()
                    .filter(|&v| v < &(g.0 as usize))
                    .count()
                    * (factor - 1)) as isize,
            );
            let expand_y = *row_expansions.entry(g.1).or_insert(
                (self
                    .empty_rows
                    .iter()
                    .filter(|&v| v < &(g.1 as usize))
                    .count()
                    * (factor - 1)) as isize,
            );
            galaxies.push((g.0 + expand_x, g.1 + expand_y));
        }
        Map {
            galaxies,
            empty_rows: vec![],
            empty_cols: vec![],
        }
    }

    fn get_min_distances(&self) -> usize {
        let mut total_min_distance: usize = 0;
        for i in 0..self.galaxies.len() - 1 {
            let (lx, ly) = self.galaxies[i];
            for j in i + 1..self.galaxies.len() {
                let (rx, ry) = self.galaxies[j];
                let distance: usize = ((lx - rx).abs() + (ly - ry).abs()).try_into().unwrap();
                total_min_distance += distance;
            }
        }
        total_min_distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "..#..".parse::<Map>()?;
        let expected_galaxies = vec![(2, 0)];
        let expected_cols = vec![0, 1, 3, 4];
        let expected_rows: Vec<usize> = vec![];
        assert_eq!(expected_galaxies, test.galaxies);
        assert_eq!(expected_rows, test.empty_rows);
        assert_eq!(expected_cols, test.empty_cols);
        Ok(())
    }

    #[test]
    fn should_expand() -> Result<()> {
        let test = ".#.".parse::<Map>()?;
        assert_eq!(vec![(1, 0)], test.galaxies);
        let expanded = test.expand(2);
        assert_eq!(vec![(2, 0)], expanded.galaxies);
        Ok(())
    }

    #[test]
    fn should_get_distance() -> Result<()> {
        let test = ".#..\n....\n...#".parse::<Map>()?;
        assert_eq!(4, test.get_min_distances());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<Map>()?;
        let expanded = map.expand(2);
        assert_eq!(374, expanded.get_min_distances());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let map = EXAMPLE_INPUT.parse::<Map>()?;
        let expanded = map.expand(100);
        assert_eq!(8410, expanded.get_min_distances());
        Ok(())
    }
}
