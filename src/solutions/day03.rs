use std::fs;

fn get_trees<T: AsRef<str>>(lines: impl Iterator<Item = T>, dist_x: usize, dist_y: usize) -> usize {
    let mut trees = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    let width = 31;

    for l in lines {
        let l = l.as_ref();
        if cur_y % dist_y != 0 {
            cur_y += 1;
            continue;
        }
        let bytes = l.as_bytes();
        if bytes[cur_x] == b'#' {
            trees += 1;
        }
        cur_x = (cur_x + dist_x) % width;
        cur_y += 1;
    }
    trees
}

pub fn part1() {
    let datafile = "data/day3.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let lines = read.lines();

    let trees = get_trees(lines, 3, 1);
    println!("Trees hit: {}", trees);
}

pub fn part2() {
    let datafile = "data/day3.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;

    for &(x, y) in slopes.iter() {
        let trees = get_trees(read.lines(), x, y);
        product *= trees;
        println!("Trees hit going right {} down {}: {}", x, y, trees);
    }

    println!("Answer: {}", product);
}
