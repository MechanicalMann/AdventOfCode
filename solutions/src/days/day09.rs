use std::{collections::HashSet, str::FromStr};

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 9;

    const TITLE: &'static str = "Rope Bridge";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let instructions = self.input().get_lines_as::<Move>()?;
        let rope = simulate(&instructions);
        Ok(rope.tail_history.len())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

fn simulate(instructions: &[Move]) -> Rope {
    let mut rope = Rope::new();
    for i in instructions {
        rope.move_head(i);
    }
    rope
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}
impl Move {
    fn get_value(&self) -> isize {
        match self {
            Move::Up(v) => *v,
            Move::Down(v) => *v,
            Move::Left(v) => *v,
            Move::Right(v) => *v,
        }
    }
}
impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let halves = s.split(' ').collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!(format!("Invalid movement instruction: {}", s)));
        }
        let steps = halves[1].parse::<isize>()?;
        match halves[0] {
            "U" => Ok(Move::Up(steps)),
            "D" => Ok(Move::Down(steps)),
            "L" => Ok(Move::Left(steps)),
            "R" => Ok(Move::Right(steps)),
            _ => Err(anyhow!(format!("Invalid movement instruction: {}", s))),
        }
    }
}

struct Rope {
    head_pos: (isize, isize),
    tail_pos: (isize, isize),
    tail_history: HashSet<(isize, isize)>,
}
impl Rope {
    fn new() -> Self {
        Rope {
            head_pos: (0, 0),
            tail_pos: (0, 0),
            tail_history: HashSet::from_iter([(0, 0)]),
        }
    }

    fn move_head(&mut self, instr: &Move) {
        let (mut head_x, mut head_y) = self.head_pos;
        let (mut tail_x, mut tail_y) = self.tail_pos;
        let steps = instr.get_value();
        for _ in 0..steps {
            match instr {
                Move::Up(_) => head_y += 1,
                Move::Down(_) => head_y -= 1,
                Move::Left(_) => head_x -= 1,
                Move::Right(_) => head_x += 1,
            }
            let (dx, dy) = ((head_x - tail_x), (head_y - tail_y));
            match dx {
                2 => tail_x += 1,
                -2 => tail_x -= 1,
                1 if dy.abs() == 2 => tail_x += 1,
                -1 if dy.abs() == 2 => tail_x -= 1,
                _ => (),
            }
            match dy {
                2 => tail_y += 1,
                -2 => tail_y -= 1,
                1 if dx.abs() == 2 => tail_y += 1,
                -1 if dx.abs() == 2 => tail_y -= 1,
                _ => (),
            }
            self.tail_history.insert((tail_x, tail_y));
        }
        self.head_pos = (head_x, head_y);
        self.tail_pos = (tail_x, tail_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn should_parse() {
        let input = "U 1
D 2
L 3
R 4";
        let expected = [Move::Up(1), Move::Down(2), Move::Left(3), Move::Right(4)];
        let actual = input
            .lines()
            .map(|l| l.parse::<Move>().unwrap())
            .collect_vec();
        assert!(itertools::equal(expected, actual));
    }

    #[test]
    fn should_move_head() {
        let mut rope = Rope::new();
        rope.move_head(&Move::Up(1));
        rope.move_head(&Move::Left(1));
        assert_eq!((-1, 1), rope.head_pos);
    }

    #[test]
    fn should_move_tail() {
        let mut rope = Rope::new();
        rope.move_head(&Move::Up(2));
        rope.move_head(&Move::Left(2));
        assert_eq!((-2, 2), rope.head_pos);
        assert_eq!((-1, 1), rope.tail_pos);
    }

    #[test]
    fn should_keep_tail_history() {
        let mut rope = Rope::new();
        rope.move_head(&Move::Up(3));
        rope.move_head(&Move::Left(3));

        let expected: HashSet<(isize, isize)> =
            HashSet::from_iter([(0, 0), (0, 1), (0, 2), (-1, 3), (-2, 3)]);
        assert_eq!(expected, rope.tail_history);
    }

    #[test]
    fn should_solve_part_1() {
        let instructions = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Move>().unwrap())
            .collect_vec();
        let rope = simulate(&instructions);
        assert_eq!(13, rope.tail_history.len());
    }
}
