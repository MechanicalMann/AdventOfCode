use std::str::FromStr;

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 6;
    const TITLE: &'static str = "Wait For It";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let record = self.input().get_as::<RecordSheet>()?;
        Ok(get_win_factor(&record))
    }

    fn part_two(&self) -> Result<usize> {
        let race = self.input().get_as::<Race>()?;
        Ok(get_win_possibilities(&race))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Race { time, distance }
    }
}
impl FromStr for Race {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect_vec();
        if lines.len() != 2 {
            return Err(anyhow!("Invalid race"));
        }
        let time = lines[0].split_whitespace().skip(1).join("").parse()?;
        let distance = lines[1].split_whitespace().skip(1).join("").parse()?;
        Ok(Race { time, distance })
    }
}

struct RecordSheet {
    races: Vec<Race>,
}
impl FromStr for RecordSheet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().collect_vec();
        if lines.len() != 2 {
            return Err(anyhow!("Invalid record sheet"));
        }
        let times = lines[0].split_whitespace().collect_vec();
        let dists = lines[1].split_whitespace().collect_vec();
        if times.len() != dists.len() {
            return Err(anyhow!("Invalid race specifications"));
        }
        let mut races = vec![];
        for (&t, &d) in times.iter().skip(1).zip(dists.iter().skip(1)) {
            let (time, distance) = (t.parse()?, d.parse()?);
            races.push(Race::new(time, distance));
        }
        Ok(RecordSheet { races })
    }
}

fn get_range_to_win(race: &Race) -> (usize, usize) {
    let mut min_win: Option<usize> = None;
    let mut max_win: Option<usize> = None;

    for press in (1..race.time).rev() {
        if press * (race.time - press) <= race.distance {
            continue;
        }
        max_win = Some(press);
        break;
    }
    for press in 1..race.time {
        if press * (race.time - press) <= race.distance {
            continue;
        }
        min_win = Some(press);
        break;
    }

    match (min_win, max_win) {
        (Some(min), Some(max)) => (min, max),
        _ => (0, 0),
    }
}

fn get_win_possibilities(race: &Race) -> usize {
    let (min, max) = get_range_to_win(race);
    if min == usize::MAX {
        return 0;
    }
    max + 1 - min
}

fn get_win_factor(record: &RecordSheet) -> usize {
    record
        .races
        .iter()
        .map(|r| get_win_possibilities(r))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "Times:       1 2 3\nDistances: 4 5 6";
        let record = test.parse::<RecordSheet>()?;
        let expected = vec![Race::new(1, 4), Race::new(2, 5), Race::new(3, 6)];
        assert_eq!(expected, record.races);
        Ok(())
    }

    #[test]
    fn should_get_win_range() -> Result<()> {
        let test = "Times: 7\nDistances: 9";
        let record = test.parse::<RecordSheet>()?;
        let range = get_range_to_win(&record.races[0]);
        assert_eq!((2, 5), range);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let record = EXAMPLE_INPUT.parse::<RecordSheet>()?;
        let win_factor = get_win_factor(&record);
        assert_eq!(288, win_factor);
        Ok(())
    }

    #[test]
    fn should_parse_race() -> Result<()> {
        let test = "Times:       1 2 3\nDistances: 4 5 6";
        let race = test.parse::<Race>()?;
        assert_eq!(Race::new(123, 456), race);
        Ok(())
    }

    #[test]
    fn should_solve_part2() -> Result<()> {
        let race = EXAMPLE_INPUT.parse::<Race>()?;
        let possibilities = get_win_possibilities(&race);
        assert_eq!(71503, possibilities);
        Ok(())
    }
}
