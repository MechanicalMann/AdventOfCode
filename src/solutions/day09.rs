use anyhow::*;
use std::fs;
use std::str::FromStr;

// "Borrowing" this from day 1
// I still can't decide if I should refactor while the challenge is still going
fn find_smarter(
    slice: &[isize],
    current: isize,
    target: isize,
    depth: isize,
) -> Result<Vec<isize>> {
    let mut idx: usize = 0;
    let len = slice.len();
    while idx < len {
        let val = slice[idx];
        idx += 1;
        if val >= target {
            continue;
        }
        if depth > 0 {
            let next_depth = depth - 1;
            let inner = match find_smarter(&slice[idx..], val, target, next_depth) {
                Err(_) => continue,
                Ok(v) => v,
            };
            let mut result = vec![val];
            result.extend(inner);
            return Ok(result);
        } else if val + current == target {
            return Ok(vec![val]);
        }
    }
    bail!("No numbers found!");
}

fn checksum(input: &Vec<isize>, window: usize) -> Result<()> {
    let max = input.len();
    if window >= max {
        bail!("Invalid input: too small!");
    }
    let mut target_idx = window;
    let mut start_idx = 0;
    while target_idx < max {
        let target = input[target_idx];
        let slice = &input[start_idx..(start_idx + window + 1)];
        match find_smarter(slice, 0, target, 1) {
            Ok(_) => (),
            Err(_) => bail!("Invalid value found: {}", target),
        }
        target_idx += 1;
        start_idx += 1;
    }
    Ok(())
}

fn attack(input: &Vec<isize>, target: isize) -> Result<&'_ [isize]> {
    let max = input.len();
    // extremely naive solution incoming
    let mut start_idx = 0;
    while start_idx < max - 1 {
        let mut current = input[start_idx];
        if current >= target {
            continue;
        }
        for i in (start_idx + 1)..max {
            current += input[i];
            if current == target {
                let result = &input[start_idx..(i + 1)];
                return Ok(result);
            }
        }
        start_idx += 1;
    }
    bail!("No attack vector found");
}

pub fn part1() {
    let datafile = "data/day9.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let values: Vec<_> = read
        .lines()
        .map(isize::from_str)
        .map(|x| x.unwrap())
        .collect();
    match checksum(&values, 25) {
        Ok(()) => println!("No answer found!"),
        Err(e) => println!("Error occurred: {}", e),
    };
}

pub fn part2() {
    let datafile = "data/day9.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let values: Vec<_> = read
        .lines()
        .map(isize::from_str)
        .map(|x| x.unwrap())
        .collect();
    let mut numbers = match attack(&values, 26796446) {
        Ok(v) => v.to_owned(),
        Err(_) => panic!("Failed to find an attack..."),
    };
    numbers.sort();
    let first = numbers[0];
    let last = numbers[numbers.len() - 1];
    println!("Answer: {}", first + last);
}
