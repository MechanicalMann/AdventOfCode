use std::{collections::HashMap, str::FromStr};

use itertools::{Itertools, MinMaxResult::MinMax};

use crate::solver::Solver;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 14;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<usize> {
        let mut polymer = self.input().get_as::<Polymer>()?;
        polymer.step_for(10);
        Ok(polymer.get_range())
    }

    fn part_two(&self) -> anyhow::Result<usize> {
        Ok(0)
    }
}

struct Polymer {
    elements: Vec<char>,
    insertions: HashMap<(char, char), char>,
}
impl FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split("\n\n");
        let elements = spl.next().unwrap().chars().collect_vec();
        let mut insertions = HashMap::new();
        for line in spl.next().unwrap().lines() {
            let mut ins = line.split(" -> ");
            let pair = ins.next().unwrap().chars().collect_tuple().unwrap();
            let c = ins.next().unwrap().chars().next().unwrap();
            insertions.insert(pair, c);
        }
        Ok(Polymer {
            elements,
            insertions,
        })
    }
}
impl Polymer {
    fn step(&mut self) {
        let mut to_insert: Vec<(usize, char)> = vec![];
        for i in 0..(self.elements.len() - 1) {
            let a = self.elements[i];
            let b = self.elements[i + 1];
            if let Some(&c) = self.insertions.get(&(a, b)) {
                to_insert.push((i + 1, c));
            }
        }
        for (i, &(pos, c)) in to_insert.iter().enumerate() {
            self.elements.insert(pos + i, c);
        }
    }

    fn step_for(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step();
        }
    }

    fn count(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for e in &self.elements {
            *counts.entry(*e).or_insert(0) += 1;
        }
        counts
    }

    fn get_range(&self) -> usize {
        let counts = self.count();
        if let MinMax(min, max) = counts.iter().minmax_by(|&a, &b| a.1.cmp(&b.1)) {
            return max.1 - min.1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "AB

AB -> C";
        let polymer = input.parse::<Polymer>().unwrap();
        assert_eq!(2, polymer.elements.len());
        assert_eq!(vec!['A', 'B'], polymer.elements);
        assert_eq!(1, polymer.insertions.len());
        assert_eq!(
            vec![(('A', 'B'), 'C')],
            polymer.insertions.into_iter().collect_vec()
        );
    }

    #[test]
    fn should_step() {
        let input = "AB

AB -> C";
        let mut polymer = input.parse::<Polymer>().unwrap();
        polymer.step();
        assert_eq!(3, polymer.elements.len());
        assert_eq!(vec!['A', 'C', 'B'], polymer.elements);
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let mut polymer = input.parse::<Polymer>().unwrap();
        polymer.step_for(10);
        assert_eq!(3073, polymer.elements.len());

        let counts = polymer.count();
        assert_eq!(1749, counts[&'B']);
        assert_eq!(298, counts[&'C']);
        assert_eq!(161, counts[&'H']);
        assert_eq!(865, counts[&'N']);

        let range = polymer.get_range();
        assert_eq!(1588, range);
    }
}
