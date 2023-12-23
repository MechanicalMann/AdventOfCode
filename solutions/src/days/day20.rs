use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::solver::Solver;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 20;
    const TITLE: &'static str = "Pulse Propagation";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut bus = self.input().get_as::<Bus>()?;
        let (low, high) = bus.push_cycle(1000);
        Ok(low * high)
    }

    fn part_two(&self) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

trait Module: std::fmt::Debug {
    fn pulse(&mut self, pulse: Pulse, from: &u16) -> Option<Pulse>;
}

#[derive(Debug)]
struct FlipFlop {
    on: bool,
}
impl Module for FlipFlop {
    fn pulse(&mut self, pulse: Pulse, _: &u16) -> Option<Pulse> {
        match pulse {
            Pulse::Low => {
                self.on = !self.on;
                match self.on {
                    true => Some(Pulse::High),
                    false => Some(Pulse::Low),
                }
            }
            Pulse::High => None,
        }
    }
}
impl FlipFlop {
    fn new() -> Self {
        FlipFlop { on: false }
    }
}

#[derive(Debug)]
struct Broadcast;
impl Module for Broadcast {
    fn pulse(&mut self, pulse: Pulse, _: &u16) -> Option<Pulse> {
        Some(pulse)
    }
}

#[derive(Debug)]
struct Conjunction {
    memory: HashMap<u16, Pulse>,
}
impl Module for Conjunction {
    fn pulse(&mut self, pulse: Pulse, from: &u16) -> Option<Pulse> {
        self.memory.insert(*from, pulse);
        match self.memory.values().all(|&p| p == Pulse::High) {
            true => Some(Pulse::Low),
            false => Some(Pulse::High),
        }
    }
}
impl Conjunction {
    fn new() -> Self {
        Conjunction {
            memory: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Bus {
    modules: HashMap<u16, Box<dyn Module>>,
    cables: HashMap<u16, Vec<u16>>,
}
impl FromStr for Bus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut modules: HashMap<u16, Box<dyn Module>> = HashMap::new();
        let mut cables = HashMap::new();
        let mut conjunctions = vec![];

        for l in s.lines() {
            let parts = l.split(" -> ").collect_vec();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid wiring bus specification"));
            }
            let id;
            let outputs = parts[1]
                .split(", ")
                .filter_map(|s| to_id(s).ok())
                .collect_vec();
            if parts[0] == "broadcaster" {
                id = 0;
                modules.insert(id, Box::new(Broadcast {}));
            } else {
                id = to_id(&parts[0][1..]).unwrap();
                match &parts[0][0..1] {
                    "%" => {
                        modules.insert(id, Box::new(FlipFlop::new()));
                    }
                    "&" => {
                        let conj = Conjunction::new();
                        conjunctions.push((id, conj));
                    }
                    _ => (),
                }
            }
            cables.insert(id, outputs);
        }

        // Initialize conjunctions
        while let Some((cid, mut conj)) = conjunctions.pop() {
            for input in cables
                .iter()
                .filter(|(_, v)| v.contains(&cid))
                .map(|(&k, _)| k)
            {
                conj.memory.insert(input, Pulse::Low);
            }
            modules.insert(cid, Box::new(conj));
        }

        Ok(Self { modules, cables })
    }
}
impl Bus {
    fn push_button(&mut self) -> (usize, usize) {
        let (mut low, mut high) = (0, 0);
        let mut queue = VecDeque::from_iter([(0, 0, Pulse::Low)]);
        while let Some((from, to, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }
            let Some(module) = self.modules.get_mut(&to) else {
                continue;
            };
            let Some(output) = module.pulse(pulse, &from) else {
                continue;
            };
            for &next in self.cables.get(&to).unwrap() {
                queue.push_back((to, next, output));
            }
        }
        (low, high)
    }

    fn push_cycle(&mut self, times: usize) -> (usize, usize) {
        let (mut total_low, mut total_high) = (0, 0);
        for _ in 0..times {
            let (low, high) = self.push_button();
            total_low += low;
            total_high += high;
        }
        (total_low, total_high)
    }
}

fn to_id(s: &str) -> Result<u16> {
    let id = u16::from_str_radix(s, 36)?;
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn should_parse() -> Result<()> {
        let test = "broadcaster -> a, b\n%a -> b\n&b -> a".parse::<Bus>()?;
        assert_eq!(3, test.cables.len());
        assert_eq!(3, test.modules.len());
        println!("{test:?}");
        let expected_cables =
            HashMap::from_iter([(0, vec![10, 11]), (10, vec![11]), (11, vec![10])]);
        let expected_ids = vec![&0u16, &10, &11];
        assert_eq!(expected_cables, test.cables);
        assert_eq!(expected_ids, test.modules.keys().sorted().collect_vec());
        Ok(())
    }

    #[test]
    fn should_pulse() -> Result<()> {
        let mut test = "broadcaster -> a, b\n%a -> b\n&b -> a".parse::<Bus>()?;
        let (low, high) = test.push_button();
        assert_eq!(3, low);
        assert_eq!(3, high);
        Ok(())
    }

    #[test]
    fn should_solve_part1() -> Result<()> {
        let mut bus = EXAMPLE_INPUT.parse::<Bus>()?;
        let (low, high) = bus.push_button();
        assert_eq!(8, low);
        assert_eq!(4, high);
        Ok(())
    }
}
