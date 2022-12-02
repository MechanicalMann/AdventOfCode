use crate::input::AdventInput;
use anyhow::Result;
use gag::Gag;
use std::{
    fmt::Display,
    marker::PhantomData,
    time::{Duration, Instant},
};
pub trait Solver<'a, T1: Display, T2: Display> {
    const DAY: u8;
    const TITLE: &'a str;

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
        println!("{}: {}", self.describe_part_one(), result);
    }
    fn print_part_two<T: Display>(&self, result: T) {
        println!("{}: {}", self.describe_part_two(), result);
    }

    fn part_one(&self) -> Result<T1>;
    fn part_two(&self) -> Result<T2>;

    fn describe(&self) -> String {
        format!("Day {:02}", Self::DAY)
    }
    fn describe_part(&self, part: isize) -> String {
        format!("{} Part {:02}", self.describe(), part)
    }
    fn describe_part_one(&self) -> String {
        self.describe_part(1)
    }
    fn describe_part_two(&self) -> String {
        self.describe_part(2)
    }
}

pub trait Measurable {
    fn title(&self) -> String;
    fn describe_part_one(&self) -> String;
    fn describe_part_two(&self) -> String;
    fn time_part_one(&self) -> Result<Duration>;
    fn time_part_two(&self) -> Result<Duration>;
}

pub struct Measure<T: for<'a> Solver<'a, T1, T2>, T1: Display, T2: Display> {
    solver: T,
    _p1: PhantomData<T1>,
    _p2: PhantomData<T2>,
}
impl<T: for<'a> Solver<'a, T1, T2>, T1: Display, T2: Display> Measurable for Measure<T, T1, T2> {
    fn title(&self) -> String {
        String::from(T::TITLE)
    }
    fn describe_part_one(&self) -> String {
        self.solver.describe_part_one()
    }
    fn describe_part_two(&self) -> String {
        self.solver.describe_part_two()
    }
    fn time_part_one(&self) -> Result<Duration> {
        time_execution(|| self.solver.part_one())
    }
    fn time_part_two(&self) -> Result<Duration> {
        time_execution(|| self.solver.part_two())
    }
}
impl<T: 'static + for<'a> Solver<'a, T1, T2>, T1: 'static + Display, T2: 'static + Display>
    Measure<T, T1, T2>
{
    pub fn get(solver: T) -> Box<dyn Measurable> {
        Box::new(Measure {
            solver,
            _p1: PhantomData {},
            _p2: PhantomData {},
        })
    }
}

fn time_execution<F: Fn() -> Result<T>, T>(f: F) -> Result<Duration> {
    let gag = Gag::stdout()?;
    let now = Instant::now();
    f()?;
    let res = now.elapsed();
    drop(gag);
    Ok(res)
}
