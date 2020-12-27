use std::fs;

fn do_transform(value: usize, subject: usize) -> usize {
    (value * subject) % 20201227
}

fn transform(subject: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value = do_transform(value, subject);
    }
    value
}

fn guess(target: usize) -> usize {
    let (mut loop_size, mut value) = (0, 1);
    while value != target {
        loop_size += 1;
        value = do_transform(value, 7);
    }
    loop_size
}

fn load_data() -> (usize, usize) {
    let datafile = "data/day25.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut lines = read.lines();
    let card_key = lines.next().unwrap().parse::<usize>().unwrap();
    let door_key = lines.next().unwrap().parse::<usize>().unwrap();
    (card_key, door_key)
}

pub fn part1() {
    let (card, door) = load_data();
    let card_loop = guess(card);
    let door_loop = guess(door);
    let key = transform(door, card_loop);
    let chk = transform(card, door_loop);
    println!(
        "Card's loop size: {}\nDoor's loop size: {}\nEncryption key: {} ({})",
        card_loop, door_loop, key, chk
    );
}
pub fn part2() {
    println!("\n\n🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️");
    println!("\n\n   🎅️ 🌟️ ❄️     Merry Christmas!   ❄️ 🌟️ ☃️");
    println!("\n\n🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️🎄️");
}
