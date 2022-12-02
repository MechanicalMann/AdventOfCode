use std::{str::FromStr, string::ParseError};

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl<'a> Solver<'a, usize, usize> for Solution {
    const DAY: u8 = 1;
    const TITLE: &'a str = "Counting Calories";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let data = self.input().get_as::<ElfParty>()?;
        Ok(get_max_calories(&data))
    }

    fn part_two(&self) -> Result<usize> {
        let data = self.input().get_as::<ElfParty>()?;
        Ok(get_top_three_calories(&data))
    }
}

fn get_max_calories(party: &ElfParty) -> usize {
    party
        .food_packs
        .iter()
        .map(|p| p.rations.iter().sum())
        .max()
        .unwrap()
}

fn get_top_three_calories(party: &ElfParty) -> usize {
    party
        .food_packs
        .iter()
        .map(|p| p.rations.iter().sum::<usize>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

struct FoodPack {
    rations: Vec<usize>,
}
impl FromStr for FoodPack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<FoodPack, Self::Err> {
        let rations: Vec<_> = s.lines().map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(FoodPack { rations })
    }
}

struct ElfParty {
    food_packs: Vec<FoodPack>,
}
impl FromStr for ElfParty {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<ElfParty, Self::Err> {
        let food_packs: Vec<_> = s
            .split("\n\n")
            .map(|x| x.parse::<FoodPack>().unwrap())
            .collect();
        Ok(ElfParty { food_packs })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn should_parse() {
        let input = "123
456
789";
        let parsed = input.parse::<FoodPack>().expect("Parsing failed!");
        assert!(itertools::equal(vec![123, 456, 789], parsed.rations));
    }

    #[test]
    fn should_get_max() {
        let party = ElfParty {
            food_packs: vec![
                FoodPack {
                    rations: vec![1, 2, 3],
                },
                FoodPack {
                    rations: vec![4, 5, 6],
                },
            ],
        };
        let res = get_max_calories(&party);
        assert_eq!(15, res);
    }

    #[test]
    fn should_solve_part_1() {
        let party = EXAMPLE_INPUT.parse::<ElfParty>().unwrap();
        let res = get_max_calories(&party);
        assert_eq!(24000, res)
    }

    #[test]
    fn should_get_top_three() {
        let party = ElfParty {
            food_packs: vec![
                FoodPack {
                    rations: vec![1, 2],
                },
                FoodPack {
                    rations: vec![3, 4],
                },
                FoodPack {
                    rations: vec![5, 6],
                },
                FoodPack {
                    rations: vec![7, 8],
                },
            ],
        };
        let res = get_top_three_calories(&party);
        assert_eq!(33, res);
    }

    #[test]
    fn should_solve_part_2() {
        let party = EXAMPLE_INPUT.parse::<ElfParty>().unwrap();
        let res = get_top_three_calories(&party);
        assert_eq!(45000, res);
    }
}
