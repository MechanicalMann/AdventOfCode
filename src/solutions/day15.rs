use std::collections::HashMap;
use std::fs;

fn load_data() -> Vec<usize> {
    let datafile = "data/day15.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let line = read.lines().next().unwrap();
    line.split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn play_game(starts: &Vec<usize>, turns: usize) -> usize {
    let mut history = HashMap::new();
    let mut turn = 0;
    let mut number = 0;
    let mut details = (0, 0);
    for s in starts {
        turn += 1;
        history.insert(*s, (turn, 0));
        number = *s;
    }
    while turn < turns {
        turn += 1;
        let (last, penult) = details;
        number = match penult {
            0 => 0,
            _ => last - penult,
        };
        let (last, _) = history.get(&number).or(Some(&(0, 0))).unwrap();
        details = (turn, *last);
        history.insert(number, details);
    }
    number
}

pub fn part1() {
    let result = play_game(&load_data(), 2020);
    println!("Answer: {}", result);
}

pub fn part2() {
    let result = play_game(&load_data(), 30000000);
    println!("Answer: {}", result);
}
