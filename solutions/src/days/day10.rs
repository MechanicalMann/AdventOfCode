use std::str::FromStr;

use crate::solver::Solver;
use anyhow::Result;
use itertools::Itertools;

pub struct Solution;
impl Solver<isize, String> for Solution {
    const DAY: u8 = 10;

    const TITLE: &'static str = "Cathode-Ray Tube";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<isize> {
        let program = self.input().get_lines_as::<Opcode>()?;
        let mut cpu = Cpu::new();
        let signals = find_signals(&mut cpu, &program);
        Ok(signals.iter().sum())
    }

    fn part_two(&self) -> Result<String> {
        let program = self.input().get_lines_as::<Opcode>()?;
        let mut cpu = Cpu::new();
        let pixels = get_pixels(&mut cpu, &program);
        Ok(draw_crt(&pixels))
    }
}

fn sample_program(
    cpu: &mut Cpu,
    program: &[Opcode],
    start: usize,
    freq: usize,
) -> Vec<(usize, isize)> {
    let mut register_vals = vec![];
    let mut next = start;
    for &i in program {
        cpu.instruction = Some(i);
        while matches!(cpu.instruction, Some(_)) {
            cpu.tick();
            if cpu.cycle == next {
                register_vals.push((next, cpu.register));
                next += freq;
            }
        }
    }
    register_vals
}

fn find_signals(cpu: &mut Cpu, program: &[Opcode]) -> Vec<isize> {
    let values = sample_program(cpu, program, 20, 40);
    values
        .iter()
        .map(|&(cyc, val)| isize::try_from(cyc).expect("Cycle overflow") * val)
        .collect_vec()
}

fn get_pixels(cpu: &mut Cpu, program: &[Opcode]) -> Vec<(usize, usize)> {
    let mut illuminated = vec![];
    let (mut line, mut col) = (0, 0);
    for &i in program {
        cpu.instruction = Some(i);
        while matches!(cpu.instruction, Some(_)) {
            cpu.tick();
            let sprite = (cpu.register - 1, cpu.register, cpu.register + 1);
            if col == sprite.0 || col == sprite.1 || col == sprite.2 {
                illuminated.push((line, col as usize));
            }
            if col == 39 {
                line += 1;
                col = 0
            } else {
                col += 1;
            }
        }
    }
    illuminated
}

fn draw_crt(pixels: &[(usize, usize)]) -> String {
    let mut output = String::from("\n");
    for line in 0..6 {
        for col in 0..40 {
            if pixels.contains(&(line, col)) {
                output.push('#');
            } else {
                output.push('.')
            }
        }
        output.push('\n');
    }
    output
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Opcode {
    Noop,
    AddX(isize),
}
impl FromStr for Opcode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(' ').collect_vec();
        if split.len() < 1 || split.len() > 2 {
            return Err(anyhow!("Invalid instruction: {}", s));
        }

        match split[0] {
            "noop" => Ok(Opcode::Noop),
            "addx" => {
                let val = split[1].parse::<isize>()?;
                Ok(Opcode::AddX(val))
            }
            _ => Err(anyhow!("Invalid instruction: {}", s)),
        }
    }
}

struct Cpu {
    cycle: usize,
    register: isize,
    next: Option<isize>,
    stack: Vec<isize>,
    instruction: Option<Opcode>,
}
impl Cpu {
    fn new() -> Cpu {
        Cpu {
            cycle: 0,
            register: 1,
            next: None,
            stack: vec![],
            instruction: None,
        }
    }

    fn tick(&mut self) {
        match self.next {
            Some(reg) => self.register = reg,
            None => (),
        }
        match self.instruction {
            Some(i) => match i {
                Opcode::AddX(value) => match self.stack.pop() {
                    Some(stack_val) => {
                        self.next = Some(self.register + stack_val);
                        self.instruction = None
                    }
                    None => self.stack.push(value),
                },
                Opcode::Noop => self.instruction = None,
            },
            None => (),
        }
        self.cycle += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let noop = "noop";
        let expected_noop = Opcode::Noop;
        let actual_noop = noop.parse::<Opcode>().unwrap();
        assert_eq!(expected_noop, actual_noop);

        let addx = "addx -42";
        let expected_addx = Opcode::AddX(-42);
        let actual_addx = addx.parse::<Opcode>().unwrap();
        assert_eq!(expected_addx, actual_addx);
    }

    #[test]
    fn should_do_nothing() {
        let mut cpu = Cpu::new();
        cpu.tick();
        assert_eq!(1, cpu.cycle);
        assert_eq!(1, cpu.register);
    }

    #[test]
    fn should_noop() {
        let mut cpu = Cpu::new();
        cpu.instruction = Some(Opcode::Noop);
        cpu.tick();
        assert_eq!(1, cpu.cycle);
        assert_eq!(1, cpu.register);
        assert_eq!(None, cpu.instruction);
    }

    #[test]
    fn should_add() {
        let mut cpu = Cpu::new();
        cpu.instruction = Some(Opcode::AddX(41));
        cpu.tick();
        assert_eq!(1, cpu.cycle);
        assert_eq!(1, cpu.register);
        assert!(itertools::equal(&[41], &cpu.stack));
        cpu.tick();
        assert_eq!(2, cpu.cycle);
        assert_eq!(1, cpu.register);
        cpu.tick();
        assert_eq!(3, cpu.cycle);
        assert_eq!(42, cpu.register);
    }

    #[test]
    fn should_sample() {
        let mut cpu = Cpu::new();
        let program = vec![Opcode::Noop, Opcode::AddX(41), Opcode::Noop];
        let samples = sample_program(&mut cpu, &program, 2, 2);
        assert!(itertools::equal([(2, 1), (4, 42)], samples));
    }

    #[test]
    fn should_solve_part_1() {
        let mut cpu = Cpu::new();
        let program = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Opcode>().unwrap())
            .collect_vec();
        let signals = find_signals(&mut cpu, &program);
        let expected = vec![420, 1140, 1800, 2940, 2880, 3960];
        assert!(itertools::equal(expected, signals));
    }

    #[test]
    fn should_illuminate() {
        let mut cpu = Cpu::new();
        let program = vec![Opcode::AddX(15), Opcode::AddX(-11)];
        let expected = vec![(0, 0), (0, 1)];
        let actual = get_pixels(&mut cpu, &program);
        assert!(itertools::equal(expected, actual));
    }

    #[test]
    fn should_draw() {
        let pixels = vec![(2, 19), (2, 20), (3, 19), (3, 20)];
        let expected = "
........................................
........................................
...................##...................
...................##...................
........................................
........................................
";
        let actual = draw_crt(&pixels);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_solve_part_2() {
        let mut cpu = Cpu::new();
        let program = EXAMPLE_INPUT
            .lines()
            .map(|l| l.parse::<Opcode>().unwrap())
            .collect_vec();
        let illuminated = get_pixels(&mut cpu, &program);
        let actual = draw_crt(&illuminated);
        assert_eq!(EXAMPLE_OUTPUT, actual);
    }

    const EXAMPLE_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const EXAMPLE_OUTPUT: &str = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
}
