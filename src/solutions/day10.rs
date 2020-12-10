use std::cmp::max;
use std::fs;
use std::str::FromStr;

fn load_data() -> Vec<usize> {
    let datafile = "data/day10.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut values: Vec<_> = read
        .lines()
        .map(usize::from_str)
        .map(|x| x.unwrap())
        .collect();
    values.sort();
    values
}

pub fn part1() {
    let values = load_data();
    let mut diffs = [0, 0, 1]; // Built-in adapter counts for a difference of 3
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

fn get_paths_from(current: usize, values: &Vec<usize>, paths: &mut Vec<usize>) {
    for i in current + 1..values.len() {
        match values[i] - values[current] {
            1..=3 => paths[i] += max(paths[current], 1),
            _ => break,
        }
    }
}

pub fn part2() {
    let values = load_data();
    let mut paths = vec![0; values.len()];
    get_paths_from(0, &values, &mut paths); // seed the first few numbers (paths from assumed 0)
    for i in 0..values.len() {
        get_paths_from(i, &values, &mut paths);
        // println!("{} possible paths to {}", paths[i], values[i]);
    }
    println!("{} total combinations!", paths[paths.len() - 1]);
}
