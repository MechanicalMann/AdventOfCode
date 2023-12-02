use crate::solver::Solver;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 2;
    const TITLE: &'static str = "Cube Conundrum";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let constraints = vec![(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)];
        let games = self.input().get_lines_as::<Game>()?;
        Ok(sum_possible(&games, &constraints))
    }

    fn part_two(&self) -> Result<usize> {
        let games = self.input().get_lines_as::<Game>()?;
        Ok(sum_power(&games))
    }
}

lazy_static! {
    static ref GAME_ID: Regex = Regex::new(r"Game (\d+): ").unwrap();
    static ref REVEAL_CUBES: Regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cube {
    Red,
    Green,
    Blue,
}

struct Game {
    id: usize,
    required: HashMap<Cube, usize>,
}
impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let gid_cap = GAME_ID.captures(s).ok_or(anyhow!("Invalid game"))?;
        let id = gid_cap[1].parse::<usize>()?;
        let mut required = HashMap::new();
        for cubes in REVEAL_CUBES.captures_iter(s) {
            let color = match &cubes[2] {
                "red" => Cube::Red,
                "green" => Cube::Green,
                "blue" => Cube::Blue,
                _ => {
                    return Err(anyhow!("Invalid color"));
                }
            };
            let num = cubes[1].parse::<usize>()?;
            let cur = required.entry(color).or_insert(0);
            if &num > cur {
                *cur = num;
            }
        }
        Ok(Game { id, required })
    }
}
impl Game {
    fn is_possible(&self, constraints: &[(Cube, usize)]) -> bool {
        for con in constraints {
            if self.required.get(&con.0).or(Some(&0)).unwrap() > &con.1 {
                return false;
            }
        }
        true
    }

    fn power(&self) -> usize {
        self.required.values().product()
    }
}

fn sum_possible(games: &[Game], constraints: &[(Cube, usize)]) -> usize {
    games
        .iter()
        .filter_map(|g| {
            if g.is_possible(&constraints) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}

fn sum_power(games: &[Game]) -> usize {
    games.iter().map(|g| g.power()).sum()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "Game 1: 1 red, 2 green; 3 blue; 4 red";
        let game = test.parse::<Game>()?;
        assert_eq!(Some(&4), game.required.get(&Cube::Red));
        assert_eq!(Some(&2), game.required.get(&Cube::Green));
        assert_eq!(Some(&3), game.required.get(&Cube::Blue));
        Ok(())
    }

    #[test]
    fn should_be_possible() -> Result<()> {
        let test = "Game 1: 1 red, 2 green; 3 blue; 4 red";
        let game = test.parse::<Game>()?;
        let constraints = vec![(Cube::Red, 5)];
        assert_eq!(true, game.is_possible(&constraints));
        Ok(())
    }

    #[test]
    fn should_be_impossible() -> Result<()> {
        let test = "Game 1: 1 red, 2 green; 3 blue; 4 red";
        let game = test.parse::<Game>()?;
        let constraints = vec![(Cube::Red, 1)];
        assert_eq!(false, game.is_possible(&constraints));
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let constraints = vec![(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)];
        let games = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Game>().ok())
            .collect_vec();
        let sum = sum_possible(&games, &constraints);
        assert_eq!(8, sum);
        Ok(())
    }

    #[test]
    fn should_get_power() -> Result<()> {
        let test = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = test.parse::<Game>()?;
        assert_eq!(48, game.power());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let games = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Game>().ok())
            .collect_vec();
        assert_eq!(2286, sum_power(&games));
        Ok(())
    }
}
