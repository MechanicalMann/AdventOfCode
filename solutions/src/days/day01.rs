use std::str::FromStr;

use crate::solver::Solver;
use anyhow::*;

pub struct Solution;
impl Solver<isize, isize> for Solution {
    const DAY: u8 = 1;
    const TITLE: &'static str = "Secret Entrance";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let instructions = self.input().get_lines_as::<Rotation>()?;
        let mut safe = Safe::new();
        safe.rotate(&instructions);
        Ok(safe.zero_count)
    }

    fn part_two(&self) -> Result<isize> {
        let instructions = self.input().get_lines_as::<Rotation>()?;
        let mut safe = Safe::new();
        safe.rotate_0x434C49434B(&instructions);
        Ok(safe.zero_count)
    }
}

#[derive(Debug)]
struct Safe {
    dial: isize,
    zero_count: isize,
}
impl Safe {
    fn new() -> Self {
        Safe {
            dial: 50,
            zero_count: 0,
        }
    }
    fn rotate(&mut self, instructions: &[Rotation]) {
        for i in instructions {
            match i {
                Rotation::Left(n) => self.dial = (((self.dial - n) % 100) + 100) % 100,
                Rotation::Right(n) => self.dial = (self.dial + n) % 100,
            }
            if self.dial == 0 {
                self.zero_count += 1;
            }
        }
    }
    #[allow(non_snake_case)] // ???
    fn rotate_0x434C49434B(&mut self, instructions: &[Rotation]) {
        for i in instructions {
            let prev = self.dial;
            match i {
                Rotation::Left(n) => {
                    self.dial = self.dial - n;
                }
                Rotation::Right(n) => {
                    self.dial = self.dial + n;
                }
            }
            self.zero_count += self.dial.abs() / 100;
            if prev != 0 && self.dial <= 0 {
                self.zero_count += 1;
            }
            self.dial = ((self.dial % 100) + 100) % 100;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Left(isize),
    Right(isize),
}
impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let num = s[1..].parse::<isize>()?;
        match &s.chars().nth(0).ok_or(anyhow!("Invalid input"))? {
            'L' => Ok(Rotation::Left(num)),
            'R' => Ok(Rotation::Right(num)),
            _ => Err(anyhow!("Invalid rotation specification")),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn should_parse() -> Result<()> {
        let l = "L12".parse::<Rotation>()?;
        let r = "R34".parse::<Rotation>()?;
        assert_eq!(Rotation::Left(12), l);
        assert_eq!(Rotation::Right(34), r);
        Ok(())
    }

    #[test]
    fn should_rotate() {
        let instr = vec![Rotation::Left(12), Rotation::Right(34)];
        let mut safe = Safe::new();
        safe.rotate(&instr);
        assert_eq!(72, safe.dial);
        assert_eq!(0, safe.zero_count);
    }

    #[test]
    fn should_wrap() {
        let instr = vec![Rotation::Left(75)];
        let mut safe = Safe::new();
        safe.rotate(&instr);
        assert_eq!(75, safe.dial);

        let next = vec![Rotation::Right(26)];
        safe.rotate(&next);
        assert_eq!(1, safe.dial);
    }

    #[test]
    fn should_count_zeroes() {
        let instr = vec![
            Rotation::Right(50),
            Rotation::Left(13),
            Rotation::Right(13),
            Rotation::Left(37),
        ];
        let mut safe = Safe::new();
        safe.rotate(&instr);
        assert_eq!(2, safe.zero_count);
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let instructions = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Rotation>().ok())
            .collect_vec();
        let mut safe = Safe::new();
        safe.rotate(&instructions);
        assert_eq!(3, safe.zero_count);
        Ok(())
    }

    #[test]
    fn should_count_zeroes_method_2() {
        let instr = vec![Rotation::Right(1000), Rotation::Left(1000)];
        let mut safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(20, safe.zero_count);
    }

    #[test]
    fn should_catch_edge_cases() {
        // Should return 1
        let mut instr = vec![Rotation::Left(50), Rotation::Right(50)];
        let mut safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(1, safe.zero_count);

        instr = vec![Rotation::Left(50), Rotation::Left(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(1, safe.zero_count);

        instr = vec![Rotation::Right(50), Rotation::Left(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(1, safe.zero_count);

        instr = vec![Rotation::Right(50), Rotation::Right(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(1, safe.zero_count);

        // Should return 2
        instr = vec![Rotation::Left(150), Rotation::Left(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(2, safe.zero_count);

        instr = vec![Rotation::Left(150), Rotation::Right(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(2, safe.zero_count);

        instr = vec![Rotation::Right(150), Rotation::Left(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(2, safe.zero_count);

        instr = vec![Rotation::Right(150), Rotation::Right(50)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(2, safe.zero_count);

        instr = vec![Rotation::Left(50), Rotation::Right(101)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(2, safe.zero_count);

        // Should return 7
        instr = vec![Rotation::Left(650)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(7, safe.zero_count);

        instr = vec![Rotation::Right(650)];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(7, safe.zero_count);

        // Should return 8
        instr = vec![
            Rotation::Right(50),
            Rotation::Left(500),
            Rotation::Right(200),
        ];
        safe = Safe::new();
        safe.rotate_0x434C49434B(&instr);
        assert_eq!(8, safe.zero_count);
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let instructions = EXAMPLE_INPUT
            .lines()
            .filter_map(|l| l.parse::<Rotation>().ok())
            .collect_vec();
        let mut safe = Safe::new();
        safe.rotate_0x434C49434B(&instructions);
        assert_eq!(6, safe.zero_count);
        Ok(())
    }
}
