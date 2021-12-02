mod days;
mod input;

fn main() {
    println!("Day {:02} Part {:02}: {}",  1, 1, days::day01::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  1, 2, days::day01::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  2, 1, days::day02::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  2, 2, days::day02::part2::solve().unwrap());
}
