use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::{anyhow, Result};

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 15;
    const TITLE: &'static str = "Lens Library";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let sequence = self.input().get_csv()?;
        Ok(hash_sequence(&sequence))
    }

    fn part_two(&self) -> Result<usize> {
        let sequence = self.input().get_csv_as::<Instruction>()?;
        let mut library = Library::new();
        library.run(&sequence);
        Ok(library.get_focus_power())
    }
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h += c as usize;
        h = (h * 17) % 256;
    }
    h
}

fn hash_sequence(seq: &[String]) -> usize {
    seq.iter().map(|s| hash(s)).sum()
}

#[derive(Debug)]
struct LensBox {
    lenses: Vec<u8>,
    index: HashMap<String, usize>,
}
impl LensBox {
    fn new() -> Self {
        LensBox {
            lenses: Vec::new(),
            index: HashMap::new(),
        }
    }

    fn get_focus_power(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, &lens)| (i + 1) * lens as usize)
            .sum()
    }
}

#[derive(Debug)]
struct Library {
    boxes: HashMap<usize, LensBox>,
}
impl Library {
    fn new() -> Self {
        Library {
            boxes: HashMap::new(),
        }
    }

    fn process(&mut self, instr: &Instruction) {
        match instr {
            Instruction::Upsert((label, lens)) => {
                let b = self.get_box(&label);
                match b.index.get(label) {
                    Some(&idx) => {
                        b.lenses[idx] = *lens;
                    }
                    None => {
                        let idx = b.lenses.len();
                        b.lenses.push(*lens);
                        b.index.insert(label.clone(), idx);
                    }
                }
            }
            Instruction::Remove(label) => {
                let b = self.get_box(&label);
                match b.index.get(label) {
                    Some(&idx) => {
                        b.lenses.remove(idx);
                        let mut reindex = HashMap::new();
                        for (k, &v) in b.index.iter() {
                            if k == label {
                                continue;
                            }
                            if v > idx {
                                reindex.insert(k.clone(), v - 1);
                            } else {
                                reindex.insert(k.clone(), v);
                            }
                        }
                        b.index = reindex;
                    }
                    None => (),
                }
            }
        }
    }

    fn get_box<'a>(&'a mut self, label: &str) -> &'a mut LensBox {
        self.boxes.entry(hash(label)).or_insert(LensBox::new())
    }

    fn run(&mut self, program: &[Instruction]) {
        for ins in program {
            self.process(ins);
        }
    }

    fn get_focus_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(n, b)| (n + 1) * b.get_focus_power())
            .sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Upsert((String, u8)),
    Remove(String),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        for (i, c) in s.chars().enumerate() {
            match c {
                '-' => {
                    return Ok(Instruction::Remove(s[0..i].to_owned()));
                }
                '=' => {
                    return Ok(Instruction::Upsert((
                        s[0..i].to_owned(),
                        s[i + 1..].parse()?,
                    )))
                }
                _ => (),
            }
        }
        Err(anyhow!("Invalid instruction: {s}"))
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    const EXAMPLE_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn should_hash() -> Result<()> {
        let test = "HASH";
        assert_eq!(52, hash(test));
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let inputs = EXAMPLE_INPUT.split(',').map(|s| s.to_owned()).collect_vec();
        assert_eq!(1320, hash_sequence(&inputs));
        Ok(())
    }

    #[test]
    fn should_parse() -> Result<()> {
        let rem = "ab-".parse::<Instruction>()?;
        assert_eq!(Instruction::Remove(String::from("ab")), rem);
        let ups = "cd=123".parse::<Instruction>()?;
        assert_eq!(Instruction::Upsert((String::from("cd"), 123)), ups);
        Ok(())
    }

    #[test]
    fn should_process_add() -> Result<()> {
        let mut library = Library::new();
        let one = "ab=1".parse::<Instruction>()?;
        library.process(&one);

        let exp_hash = hash("ab");
        let key = "ab".to_string();

        assert!(library.boxes.contains_key(&exp_hash));
        let b = library.boxes.get_mut(&exp_hash).unwrap();

        assert!(b.index.contains_key(&key));
        assert_eq!(Some(&0), b.index.get(&key));
        assert_eq!(vec![1], b.lenses);

        Ok(())
    }

    #[test]
    fn should_process_remove() -> Result<()> {
        let mut library = Library::new();
        let one = "ab=1".parse::<Instruction>()?;
        let two = "ab-".parse::<Instruction>()?;
        library.process(&one);
        library.process(&two);

        let exp_hash = hash("ab");
        let key = "ab".to_string();

        assert!(library.boxes.contains_key(&exp_hash));
        let b = library.boxes.get_mut(&exp_hash).unwrap();

        assert!(!b.index.contains_key(&key));
        assert_eq!(None, b.index.get(&key));
        assert_eq!(Vec::<u8>::new(), b.lenses);

        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let mut library = Library::new();
        let instructions = EXAMPLE_INPUT
            .split(',')
            .filter_map(|s| s.parse::<Instruction>().ok())
            .collect_vec();
        library.run(&instructions);
        println!("{library:?}");
        assert_eq!(145, library.get_focus_power());
        Ok(())
    }
}
