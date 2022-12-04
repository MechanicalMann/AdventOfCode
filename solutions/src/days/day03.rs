use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::Result;

const ITEMS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

lazy_static! {
    static ref PRIORITIES: HashMap<char, usize> =
        HashMap::from_iter(ITEMS.chars().enumerate().map(|(i, c)| (c, i + 1)));
}

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 3;
    const TITLE: &'static str = "Rucksack Reorganization";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let sacks = self.input().get_lines_as::<Rucksack>()?;
        Ok(get_total_priority(&sacks))
    }

    fn part_two(&self) -> Result<usize> {
        let sacks = self.input().get_lines_as::<Rucksack>()?;
        get_total_badge_priority(&sacks)
    }
}

fn get_common_item(sack: &Rucksack) -> Option<char> {
    let mut one = sack.compartment_one.chars();
    let two: HashSet<char> = HashSet::from_iter(sack.compartment_two.chars());
    one.find(|c| two.contains(c))
}

fn get_total_priority(sacks: &Vec<Rucksack>) -> usize {
    sacks
        .iter()
        .filter_map(get_common_item)
        .filter_map(|c| PRIORITIES.get(&c))
        .sum()
}

fn get_badge(sacks: &[Rucksack]) -> Result<Option<char>> {
    if sacks.len() < 2 {
        return Err(anyhow!("Must have more than one sack!"));
    }
    let (first, rest) = sacks.split_first().unwrap();
    let mut one = first.total.chars();
    let many: Vec<HashSet<char>> = rest
        .iter()
        .map(|s| HashSet::from_iter(s.total.chars()))
        .collect();
    Ok(one.find(|c| many.iter().all(|s| s.contains(c))))
}

fn get_total_badge_priority(sacks: &[Rucksack]) -> Result<usize> {
    if sacks.len() % 3 != 0 {
        return Err(anyhow!("Invalid sack grouping!"));
    }
    let groups = sacks.len() / 3;
    let mut total = 0;
    for i in 0..groups {
        let start = i * 3;
        match get_badge(&sacks[start..start + 3]) {
            Ok(Some(c)) => total += PRIORITIES.get(&c).unwrap(),
            Ok(None) => continue,
            Err(_) => return Err(anyhow!("Invalid group")),
        }
    }
    Ok(total)
}

#[derive(Debug, PartialEq)]
struct Rucksack {
    total: String,
    compartment_one: String,
    compartment_two: String,
}
impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || s.len() % 2 != 0 {
            return Err(anyhow!("Invalid rucksack length"));
        }
        let sack_size = s.len() / 2; // 😏️
        let (one, two) = s.split_at(sack_size);
        Ok(Rucksack {
            total: s.to_string(),
            compartment_one: one.to_string(),
            compartment_two: two.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn should_parse() {
        let input = "aabb";
        let sack = input.parse::<Rucksack>().unwrap();
        assert_eq!("aa", sack.compartment_one);
        assert_eq!("bb", sack.compartment_two);
    }

    #[test]
    fn should_not_parse() {
        let empty = "";
        let one = "a";
        let three = "aaa";
        assert!(empty.parse::<Rucksack>().is_err());
        assert!(one.parse::<Rucksack>().is_err());
        assert!(three.parse::<Rucksack>().is_err());
    }

    #[test]
    fn should_find_common_item() {
        let sack = "abac".parse::<Rucksack>().unwrap();
        let res = get_common_item(&sack);
        assert!(matches!(res, Some('a')));
    }

    #[test]
    fn should_get_total_priority() {
        let sacks = vec![
            "abac".parse::<Rucksack>().unwrap(),
            "bcbd".parse::<Rucksack>().unwrap(),
        ];
        let priority = get_total_priority(&sacks);
        assert_eq!(3, priority);
    }

    #[test]
    fn should_solve_part_1() {
        let sacks: Vec<_> = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Rucksack>().unwrap())
            .collect();
        let priority = get_total_priority(&sacks);
        assert_eq!(157, priority);
    }

    #[test]
    fn should_get_badge() {
        let sacks = vec![
            "abcd".parse::<Rucksack>().unwrap(),
            "defg".parse::<Rucksack>().unwrap(),
        ];
        let badge = get_badge(&sacks).unwrap().unwrap();
        assert_eq!('d', badge);
    }

    #[test]
    fn should_get_badge_total() {
        let sacks = vec![
            "ab".parse::<Rucksack>().unwrap(),
            "ac".parse::<Rucksack>().unwrap(),
            "ad".parse::<Rucksack>().unwrap(),
            "AB".parse::<Rucksack>().unwrap(),
            "AC".parse::<Rucksack>().unwrap(),
            "AD".parse::<Rucksack>().unwrap(),
        ];
        let priority = get_total_badge_priority(&sacks).unwrap();
        assert_eq!(28, priority);
    }

    #[test]
    fn should_solve_part_2() {
        let sacks: Vec<_> = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Rucksack>().unwrap())
            .collect();
        let priority = get_total_badge_priority(&sacks).unwrap();
        assert_eq!(70, priority);
    }
}
