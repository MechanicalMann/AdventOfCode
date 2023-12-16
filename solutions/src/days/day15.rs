use crate::solver::Solver;
use anyhow::Result;

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
        Ok(0)
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
}
