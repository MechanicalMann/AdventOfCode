use anyhow::*;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum SeatState {
    Floor,
    Empty,
    Taken,
}
impl SeatState {
    fn from_char(c: char) -> Result<Self> {
        match c {
            '.' => Ok(SeatState::Floor),
            'L' => Ok(SeatState::Empty),
            '#' => Ok(SeatState::Taken),
            _ => panic!("Invalid seat state descriptor: {}", c),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Seat {
    col: usize,
    row: usize,
    state: SeatState,
}
impl Seat {
    fn new(c: char, row: usize, col: usize) -> Self {
        let state: SeatState = SeatState::from_char(c).unwrap();
        Seat { col, row, state }
    }
    fn to_string(&self) -> String {
        String::from(match self.state {
            SeatState::Floor => ".",
            SeatState::Empty => "L",
            SeatState::Taken => "#",
        })
    }
}

type SeatMap = Vec<Vec<Seat>>;

fn load_data() -> SeatMap {
    let datafile = "data/day11.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut values: SeatMap = vec![];
    for (row, line) in read.lines().enumerate() {
        let seats: Vec<_> = line
            .chars()
            .enumerate()
            .map(|(col, c)| Seat::new(c, row, col))
            .collect();
        values.push(seats);
    }
    values
}

fn count_adjacent(to_row: usize, to_col: usize, state: SeatState, map: &SeatMap) -> usize {
    let max_row = map.len() - 1;
    let max_col = map[0].len() - 1;

    let (row_start, row_end) = match to_row {
        0 => (0, to_row + 1),
        _ if to_row == max_row => (to_row - 1, to_row),
        _ => (to_row - 1, to_row + 1),
    };
    let (col_start, col_end) = match to_col {
        0 => (0, to_col + 1),
        _ if to_col == max_col => (to_col - 1, to_col),
        _ => (to_col - 1, to_col + 1),
    };

    let mut count = 0;
    for r in row_start..(row_end + 1) {
        for c in col_start..(col_end + 1) {
            if r == to_row && c == to_col {
                continue;
            }
            if map[r][c].state == state {
                count += 1;
            }
        }
    }
    count
}

fn seek(
    start_row: usize,
    start_col: usize,
    delta_y: isize,
    delta_x: isize,
    target: SeatState,
    map: &SeatMap,
) -> usize {
    let max_row = (map.len() - 1) as isize;
    let max_col = (map[0].len() - 1) as isize;
    let mut row = start_row as isize;
    let mut col = start_col as isize;
    loop {
        row += delta_y;
        col += delta_x;
        if row < 0 || row > max_row || col < 0 || col > max_col {
            break;
        }
        let state = map[row as usize][col as usize].state;
        match map[row as usize][col as usize].state {
            SeatState::Floor => continue,
            _ => {
                if state == target {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
    }
    0
}

fn count_visible(to_row: usize, to_col: usize, state: SeatState, map: &SeatMap) -> usize {
    let count = seek(to_row, to_col, -1, -1, state, &map)
        + seek(to_row, to_col, -1, 0, state, &map)
        + seek(to_row, to_col, -1, 1, state, &map)
        + seek(to_row, to_col, 0, 1, state, &map)
        + seek(to_row, to_col, 1, 1, state, &map)
        + seek(to_row, to_col, 1, 0, state, &map)
        + seek(to_row, to_col, 1, -1, state, &map)
        + seek(to_row, to_col, 0, -1, state, &map);
    count
}

fn get_seat_change(seat: &Seat, map: &SeatMap) -> Option<SeatState> {
    match seat.state {
        SeatState::Empty => match count_adjacent(seat.row, seat.col, SeatState::Taken, &map) {
            0 => Some(SeatState::Taken),
            _ => None,
        },
        SeatState::Taken => match count_adjacent(seat.row, seat.col, SeatState::Taken, &map) {
            4..=8 => Some(SeatState::Empty),
            _ => None,
        },
        _ => None,
    }
}

fn get_seat_change_2(seat: &Seat, map: &SeatMap) -> Option<SeatState> {
    match seat.state {
        SeatState::Empty => match count_visible(seat.row, seat.col, SeatState::Taken, &map) {
            0 => Some(SeatState::Taken),
            _ => None,
        },
        SeatState::Taken => match count_visible(seat.row, seat.col, SeatState::Taken, &map) {
            5..=8 => Some(SeatState::Empty),
            _ => None,
        },
        _ => None,
    }
}

fn get_changes(
    map: &SeatMap,
    change_func: &dyn Fn(&Seat, &SeatMap) -> Option<SeatState>,
) -> Vec<Seat> {
    let mut changes = vec![];
    for (row, seats) in map.iter().enumerate() {
        for (col, seat) in seats.iter().enumerate() {
            match change_func(seat, &map) {
                Some(state) => changes.push(Seat { row, col, state }),
                None => (),
            }
        }
    }
    changes
}

fn apply_changes(changes: &Vec<Seat>, map: &mut SeatMap) {
    for change in changes {
        map[change.row][change.col] = Seat {
            row: change.row,
            col: change.col,
            state: change.state,
        };
    }
}

fn print_map(map: &SeatMap) {
    for row in map {
        let symbols: Vec<_> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", symbols.join(""));
    }
}

fn change_map(map: &mut SeatMap, change_func: &dyn Fn(&Seat, &SeatMap) -> Option<SeatState>) {
    let mut attempt = 0;
    loop {
        if attempt == 1000 {
            break;
        }
        let changes = &get_changes(&map, &change_func);
        if changes.len() == 0 {
            break;
        }
        apply_changes(changes, map);
        attempt += 1;
    }
}

pub fn part1() {
    let mut map = load_data();
    change_map(&mut map, &get_seat_change);
    let occupied = map.iter().flatten().filter(|x| x.state == SeatState::Taken);
    println!("Answer: {} occupied seats", occupied.count());
}

pub fn part2() {
    let mut map = load_data();
    change_map(&mut map, &get_seat_change_2);
    let occupied = map.iter().flatten().filter(|x| x.state == SeatState::Taken);
    println!("Answer: {} occupied seats", occupied.count());
}
