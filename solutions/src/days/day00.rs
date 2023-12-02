// template for future days
use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 0;
    const TITLE: &'static str = "UNKNOWN";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        Ok(0)
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
