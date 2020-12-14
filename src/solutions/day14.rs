use anyhow::*;
use std::collections::HashMap;
use std::fs;

enum Set {
    Mask(String),
    Assign(usize, usize),
}

struct Instruction {
    set: Set,
}
impl Instruction {
    fn from_str(s: &str) -> Self {
        let parts: Vec<_> = s.split(" = ").collect();
        let instructions = parts[0];
        let value = parts[1];
        match parts[0] {
            "mask" => Instruction {
                set: Set::Mask(parts[1].to_owned()),
            },
            _ => {
                let addr = instructions[4..(instructions.len() - 1)]
                    .parse::<usize>()
                    .unwrap();
                let value = value.parse::<usize>().unwrap();
                Instruction {
                    set: Set::Assign(addr, value),
                }
            }
        }
    }
}

struct Program {
    cur_mask: String,
    instructions: Vec<Instruction>,
    state: HashMap<usize, usize>,
    decode_memory: bool,
}
impl Program {
    fn run(&mut self) -> Result<usize> {
        for i in self.instructions.iter() {
            match &i.set {
                Set::Mask(m) => self.cur_mask = m.to_string(),
                Set::Assign(addr, val) if !self.decode_memory => {
                    self.state.insert(*addr, apply_mask(&self.cur_mask, *val));
                }
                Set::Assign(input, val) if self.decode_memory => {
                    for addr in decode_addresses(&self.cur_mask, *input) {
                        self.state.insert(addr, *val);
                    }
                }
                _ => bail!("Unknown instruction"),
            }
        }
        Ok(self.state.values().sum())
    }
}

fn apply_mask(mask: &str, num: usize) -> usize {
    let bits = mask.len() - 1;
    let mut val = num;
    for (i, c) in mask.chars().enumerate() {
        match c {
            '1' => val |= 0b1 << (bits - i),
            '0' => val &= !(0b1 << (bits - i)),
            _ => continue,
        }
    }
    val
}

fn decode_addresses(mask: &str, num: usize) -> Vec<usize> {
    let bits = mask.len() - 1;
    let mut addresses = vec![num];
    for (i, c) in mask.chars().enumerate() {
        let max = addresses.len();
        match c {
            '1' => {
                for n in 0..max {
                    addresses[n] |= 0b1 << (bits - i)
                }
            }
            'X' => {
                for n in 0..max {
                    addresses.push(addresses[n] | 0b1 << (bits - i));
                    addresses[n] &= !(0b1 << (bits - i));
                }
            }
            _ => continue,
        }
    }
    addresses
}

fn load_data() -> Vec<Instruction> {
    let datafile = "data/day14.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    read.lines().map(Instruction::from_str).collect()
}

pub fn part1() {
    let data = load_data();
    let mut program = Program {
        cur_mask: String::new(),
        instructions: data,
        state: HashMap::new(),
        decode_memory: false,
    };
    let result = program.run().unwrap();
    println!("Answer: {}", result);
}

pub fn part2() {
    let data = load_data();
    let mut program = Program {
        cur_mask: String::new(),
        instructions: data,
        state: HashMap::new(),
        decode_memory: true,
    };
    let result = program.run().unwrap();
    println!("Answer: {}", result);
}
