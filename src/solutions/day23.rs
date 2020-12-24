use std::collections::HashMap;
use std::fs;

fn predict(cups: &mut Vec<u32>, moves: usize) {
    let length = cups.len();
    let (mut min, mut max) = (10, 0);
    for c in cups.iter() {
        if c < &min {
            min = *c
        } else if c > &max {
            max = *c
        }
    }
    let mut index: HashMap<_, _> = cups
        .iter()
        .cloned()
        .enumerate()
        .map(|(x, y)| (y, x))
        .collect();
    println!("Begin!");
    for m in 0..moves {
        let i = m % length;
        let selected = [(i + 1) % length, (i + 2) % length, (i + 3) % length];
        let holding = [cups[selected[0]], cups[selected[1]], cups[selected[2]]];
        if m % 1000 == 0 {
            print!("\rMove {}", m + 1);
        }
        // println!("\n-- Move {} --", m + 1);
        // println!("Cups: {}", get_cup_string(&cups, true));
        // println!("Pick up: {}", get_cup_string(&holding, true));
        let current = cups[i];
        let mut target = if current == min { max } else { current - 1 };
        while holding.contains(&target) {
            target = if target <= min { max } else { target - 1 };
        }
        let pos = index[&target];
        if cups[pos] != target {
            println!(
                "Missed the target: target was {}, but got {} (index {})",
                target, cups[pos], pos
            );
            panic!("Failed to get correct target!");
        }
        // println!("Destination: {}", target);
        let new_pos = ((pos + length) - 3) % length;
        index.insert(target, new_pos);
        let new_places = [
            new_pos,
            (new_pos + 1) % length,
            (new_pos + 2) % length,
            (new_pos + 3) % length,
        ];
        let pmax = if new_pos < i {
            new_pos + length
        } else {
            new_pos
        };
        for idx in i + 1..pmax {
            let (idx, val) = (idx % length, cups[(idx + 3) % length]);
            cups[idx] = val;
            index.insert(val, idx);
        }
        for idx in &new_places {
            let idx = *idx;
            let val = if idx == new_pos {
                target
            } else {
                holding[((idx + length) - new_pos - 1) % length]
            };
            cups[idx] = val;
            index.insert(val, idx);
        }
    }
    println!("\rDone!                                                        ");
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
    let mut data = load_data();
    predict(&mut data, 100);
    println!("\n-- Final --");
    println!("Cups: {}", get_cup_string(&data, true));
    let mut answer = vec![];
    let offset = data.iter().position(|x| x == &1).unwrap();
    for i in 1..data.len() {
        answer.push(data[(i + offset) % data.len()]);
    }
    println!("Answer: {}", get_cup_string(&answer, false));
}
pub fn part2() {
    println!("\n\nTime for Crab: Revenge of the Crab");
    let mut data = load_data();
    let start = *data.iter().max().unwrap();
    for i in start..1_000_000 {
        data.push(i + 1);
    }
    predict(&mut data, 10_000_000);
    println!("\n-- Final --");
    println!("Cups: {}", get_cup_string(&data, true));
    let mut final_cups = vec![];
    let mut answer: usize = 1;
    let offset = data.iter().position(|x| x == &1).unwrap();
    for i in &[1, 2] {
        let cup = data[(i + offset) % data.len()];
        final_cups.push(cup);
        answer *= cup as usize;
    }
    println!("Stars are under: {}", get_cup_string(&final_cups, true));
    println!("Answer: {}", answer);
}
