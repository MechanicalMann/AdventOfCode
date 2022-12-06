use std::str::FromStr;

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

pub struct Solution;
impl Solver<String, String> for Solution {
    const DAY: u8 = 5;
    const TITLE: &'static str = "Supply Stacks";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<String> {
        let mut input = self.input().get_as::<Input>()?;
        operate_crane(&mut input.crates, &input.instructions);
        Ok(get_top_crates(&input.crates))
    }

    fn part_two(&self) -> Result<String> {
        let mut input = self.input().get_as::<Input>()?;
        operate_crane_over_9000(&mut input.crates, &input.instructions);
        Ok(get_top_crates(&input.crates))
    }
}

fn operate_crane(crates: &mut Crates, instructions: &[Instruction]) {
    for i in instructions {
        let (s, d) = (i.source - 1, i.dest - 1); // Instructions are 1-indexed
        for _ in 0..i.number {
            let c = crates.stacks[s].pop().unwrap();
            crates.stacks[d].push(c);
        }
    }
}

fn operate_crane_over_9000(crates: &mut Crates, instructions: &[Instruction]) {
    for i in instructions {
        let (s, d) = (i.source - 1, i.dest - 1); // Instructions are 1-indexed
        let l = crates.stacks[s].len();
        let offset = l - i.number;
        for _ in offset..l {
            let c = crates.stacks[s].remove(offset);
            crates.stacks[d].push(c);
        }
    }
}

fn get_top_crates(crates: &Crates) -> String {
    String::from_iter(crates.stacks.iter().filter_map(|c| c.last()))
}

#[derive(Debug, PartialEq)]
struct Crates {
    stacks: Vec<Vec<char>>,
}
impl FromStr for Crates {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CRATE_RE: Regex = Regex::new(r"(?:(?:\[[A-Z]\]| ( ) ) ?)").unwrap();
        }
        let mut stacks: Vec<Vec<char>> = vec![];
        for l in s.lines().rev() {
            if l.starts_with("  1") {
                continue;
            }
            for (i, cap) in CRATE_RE.captures_iter(l).enumerate() {
                if stacks.len() == i {
                    stacks.push(vec![]);
                }
                let c = cap[0].chars().nth(1).unwrap();
                if c == ' ' {
                    continue;
                }
                stacks[i].push(c);
            }
        }
        Ok(Crates { stacks })
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    number: usize,
    source: usize,
    dest: usize,
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref INSTRUCT_RE: Regex =
                Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        let mat = INSTRUCT_RE.captures(s);
        match mat {
            Some(cap) => {
                let number = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let source = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let dest = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
                Ok(Instruction {
                    number,
                    source,
                    dest,
                })
            }
            None => Err(anyhow!("Invalid instruction")),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    crates: Crates,
    instructions: Vec<Instruction>,
}
impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spl = s.split("\n\n").collect_vec();
        if spl.len() != 2 {
            return Err(anyhow!("Invalid input"));
        }
        let crates = spl[0].parse::<Crates>()?;
        let instructions = spl[1]
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect_vec();
        Ok(Input {
            crates,
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
    1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn should_parse_crates() {
        let crates = "[A] [B]    [C]
[D]     [E] [F]
 1   2   3   4";
        let expected = vec![vec!['D', 'A'], vec!['B'], vec!['E'], vec!['F', 'C']];
        let actual = crates.parse::<Crates>().unwrap();
        assert!(itertools::equal(expected, actual.stacks));
    }

    #[test]
    fn should_parse_instruction() {
        let instr = "move 1 from 2 to 3";
        let expected = Instruction {
            number: 1,
            source: 2,
            dest: 3,
        };
        let actual = instr.parse::<Instruction>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_parse_input() {
        let expected = Input {
            crates: Crates {
                stacks: vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            },
            instructions: vec![
                Instruction {
                    number: 1,
                    source: 2,
                    dest: 1,
                },
                Instruction {
                    number: 3,
                    source: 1,
                    dest: 3,
                },
                Instruction {
                    number: 2,
                    source: 2,
                    dest: 1,
                },
                Instruction {
                    number: 1,
                    source: 1,
                    dest: 2,
                },
            ],
        };
        let actual = EXAMPLE_INPUT.parse::<Input>().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_move_crates() {
        let expected = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        let mut input = EXAMPLE_INPUT.parse::<Input>().unwrap();
        operate_crane(&mut input.crates, &input.instructions);
        assert!(itertools::equal(expected, input.crates.stacks));
    }

    #[test]
    fn should_solve_part_1() {
        let expected = "CMZ";
        let mut input = EXAMPLE_INPUT.parse::<Input>().unwrap();
        operate_crane(&mut input.crates, &input.instructions);
        let actual = get_top_crates(&input.crates);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_move_crates_better() {
        let expected = vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']];
        let mut input = EXAMPLE_INPUT.parse::<Input>().unwrap();
        operate_crane_over_9000(&mut input.crates, &input.instructions);
        assert!(itertools::equal(expected, input.crates.stacks));
    }

    #[test]
    fn should_solve_part_2() {
        let expected = "MCD";
        let mut input = EXAMPLE_INPUT.parse::<Input>().unwrap();
        operate_crane_over_9000(&mut input.crates, &input.instructions);
        let actual = get_top_crates(&input.crates);
        assert_eq!(expected, actual);
    }
}
