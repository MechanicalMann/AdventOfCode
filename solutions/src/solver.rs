use crate::input::AdventInput;
use anyhow::Result;
use std::fmt::Display;

pub trait Solver<T1: Display, T2: Display> {
    const DAY: u8;

    fn input(&self) -> AdventInput {
        AdventInput::for_day(Self::DAY)
    }

    fn solve(&self) -> Result<()> {
        let part1 = self.part_one()?;
        self.print_part_one(part1);
        let part2 = self.part_two()?;
        self.print_part_two(part2);
        Ok(())
    }

    fn new() -> Self;

    fn print_part_one<T: Display>(&self, result: T) {
        println!("Day {:02} Part 01: {}", Self::DAY, result);
    }
    fn print_part_two<T: Display>(&self, result: T) {
        println!("Day {:02} Part 02: {}", Self::DAY, result);
    }

    fn part_one(&self) -> Result<T1>;
    fn part_two(&self) -> Result<T2>;
}
