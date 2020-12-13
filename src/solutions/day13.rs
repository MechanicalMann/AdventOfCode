use anyhow::*;
use std::fs;

fn calc_next(target: usize, bus: usize) -> usize {
    let prev = target % bus;
    match prev {
        0 => target,
        _ => (target - prev) + bus,
    }
}

fn get_next_bus(target: usize, buses: &[usize]) -> Result<(usize, usize)> {
    let mut possibilities: Vec<(usize, usize)> = vec![];
    for bus in buses {
        let next = calc_next(target, *bus);
        possibilities.push((next - target, *bus));
    }
    if possibilities.len() < 1 {
        bail!("No buses were found close to the target time");
    }
    possibilities.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(possibilities[0])
}

fn get_sequential(buses: &Vec<(usize, usize)>) -> usize {
    let mut timestamp = 0;
    let mut multiplier = 1;
    for i in 0..(buses.len() - 1) {
        let (_, cur) = buses[i];
        let (offset, next) = buses[i + 1];
        multiplier *= cur;
        loop {
            timestamp += multiplier;
            if (timestamp + offset) % next == 0 {
                break;
            }
        }
    }
    timestamp
}

fn load_data_1() -> (usize, Vec<usize>) {
    let datafile = "data/day13.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut lines = read.lines();
    let target: usize = lines.next().unwrap().parse().unwrap();
    let timestamps: Vec<_> = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    (target, timestamps)
}

fn load_data_2() -> Vec<(usize, usize)> {
    let datafile = "data/day13.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let lines: Vec<_> = read.lines().collect();
    let sequence: Vec<_> = lines[1]
        .split(",")
        .enumerate()
        .filter(|&(_, c)| c != "x")
        .map(|(i, c)| (i, c.parse::<usize>().unwrap()))
        .collect();
    sequence
}

pub fn part1() {
    let (target_timestamp, data) = load_data_1();
    let (wait_time, next_bus) =
        get_next_bus(target_timestamp, &data).expect("Could not find any available buses!");
    println!("The next bus is {}, in {} minutes", next_bus, wait_time);
    println!("Answer: {}", next_bus * wait_time);
}

pub fn part2() {
    let data = load_data_2();
    let depart = get_sequential(&data);
    println!("Answer: {}", depart);
}
