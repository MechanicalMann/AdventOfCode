use anyhow::Result;
use solver::Solver;

mod days;
mod input;
mod solver;

fn main() -> Result<()> {
    days::day01::Solution::new().solve()?;
    days::day02::Solution::new().solve()?;
    days::day03::Solution::new().solve()?;
    days::day04::Solution::new().solve()?;
    days::day05::Solution::new().solve()?;
    days::day06::Solution::new().solve()?;
    days::day07::Solution::new().solve()?;
    days::day08::Solution::new().solve()?;
    days::day09::Solution::new().solve()?;
    days::day10::Solution::new().solve()?;
    days::day11::Solution::new().solve()?;
    days::day12::Solution::new().solve()?;
    days::day13::Solution::new().solve()?;
    days::day14::Solution::new().solve()?;
    Ok(())
}
