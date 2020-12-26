use std::collections::HashMap;
use std::fs;

struct CrabCups {
    head: Link,
    index: HashMap<u32, Cup>,
}
impl CrabCups {
    fn new() -> Self {
        CrabCups {
            head: None,
            index: HashMap::new(),
        }
    }

    fn from(cups: &[u32]) -> Self {
        if cups.len() == 0 {
            return CrabCups::new();
        }

        let first = cups[0];
        let mut index = HashMap::new();
        for i in 0..cups.len() - 1 {
            let cup = Cup {
                value: cups[i],
                next: Some(cups[i + 1]),
            };
            index.insert(cups[i], cup);
        }
        let last = cups[cups.len() - 1];
        index.insert(
            last,
            Cup {
                value: last,
                next: Some(first),
            },
        );
        CrabCups {
            head: Some(first),
            index,
        }
    }

    fn pick_up(&mut self) -> Option<[u32; 3]> {
        let hi = self.head?;
        let mut values = [0; 3];
        let mut cur = hi;
        for i in 0..3 {
            cur = self.index[&cur].next?;
            values[i] = cur;
        }
        let next = self.index[&values[2]].next;
        let head = self.index.get_mut(&hi)?;
        head.next = next;
        Some(values)
    }

    fn place(&mut self, target: u32, values: &[u32; 3]) {
        let mut node = self.index.get_mut(&target).unwrap();
        let prev = node.next.unwrap();
        node.next = Some(values[0]);

        let mut last = self.index.get_mut(&values[2]).unwrap();
        last.next = Some(prev);
    }

    fn step(&mut self) {
        self.head = self.index[&self.head.unwrap()].next;
    }

    fn len(&self) -> usize {
        self.index.len()
    }

    fn get_slice(&self, after: u32, length: usize) -> Vec<u32> {
        let mut values = vec![];
        let mut cur = after;
        for _ in 0..length {
            cur = self.index[&cur].next.unwrap();
            values.push(cur);
        }
        values
    }
}

struct Cup {
    value: u32,
    next: Link,
}
impl Cup {
    fn new(value: u32) -> Self {
        Cup { value, next: None }
    }
}

type Link = Option<u32>; // he come to town

fn predict(cups: &mut CrabCups, moves: usize) {
    let (min, max) = (
        *cups.index.keys().min().unwrap(),
        *cups.index.keys().max().unwrap(),
    );
    println!("Begin!");
    for _ in 0..moves {
        let current = cups.head.unwrap();
        let holding = cups.pick_up().unwrap();
        let mut target = if current == min { max } else { current - 1 };
        while holding.contains(&target) {
            target = if target <= min { max } else { target - 1 };
        }
        // println!("Target: {}", target);
        cups.place(target, &holding);
        cups.step();
    }
}

fn get_cup_string(cups: &[u32], spaces: bool) -> String {
    cups.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(if spaces { " " } else { "" })
}

fn load_data() -> Vec<u32> {
    let datafile = "data/day23.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut cups: Vec<u32> = vec![];
    for c in read.chars() {
        cups.push(c.to_digit(10).unwrap());
    }
    cups
}

pub fn part1() {
    println!("Time for Crab: Return of the Crab");
    let data = load_data();
    let mut cups = CrabCups::from(&data);
    predict(&mut cups, 100);
    println!("\n-- Final --");
    // println!("Cups: {}", get_cup_string(, true));
    let answer = cups.get_slice(1, 8);
    println!("Answer: {}", get_cup_string(&answer, false));
}

pub fn part2() {
    println!("\n\nTime for Crab: Revenge of the Crab");
    let mut data = load_data();
    let start = *data.iter().max().unwrap();
    for i in start..1_000_000 {
        data.push(i + 1);
    }
    let mut cups = CrabCups::from(&data);
    predict(&mut cups, 10_000_000);
    println!("\n-- Final --");
    let mut final_cups = vec![];
    let mut answer: usize = 1;
    let mut cup = &cups.index[&1];
    for _ in 0..2 {
        cup = &cups.index[&cup.next.unwrap()];
        final_cups.push(cup);
        answer *= cup.value as usize;
    }
    println!(
        "Stars are under: {}",
        get_cup_string(
            &final_cups.iter().map(|c| c.value).collect::<Vec<u32>>(),
            true
        )
    );
    println!("Answer: {}", answer);
}
