use std::collections::HashMap;
use std::fs;

pub fn part1() {
    let datafile = "data/day6.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let groups: Vec<_> = read.split("\n\n").collect();
    let sum: usize = groups.iter().map(|x| get_sum_of_yes(x)).sum();
    println!("Total where any answered yes: {}", sum);
}

pub fn part2() {
    let datafile = "data/day6.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let groups: Vec<_> = read.split("\n\n").collect();
    let sum: usize = groups.iter().map(|x| get_sum_of_everyone(x)).sum();
    println!("Total where all answered yes: {}", sum);
}

fn get_sum_of_yes(group: &str) -> usize {
    let mut answers: HashMap<char, bool> = HashMap::new();
    for c in group.replace("\n", "").chars() {
        answers.entry(c).or_insert(true);
    }
    answers.len()
}

fn get_sum_of_everyone(group: &str) -> usize {
    let mut answers: HashMap<char, u8> = HashMap::new();
    let mut total = 0u8;
    for line in group.lines() {
        total += 1;
        for c in line.chars() {
            let q = answers.entry(c).or_insert(0);
            *q += 1;
        }
    }
    answers.values().filter(|&x| *x == total).count()
}
