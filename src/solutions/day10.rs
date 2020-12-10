use std::fs;
use std::str::FromStr;

pub fn part1() {
    let datafile = "data/day10.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut values: Vec<_> = read
        .lines()
        .map(usize::from_str)
        .map(|x| x.unwrap())
        .collect();
    values.sort();
    let mut diffs = [0, 0, 1]; // Built-in adapter counts for a difference of 3
    match values[0] {
        v @ 1..=3 => diffs[v - 1] += 1,
        _ => panic!("Invalid adapter!"),
    }
    for i in 1..values.len() {
        let diff = values[i] - values[i - 1];
        diffs[diff - 1] += 1;
    }
    println!(
        "{} differences of 1, {} differences of 3",
        diffs[0], diffs[2]
    );
    println!("Product: {}", diffs[0] * diffs[2]);
}

pub fn part2() {
    //
}
