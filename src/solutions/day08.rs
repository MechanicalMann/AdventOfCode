use anyhow::*;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Token {
    NOP,
    ACC,
    JMP,
}
impl FromStr for Token {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "nop" => Ok(Token::NOP),
            "acc" => Ok(Token::ACC),
            "jmp" => Ok(Token::JMP),
            _ => bail!("Invalid instruction: {}", s),
        }
    }
}

struct State {
    acc: isize,
    exp: isize,
}
impl State {
    fn accumulate(&mut self, val: &isize) -> isize {
        self.acc += val;
        self.acc
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) -> Result<isize> {
        let len = instructions.len() as isize;
        let mut hist: HashSet<isize> = HashSet::new();
        while self.exp >= 0 && self.exp < len {
            let i = &instructions[self.exp as usize];
            if hist.contains(&self.exp) {
                bail!(
                    "Loop detected on line: {} (current state: {})",
                    i.line,
                    self.acc
                );
            }
            hist.insert(self.exp);
            self.exp += i.exec(self)?;
        }
        Ok(self.acc)
    }
}

#[derive(Clone)]
struct Instruction {
    line: usize,
    token: Token,
    value: isize,
}
impl Instruction {
    fn from_str(s: &str, line: usize) -> Result<Self> {
        let split: Vec<_> = s.split(" ").collect();
        if split.len() != 2 {
            bail!("Invalid instruction: {}", s)
        }
        let token = Token::from_str(split[0])?;
        let value: isize = split[1].parse()?;
        Ok(Instruction { line, token, value })
    }

    fn exec(&self, state: &mut State) -> Result<isize> {
        match self.token {
            Token::NOP => {}
            Token::ACC => {
                state.accumulate(&self.value);
            }
            Token::JMP => {
                return Ok(self.value);
            }
        }
        Ok(1)
    }
}

fn load_program() -> Vec<Instruction> {
    let datafile = "data/day8.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let program: Vec<_> = read
        .lines()
        .enumerate()
        .map(|(idx, txt)| Instruction::from_str(txt, idx + 1))
        .map(|x| x.unwrap())
        .collect();
    program
}

pub fn part1() {
    let program = load_program();
    let mut machine = State { acc: 0, exp: 0 };
    match machine.execute(&program) {
        Ok(v) => println!("Done.  Accumulated value: {}", v),
        Err(e) => println!("An error occurred: {}", e),
    };
}

pub fn part2() {
    let program = load_program();
    let mut tested = 0;
    loop {
        if tested >= program.len() - 1 {
            println!("No corrupted instructions found!");
            break;
        }
        let mut test_prog = program.clone();
        for (i, ins) in test_prog.iter_mut().enumerate() {
            if i <= tested {
                continue;
            }
            ins.token = match ins.token {
                Token::ACC => continue,
                Token::NOP => Token::JMP,
                Token::JMP => Token::NOP,
            };
            tested = i;
            println!(
                "Swapping line {} to {:?} and trying...",
                ins.line, ins.token
            );
            break;
        }
        let mut machine = State { acc: 0, exp: 0 };
        match machine.execute(&test_prog) {
            Ok(v) => {
                println!("Done.  Accumulated value: {}", v);
                break;
            }
            Err(e) => {
                println!("An error occurred: {}", e);
            }
        };
    }
}
