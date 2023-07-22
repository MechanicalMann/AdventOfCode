use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;
use std::{collections::VecDeque, str::FromStr};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 13;
    const TITLE: &'static str = "Distress Signal";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let pairs = self.input().get_grouped_as::<Pair>()?;
        Ok(sum_valid_indices(&pairs))
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Integer(u8),
    List(Vec<Value>),
}
impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut stack: VecDeque<Value> = VecDeque::new();

        for c in s.chars() {
            match c {
                '[' => stack.push_back(Value::List(vec![])),
                ',' | ']' => match stack.pop_back() {
                    Some(el) => match stack.back_mut() {
                        Some(Value::List(l)) => l.push(el),
                        _ => stack.push_back(el),
                    },
                    None => return Err(anyhow!("Empty stack!")),
                },
                '0'..='9' => {
                    let d = c.to_digit(10).unwrap() as u8;
                    match stack.pop_back() {
                        Some(Value::Integer(i)) => stack.push_back(Value::Integer(i * 10 + d)),
                        Some(v) => {
                            stack.push_back(v);
                            stack.push_back(Value::Integer(d))
                        }
                        _ => stack.push_back(Value::Integer(d)),
                    }
                }
                _ => (),
            }
            // println!("Stack: {:?}", stack);
        }

        Ok(stack.front().unwrap().to_owned())
    }
}

#[derive(Debug, PartialEq)]
struct Pair {
    left: Value,
    right: Value,
}
impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let halves = s.lines().collect_vec();
        match halves.len() {
            2 => Ok(Pair {
                left: halves[0].parse::<Value>()?,
                right: halves[1].parse::<Value>()?,
            }),
            _ => Err(anyhow!("Invalid pair!")),
        }
    }
}
impl Pair {
    fn is_ordered(&self) -> bool {
        match compare(&self.left, &self.right) {
            Some(v) => v,
            None => false,
        }
    }
}

fn compare(left: &Value, right: &Value) -> Option<bool> {
    // println!("Comparing: {:?} to {:?}", left, right);
    match (left, right) {
        (Value::Integer(l), Value::Integer(r)) => {
            if l < r {
                Some(true)
            } else if l > r {
                Some(false)
            } else {
                None
            }
        }
        (Value::List(l), Value::List(r)) => {
            let (mut il, mut ir) = (l.iter(), r.iter());
            loop {
                match (il.next(), ir.next()) {
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (Some(vl), Some(vr)) => match compare(vl, vr) {
                        Some(v) => return Some(v),
                        None => (),
                    },
                    _ => {
                        break;
                    }
                }
            }
            None
        }
        (Value::Integer(l), Value::List(_)) => {
            compare(&Value::List(vec![Value::Integer(*l)]), right)
        }
        (Value::List(_), Value::Integer(r)) => {
            compare(left, &Value::List(vec![Value::Integer(*r)]))
        }
    }
}

fn get_valid_indices(pairs: &[Pair]) -> Vec<usize> {
    let mut indices = vec![];
    for (i, pair) in pairs.iter().enumerate() {
        if pair.is_ordered() {
            indices.push(i + 1); // One-indexed?!?!?!
        }
    }
    indices
}

fn sum_valid_indices(pairs: &[Pair]) -> usize {
    get_valid_indices(pairs).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() -> Result<()> {
        let test1 = "[1,2,3]".parse::<Value>()?;
        let test2 = "[1,[2,[3]]]".parse::<Value>()?;
        let test3 = "[[]]".parse::<Value>()?;
        let test4 = "[1,[2],3]".parse::<Value>()?;

        assert_eq!(
            Value::List(vec![
                Value::Integer(1),
                Value::Integer(2),
                Value::Integer(3)
            ]),
            test1
        );
        assert_eq!(
            Value::List(vec![
                Value::Integer(1),
                Value::List(vec![
                    Value::Integer(2),
                    Value::List(vec![Value::Integer(3),])
                ])
            ]),
            test2
        );
        assert_eq!(Value::List(vec![Value::List(vec![])]), test3);
        assert_eq!(
            Value::List(vec![
                Value::Integer(1),
                Value::List(vec![Value::Integer(2)]),
                Value::Integer(3),
            ]),
            test4
        );
        Ok(())
    }

    #[test]
    fn should_parse_pair() -> Result<()> {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]";
        let parsed = input.parse::<Pair>()?;
        assert_eq!(
            Pair {
                left: Value::List(vec![
                    Value::Integer(1),
                    Value::Integer(1),
                    Value::Integer(3),
                    Value::Integer(1),
                    Value::Integer(1)
                ]),
                right: Value::List(vec![
                    Value::Integer(1),
                    Value::Integer(1),
                    Value::Integer(5),
                    Value::Integer(1),
                    Value::Integer(1)
                ])
            },
            parsed
        );
        Ok(())
    }

    #[test]
    fn should_compare() -> Result<()> {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]";
        let parsed = input.parse::<Pair>()?;
        assert!(parsed.is_ordered());
        Ok(())
    }

    #[test]
    fn should_solve_part_1() -> Result<()> {
        let pairs = EXAMPLE_INPUT
            .split("\n\n")
            .map(|s| s.parse::<Pair>().unwrap())
            .collect_vec();
        assert_eq!(sum_valid_indices(&pairs), 13);
        Ok(())
    }

    const EXAMPLE_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
