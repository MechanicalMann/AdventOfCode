use std::fs;

pub fn part1() {
    let data = "data/day1.1.txt";

    let read = fs::read_to_string(data).expect("Failed to read data file!");

    let parsed: Vec<i32> = read
        .lines()
        .map(|x: &str| x.parse::<i32>().unwrap())
        .collect();

    let target = 2020;

    let numbers = find_numbers(&parsed, target);

    let (left, right) = numbers;
    println!("Got: {}, {}", left, right);
    println!("Sanity check: {}", left + right);

    println!();
    println!("Answer: {}", left * right);
}

fn find_numbers(parsed: &[i32], target: i32) -> (i32, i32) {
    let mut idx = 0;
    while idx < parsed.len() - 1 {
        let cur = parsed[idx];
        if cur > target {
            continue;
        }
        for next in (idx + 1)..parsed.len() {
            let n = parsed[next];
            if n >= target { continue; }
            if cur + n == target {
                return (cur, n)
            }
        }
        idx += 1;
    }
    (0, 0)
}
