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
        let mut polymer = self.input().get_as::<Polymer>()?;
        polymer.step_for(40);
        Ok(polymer.get_range())
    }
}

struct Polymer {
    pairs: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
    insertions: HashMap<(char, char), char>,
}
impl FromStr for Polymer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spl = s.split("\n\n");

        let elements = spl.next().unwrap().chars().collect_vec();
        let mut pairs: HashMap<(char, char), usize> = HashMap::new();
        let mut counts: HashMap<char, usize> = HashMap::new();
        for i in 0..(elements.len() - 1) {
            let pair = (elements[i], elements[i + 1]);
            *pairs.entry(pair).or_insert(0) += 1;
            *counts.entry(pair.0).or_insert(0) += 1;
        }
        *counts.entry(elements[elements.len() - 1]).or_insert(0) += 1;

        let mut insertions = HashMap::new();
        for line in spl.next().unwrap().lines() {
            let mut ins = line.split(" -> ");
            let pair = ins.next().unwrap().chars().collect_tuple().unwrap();
            let c = ins.next().unwrap().chars().next().unwrap();
            insertions.insert(pair, c);
        }
        Ok(Polymer {
            pairs,
            counts,
            insertions,
        })
    }
}
impl Polymer {
    fn step(&mut self) {
        let mut newpairs = self.pairs.clone();
        for (&pair, &c) in &self.insertions {
            if let Some(&count) = self.pairs.get(&pair) {
                if count == 0 {
                    continue;
                }
                for np in [(pair.0, c), (c, pair.1)] {
                    *newpairs.entry(np).or_insert(0) += count;
                }
                *self.counts.entry(c).or_insert(0) += count;
                *newpairs.get_mut(&pair).unwrap() -= count;
            }
        }
        self.pairs = newpairs;
    }

    fn step_for(&mut self, iterations: usize) {
        for _i in 0..iterations {
            self.step();
        }
    }

    fn get_range(&self) -> usize {
        if let MinMax(min, max) = self.counts.iter().minmax_by(|&a, &b| a.1.cmp(&b.1)) {
            return max.1 - min.1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Polymer {
        fn len(&self) -> usize {
            self.counts.values().sum()
        }
    }

    #[test]
    fn should_parse() {
        let input = "AB

AB -> C";
        let polymer = input.parse::<Polymer>().unwrap();
        assert_eq!(1, polymer.pairs.len());
        assert_eq!(vec![&('A', 'B')], polymer.pairs.keys().collect_vec());
        assert_eq!(1, polymer.pairs[&('A', 'B')]);
        assert_eq!(1, polymer.insertions.len());
        assert_eq!(
            vec![(('A', 'B'), 'C')],
            polymer.insertions.into_iter().collect_vec()
        );
        assert_eq!(1, polymer.counts[&'A']);
        assert_eq!(1, polymer.counts[&'B']);
    }

    #[test]
    fn should_step() {
        let input = "AB

AB -> C";
        let mut polymer = input.parse::<Polymer>().unwrap();
        polymer.step();
        assert_eq!(3, polymer.pairs.len());
        assert_eq!(
            vec![&('A', 'B'), &('A', 'C'), &('C', 'B')],
            polymer.pairs.keys().sorted().collect_vec()
        );
        assert_eq!(0, polymer.pairs[&('A', 'B')]);
        assert_eq!(1, polymer.pairs[&('A', 'C')]);
        assert_eq!(1, polymer.pairs[&('C', 'B')]);
        assert_eq!(1, polymer.counts[&'A']);
        assert_eq!(1, polymer.counts[&'B']);
        assert_eq!(1, polymer.counts[&'C']);
    }

    #[test]
    fn should_keep_count() {
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
        polymer.step();
        assert_eq!(7, polymer.len());
        polymer.step();
        assert_eq!(13, polymer.len());
        polymer.step();
        assert_eq!(25, polymer.len());
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
        assert_eq!(1749, polymer.counts[&'B']);
        assert_eq!(298, polymer.counts[&'C']);
        assert_eq!(161, polymer.counts[&'H']);
        assert_eq!(865, polymer.counts[&'N']);

        assert_eq!(3073, polymer.len());

        let range = polymer.get_range();
        assert_eq!(1588, range);
    }

    #[test]
    fn should_solve_part2_example() {
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
        polymer.step_for(40);

        assert_eq!(2192039569602, polymer.counts[&'B']);
        assert_eq!(3849876073, polymer.counts[&'H']);

        let range = polymer.get_range();
        assert_eq!(2188189693529, range);
    }
}
