mod days;
mod input;

fn main() {
    println!("Day {:02} Part {:02}: {}",  1, 1, days::day01::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  1, 2, days::day01::part2::solve().unwrap());
}
