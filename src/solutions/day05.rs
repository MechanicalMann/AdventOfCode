use anyhow::*;
use std::fs;

struct SeatDescriptor {
    row: usize,
    seat: usize,
}
impl SeatDescriptor {
    fn new(string: &str) -> Result<SeatDescriptor> {
        if string.len() != 10 {
            return Err(anyhow!("Invalid seat descriptor string"));
        }
        let row_desc = &string[0..7];
        let seat_desc = &string[7..];
        Ok(SeatDescriptor {
            row: parse_descriptor(row_desc, 'B'),
            seat: parse_descriptor(seat_desc, 'R'),
        })
    }
    fn get_id(&self) -> usize {
        let id = (self.row * 8) + self.seat;
        id
    }
}
impl std::fmt::Display for SeatDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Row {}, Seat {} (ID: {})",
            self.row,
            self.seat,
            self.get_id()
        )
    }
}

fn parse_descriptor(string: &str, high: char) -> usize {
    let mut n: usize = 0;
    let mut pow = 2usize.pow((string.len() - 1) as u32);
    for c in string.chars() {
        if c == high {
            n ^= pow;
        }
        pow /= 2;
    }
    n
}

pub fn part1() {
    let datafile = "data/day5.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let max = read
        .lines()
        .map(|x| SeatDescriptor::new(x).expect("Invalid data file").get_id())
        .max()
        .unwrap();

    println!("Max ID: {}", max);
}

pub fn part2() {
    let datafile = "data/day5.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let seats: &Vec<SeatDescriptor> = &read
        .lines()
        .map(|x| SeatDescriptor::new(x).expect("Invalid data file"))
        .collect();
    let mut ids: Vec<usize> = seats.iter().map(|x| x.get_id()).collect();
    ids.sort(); // praise jeebus

    // Due to the binary nature of the seat IDs, they should be sequential
    for i in 0..(ids.len() - 1) {
        let cur = &ids[i];
        let next = &ids[i + 1];
        let diff = next - cur;
        if diff != 1 {
            println!(
                "Is this your card... er, seat?: {} (between {} and {})",
                cur + 1,
                cur,
                next,
            );
        }
    }
}
