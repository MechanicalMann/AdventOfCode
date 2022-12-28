use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 11;

    const TITLE: &'static str = "Monkey in the Middle";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut monkeys = get_monkeys(&self.input().get()?);
        Ok(monkey_business(&mut monkeys, 20, true))
    }

    fn part_two(&self) -> Result<usize> {
        let mut monkeys = get_monkeys(&self.input().get()?);
        Ok(monkey_business(&mut monkeys, 10000, false))
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect_vec()
}

fn monkey_business(monkeys: &mut [Monkey], rounds: usize, relieved: bool) -> usize {
    let supermodulo = monkeys
        .iter()
        .map(|m| m.divisor)
        .reduce(|a, b| a * b)
        .unwrap();
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let throws = monkey.inspect_items(relieved, supermodulo);
            let num_inspections = throws.len();
            counts
                .entry(i)
                .and_modify(|e| *e += num_inspections)
                .or_insert(num_inspections);
            for (catcher, item) in throws {
                monkeys[catcher].items.push_back(item);
            }
        }
    }
    counts
        .values()
        .sorted()
        .rev()
        .take(2)
        .copied()
        .reduce(|acc, i| acc * i)
        .expect("Not enough monkeys")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operand {
    Old,
    New(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add(Operand),
    Mult(Operand),
}

struct Monkey {
    items: VecDeque<usize>,
    operation: Operator,
    divisor: usize,
    receivers: (usize, usize),
}
impl Monkey {
    fn inspect_items(&mut self, relieved: bool, modulo: usize) -> Vec<(usize, usize)> {
        let mut throws = vec![];
        while let Some(item) = self.items.pop_front() {
            let mut new_level = match self.operation {
                Operator::Add(operand) => match operand {
                    Operand::Old => item + item,
                    Operand::New(val) => item + val,
                },
                Operator::Mult(operand) => match operand {
                    Operand::Old => item * item,
                    Operand::New(val) => item * val,
                },
            };
            if relieved {
                new_level = new_level / 3;
            }
            let dest = match new_level % self.divisor as usize {
                0 => self.receivers.0,
                _ => self.receivers.1,
            };
            throws.push((dest, new_level % modulo));
        }
        throws
    }
}
impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect_vec();
        if lines.len() != 6 {
            return Err(anyhow!("Invalid Monkey"));
        }
        let items = lines[1][18..]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let operand = match &lines[2][25..] {
            "old" => Operand::Old,
            num => Operand::New(num.parse().unwrap()),
        };
        let operation = match lines[2].chars().nth(23) {
            Some(c) => match c {
                '*' => Operator::Mult(operand),
                '+' => Operator::Add(operand),
                _ => return Err(anyhow!("Invalid operator")),
            },
            _ => return Err(anyhow!("No operator")),
        };
        let divisor = lines[3][21..].parse::<usize>()?;
        let rec_true = lines[4][29..].parse::<usize>()?;
        let rec_false = lines[5][30..].parse::<usize>()?;
        Ok(Monkey {
            items,
            operation,
            divisor,
            receivers: (rec_true, rec_false),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "Monkey 0:
  Starting items: 1, 2, 128
  Operation: new = old * 2
  Test: divisible by 8
    If true: throw to monkey 1
    If false: throw to monkey 2";
        let expected = Monkey {
            items: VecDeque::from_iter([1, 2, 128]),
            operation: Operator::Mult(Operand::New(2)),
            divisor: 8,
            receivers: (1, 2),
        };
        let actual = input.parse::<Monkey>().unwrap();
        assert!(itertools::equal(expected.items, actual.items));
        assert_eq!(expected.operation, actual.operation);
        assert_eq!(expected.divisor, actual.divisor);
        assert_eq!(expected.receivers, actual.receivers);
    }

    #[test]
    fn should_throw() {
        let input = "Monkey 0:
  Starting items: 10, 24
  Operation: new = old * 2
  Test: divisible by 8
    If true: throw to monkey 1
    If false: throw to monkey 2";
        let mut monkey = input.parse::<Monkey>().unwrap();
        let expected = vec![(2, 6), (1, 16)];
        let actual = monkey.inspect_items(true, usize::MAX);
        assert!(itertools::equal(expected, actual));
    }

    #[test]
    fn should_solve_part_1() {
        let mut monkeys = get_monkeys(&EXAMPLE_INPUT);
        let actual = monkey_business(&mut monkeys, 20, true);
        assert_eq!(10605, actual);
    }

    #[test]
    fn should_solve_part_2() {
        let mut monkeys = get_monkeys(&EXAMPLE_INPUT);
        let actual = monkey_business(&mut monkeys, 10000, false);
        assert_eq!(2713310158, actual);
    }

    const EXAMPLE_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
