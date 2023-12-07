use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 7;
    const TITLE: &'static str = "Camel Cards";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let hands = self.input().get_lines_as::<Hand>()?;
        Ok(get_winnings(&hands))
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

fn card_from(c: &char) -> Result<u8> {
    match c {
        '2' => Ok(2),
        '3' => Ok(3),
        '4' => Ok(4),
        '5' => Ok(5),
        '6' => Ok(6),
        '7' => Ok(7),
        '8' => Ok(8),
        '9' => Ok(9),
        'T' => Ok(10),
        'J' => Ok(11),
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        _ => Err(anyhow!("Invalid card")),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
}
impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let halves = s.split(' ').collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!("Invalid hand"));
        }
        let cards = halves[0]
            .chars()
            .filter_map(|c| card_from(&c).ok())
            .collect();
        let bid = halves[1].parse()?;
        Ok(Hand { cards, bid })
    }
}
impl Hand {
    fn score(&self) -> u8 {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for &c in &self.cards {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut sets: HashMap<u8, u8> = HashMap::new();
        for (_, &v) in counts.iter() {
            *sets.entry(v).or_insert(0) += 1;
        }
        if sets.contains_key(&5) {
            return 7;
        }
        if sets.contains_key(&4) {
            return 6;
        }
        if sets.contains_key(&3) {
            if sets.contains_key(&2) {
                return 5;
            }
            return 4;
        }
        if sets.contains_key(&2) {
            if sets[&2] == 2 {
                return 3;
            }
            return 2;
        }
        return 1;
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (l, r) = (self.score(), other.score());
        if l > r {
            return Ordering::Greater;
        } else if l < r {
            return Ordering::Less;
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                if a > b {
                    return Ordering::Greater;
                } else if a < b {
                    return Ordering::Less;
                }
            }
        }
        Ordering::Equal
    }
}

fn get_winnings(hands: &[Hand]) -> usize {
    let mut rank = 1;
    let mut winnings = 0;
    for hand in hands.iter().sorted() {
        winnings += hand.bid * rank;
        rank += 1;
    }
    winnings
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "57TJA 123";
        let hand = test.parse::<Hand>()?;
        assert_eq!(123, hand.bid);
        assert_eq!(vec![5, 7, 10, 11, 14], hand.cards);
        Ok(())
    }

    #[test]
    fn should_score() -> Result<()> {
        let a = "23456 1".parse::<Hand>()?;
        assert_eq!(1, a.score());

        let b = "22345 1".parse::<Hand>()?;
        assert_eq!(2, b.score());

        let c = "22334 1".parse::<Hand>()?;
        assert_eq!(3, c.score());

        let d = "22234 1".parse::<Hand>()?;
        assert_eq!(4, d.score());

        let e = "22233 1".parse::<Hand>()?;
        assert_eq!(5, e.score());

        let f = "22223 1".parse::<Hand>()?;
        assert_eq!(6, f.score());

        let g = "22222 1".parse::<Hand>()?;
        assert_eq!(7, g.score());

        Ok(())
    }

    #[test]
    fn should_sort() -> Result<()> {
        let a = "23456 1".parse::<Hand>()?;
        let b = "22345 1".parse::<Hand>()?;
        let c = "22334 1".parse::<Hand>()?;
        let d = "22335 1".parse::<Hand>()?;
        let mut test: Vec<&Hand> = vec![&b, &d, &a, &c];
        test.sort();
        assert_eq!(vec![&a, &b, &c, &d], test);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let hands = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Hand>().ok())
            .collect_vec();
        let winnings = get_winnings(&hands);
        assert_eq!(6440, winnings);
        Ok(())
    }
}
