mod days;
mod input;

fn main() {
    println!("Day {:02} Part {:02}: {}",  1, 1, days::day01::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  1, 2, days::day01::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  2, 1, days::day02::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  2, 2, days::day02::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  3, 1, days::day03::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  3, 2, days::day03::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  4, 1, days::day04::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  4, 2, days::day04::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  5, 1, days::day05::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  5, 2, days::day05::part2::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  6, 1, days::day06::part1::solve().unwrap());
    println!("Day {:02} Part {:02}: {}",  6, 2, days::day06::part2::solve().unwrap());
}
