use std::fs;

pub fn part1() {
    get_product(2);
}

pub fn part2() {
    get_product(3);
}

fn get_product(num_of_numbers: i32) -> i32 {
    let data = "data/day1.txt";

    let read = fs::read_to_string(data).expect("Failed to read data file!");

    let parsed: Vec<i32> = read
        .lines()
        .map(|x: &str| x.parse::<i32>().unwrap())
        .collect();

    let target = 2020;

    let numbers = find_smarter(&parsed, 0, target, num_of_numbers - 1);
    let display: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();

    println!("Got: {}", display.join(", "));
    println!("Sanity check: {}", sum);

    println!();
    println!("Answer: {}", product);

    product
}

fn find_smarter(slice: &[i32], current: i32, target: i32, depth: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
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
            let inner = find_smarter(&slice[idx..], val + current, target, next_depth);
            if inner.len() == 0 {
                continue;
            }
            result.push(val);
            result.extend(inner);
            break;
        }

        if val + current == target {
            result.push(val);
            break;
        }

    }
    result
}
