use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 19;
    const TITLE: &'static str = "UNKNOWN";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let system = self.input().get_as::<System>()?;
        Ok(system.get_total_rating())
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
        for rating in input[1..input.len() - 1].split(',') {
            if let Some((stat, value)) = rating.split('=').collect_tuple() {
                let v = value.parse()?;
                match stat {
                    "x" => x = v,
                    "m" => m = v,
                    "a" => a = v,
                    "s" => s = v,
                    _ => return Err(anyhow!("Invalid part: {input}")),
                }
            }
        }
        Ok(Part { x, m, a, s })
    }
}
impl Part {
    fn get_total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rating {
    X,
    M,
    A,
    S,
}
impl FromStr for Rating {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "x" => Ok(Rating::X),
            "m" => Ok(Rating::M),
            "a" => Ok(Rating::A),
            "s" => Ok(Rating::S),
            _ => Err(anyhow!("Invalid rating: {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}
impl FromStr for Destination {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(Destination::Accepted),
            "R" => Ok(Destination::Rejected),
            _ => Ok(Destination::Workflow(s.to_owned())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    LT,
    GT,
}
impl FromStr for Operator {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            ">" => Ok(Operator::GT),
            "<" => Ok(Operator::LT),
            _ => Err(anyhow!("Unsupported operator: {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Compare(Rating, Operator, usize, Destination),
    Send(Destination),
}
impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.find(':') {
            Some(i) => {
                if i < 3 {
                    return Err(anyhow!("Invalid operation: {s}"));
                }
                let rating = s[0..1].parse::<Rating>()?;
                let operator = s[1..2].parse::<Operator>()?;
                let value = s[2..i].parse::<usize>()?;
                let destination = s[i + 1..].parse::<Destination>()?;
                Ok(Operation::Compare(rating, operator, value, destination))
            }
            None => Ok(Operation::Send(s.parse()?)),
        }
    }
}

struct Workflow {
    key: String,
    operations: Vec<Operation>,
}
impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let bracket = match s.find('{') {
            Some(i) => i,
            None => return Err(anyhow!("Invalid workflow: {s}")),
        };
        let key = s[0..bracket].to_owned();
        let mut operations = vec![];
        for op in s[bracket + 1..s.len() - 1].split(',') {
            operations.push(op.parse()?);
        }
        Ok(Workflow { key, operations })
    }
}
impl Workflow {
    fn run(&self, part: &Part) -> Destination {
        for op in &self.operations {
            match op {
                Operation::Compare(rating, operator, value, dest) => {
                    let lhs = match rating {
                        Rating::X => &part.x,
                        Rating::M => &part.m,
                        Rating::A => &part.a,
                        Rating::S => &part.s,
                    };
                    let result = match operator {
                        Operator::LT => lhs < value,
                        Operator::GT => lhs > value,
                    };
                    if result {
                        return dest.clone();
                    }
                }
                Operation::Send(dest) => return dest.clone(),
            }
        }
        panic!("Something has gone horribly wrong")
    }
}

const START_RULE: &str = "in";
struct System {
    workflows: HashMap<String, Workflow>,
    start: String,
    parts: Vec<Part>,
}
impl FromStr for System {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let halves = s.split("\n\n").collect_vec();
        if halves.len() != 2 {
            return Err(anyhow!("Invalid system specification"));
        }
        let mut workflows = HashMap::new();
        for line in halves[0].lines() {
            let w = line.parse::<Workflow>()?;
            workflows.insert(w.key.clone(), w);
        }
        let parts = halves[1]
            .lines()
            .filter_map(|l| l.parse().ok())
            .collect_vec();
        Ok(System {
            workflows,
            parts,
            start: START_RULE.to_string(),
        })
    }
}
impl System {
    fn run(&self) -> Vec<&Part> {
        let mut accepted = vec![];
        for part in &self.parts {
            let mut key = self.start.clone();
            while let Some(workflow) = self.workflows.get(&key) {
                match workflow.run(part) {
                    Destination::Accepted => {
                        accepted.push(part);
                        break;
                    }
                    Destination::Rejected => break,
                    Destination::Workflow(next_key) => key = next_key,
                }
            }
        }
        accepted
    }

    fn get_total_rating(&self) -> usize {
        let accepted = self.run();
        accepted.iter().map(|a| a.get_total_rating()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn should_parse_parts() -> Result<()> {
        let test = "{x=1,m=2,a=3,s=4}".parse::<Part>()?;
        assert_eq!(1, test.x);
        assert_eq!(2, test.m);
        assert_eq!(3, test.a);
        assert_eq!(4, test.s);
        Ok(())
    }

    #[test]
    fn should_parse_workflows() -> Result<()> {
        let test = "a{x<420:b,s>69:R,A}".parse::<Workflow>()?;
        let expected = vec![
            Operation::Compare(
                Rating::X,
                Operator::LT,
                420,
                Destination::Workflow("b".to_string()),
            ),
            Operation::Compare(Rating::S, Operator::GT, 69, Destination::Rejected),
            Operation::Send(Destination::Accepted),
        ];
        assert_eq!("a", &test.key);
        assert_eq!(expected, test.operations);
        Ok(())
    }

    #[test]
    fn should_run_workflow() -> Result<()> {
        let test = "a{x<420:b,s>69:R,A}".parse::<Workflow>()?;
        let part = "{x=9001,m=420,a=42,s=76}".parse::<Part>()?;
        let dest = test.run(&part);
        assert_eq!(Destination::Rejected, dest);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let system = EXAMPLE_INPUT.parse::<System>()?;
        let rating = system.get_total_rating();
        assert_eq!(19114, rating);
        Ok(())
    }
}
