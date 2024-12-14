use std::str::FromStr;

use crate::{common::IPoint, solver::Solver};
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 13;
    const TITLE: &'static str = "Claw Contraption";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let machines = self.input().get_grouped_as::<ClawMachine>()?;
        Ok(machines.iter().filter_map(|m| m.cost_to_win(0)).sum())
    }

    fn part_two(&self) -> Result<isize> {
        let machines = self.input().get_grouped_as::<ClawMachine>()?;
        Ok(machines
            .iter()
            .filter_map(|m| m.cost_to_win(PRIZE_OFFSET))
            .sum())
    }
}

type Vector = IPoint;

lazy_static! {
    static ref BUTTONS: Regex = Regex::new(r"Button (A|B): X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZES: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}
const A_COST: isize = 3;
const B_COST: isize = 1;
const PRIZE_OFFSET: isize = 10_000_000_000_000;

struct ClawMachine {
    prize: IPoint,
    a_button: Vector,
    b_button: Vector,
}
impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut a_button, mut b_button) = (Vector::new(0, 0), Vector::new(0, 0));
        for cap in BUTTONS.captures_iter(s) {
            let v = Vector::new(cap[2].parse()?, cap[3].parse()?);
            match &cap[1] {
                "A" => a_button = v,
                "B" => b_button = v,
                _ => return Err(anyhow!("Invalid button specification")),
            }
        }
        let Some(cap) = PRIZES.captures(s) else {
            return Err(anyhow!("Invalid prize specification"));
        };
        let prize = IPoint::new(cap[1].parse()?, cap[2].parse()?);
        Ok(Self {
            a_button,
            b_button,
            prize,
        })
    }
}
impl ClawMachine {
    fn get_pushes_to_prize(&self, offset: isize) -> Option<(isize, isize)> {
        // jfc i can't believe this stupid problem has me watching MATH VIDEOS ON YOUTUBE LIKE I'M IN HIGH SCHOOL OR SOMETHING
        // When I first read this one I saw "minimum button presses" and thought it was another LCM puzzle.
        // And I was so smug.  Like, yeah, I wrote a little utility to get the LCM because it keeps coming up.
        // Oh wait.
        let prize = self.prize + (offset, offset);
        let z = self.a_button.x * self.b_button.y - self.a_button.y * self.b_button.x;
        let a = prize.x * self.b_button.y - prize.y * self.b_button.x;
        let b = prize.y * self.a_button.x - prize.x * self.a_button.y;
        if a % z == 0 && b % z == 0 {
            Some((a / z, b / z))
        } else {
            None
        }
    }

    fn cost_to_win(&self, offset: isize) -> Option<isize> {
        let Some((a, b)) = self.get_pushes_to_prize(offset) else {
            return None;
        };
        Some(a * A_COST + b * B_COST)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn should_parse() -> Result<()> {
        let example = "Button A: X+1, Y+2\nButton B: X+3, Y+4\nPrize: X=420, Y=69";
        let machine = example.parse::<ClawMachine>()?;
        assert_eq!(IPoint::new(420, 69), machine.prize);
        assert_eq!(Vector::new(1, 2), machine.a_button);
        assert_eq!(Vector::new(3, 4), machine.b_button);
        Ok(())
    }

    #[test]
    fn should_calculate_winning_button_pushes() -> Result<()> {
        let machine =
            "Button A: X+4, Y+2\nButton B: X+3, Y+3\nPrize: X=14, Y=10".parse::<ClawMachine>()?;
        let (a, b) = machine.get_pushes_to_prize(0).ok_or(anyhow!("Oops!"))?;
        assert_eq!(2, a);
        assert_eq!(2, b);
        Ok(())
    }

    #[test]
    fn should_detect_unwinnable_prize() -> Result<()> {
        let machine =
            "Button A: X+4, Y+2\nButton B: X+3, Y+3\nPrize: X=17, Y=11".parse::<ClawMachine>()?;
        let res = machine.get_pushes_to_prize(0);
        assert!(res.is_none());
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let total: isize = EXAMPLE_INPUT
            .split("\n\n")
            .filter_map(|s| s.parse::<ClawMachine>().ok())
            .filter_map(|m| m.cost_to_win(0))
            .sum();
        assert_eq!(480, total);
        Ok(())
    }

    #[test]
    fn should_calculate_distant_cost() -> Result<()> {
        let machine = "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176"
            .parse::<ClawMachine>()?;
        assert_eq!(459236326669, machine.cost_to_win(PRIZE_OFFSET).unwrap());
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let total: isize = EXAMPLE_INPUT
            .split("\n\n")
            .filter_map(|s| s.parse::<ClawMachine>().ok())
            .filter_map(|m| m.cost_to_win(PRIZE_OFFSET))
            .sum();
        assert_eq!(875318608908, total);
        Ok(())
    }
}
