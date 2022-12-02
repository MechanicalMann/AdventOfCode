use std::str::FromStr;

use anyhow::{anyhow, Ok};

use crate::solver::Solver;

pub struct Solution;
impl<'a> Solver<'a, usize, usize> for Solution {
    const DAY: u8 = 2;
    const TITLE: &'a str = "Rock Paper Scissors";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<usize> {
        let rounds = self.input().get_lines_as::<Round>()?;
        Ok(get_player_two_score(&rounds))
    }

    fn part_two(&self) -> anyhow::Result<usize> {
        Ok(0)
    }
}

fn get_player_two_score(rounds: &Vec<Round>) -> usize {
    rounds.iter().map(|round| round.score().1).sum()
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    fn beats(&self, other: &Hand) -> Outcome {
        match (self, other) {
            (Hand::Paper, Hand::Rock)
            | (Hand::Rock, Hand::Scissors)
            | (Hand::Scissors, Hand::Paper) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn score(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}
impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(anyhow!("Invalid hand")),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    player_one: Hand,
    player_two: Hand,
}
impl Round {
    fn score(&self) -> (usize, usize) {
        let mut score_one = self.player_one.score();
        let mut score_two = self.player_two.score();
        match self.player_one.beats(&self.player_two) {
            Outcome::Win => score_one += 6,
            Outcome::Draw => {
                score_one += 3;
                score_two += 3;
            }
            Outcome::Lose => score_two += 6,
        }
        (score_one, score_two)
    }
}
impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<_> = s.split(" ").collect();
        if hands.len() != 2 {
            return Err(anyhow!("Invalid round"));
        }
        let player_one = hands[0].parse::<Hand>().expect("Invalid player one hand");
        let player_two = hands[1].parse::<Hand>().expect("Invalid player two hand");
        Ok(Round {
            player_one,
            player_two,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn should_parse() {
        let input = "A Z";
        let expected = Round {
            player_one: Hand::Rock,
            player_two: Hand::Scissors,
        };
        let actual = input.parse::<Round>().expect("Parsing failed!");
        assert_eq!(expected, actual)
    }

    #[test]
    fn should_score() {
        let round1 = Round {
            player_one: Hand::Rock,
            player_two: Hand::Scissors,
        };
        assert_eq!((7, 3), round1.score());

        let round2 = Round {
            player_one: Hand::Rock,
            player_two: Hand::Paper,
        };
        assert_eq!((1, 8), round2.score());

        let round3 = Round {
            player_one: Hand::Paper,
            player_two: Hand::Paper,
        };
        assert_eq!((5, 5), round3.score());
    }

    #[test]
    fn should_solve_part_1() {
        let rounds: Vec<_> = EXAMPLE_INPUT
            .lines()
            .map(|x| x.parse::<Round>().unwrap())
            .collect();
        let score = get_player_two_score(&rounds);
        assert_eq!(15, score);
    }
}
