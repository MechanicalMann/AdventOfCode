use anyhow::*;
use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Default for Direction {
    fn default() -> Self {
        Direction::East
    }
}
const DIRECTIONS: [Direction; 4] = [
    Direction::East,
    Direction::South,
    Direction::West,
    Direction::North,
];

fn offset(dir: Direction) -> usize {
    match dir {
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
        Direction::North => 3,
    }
}

#[derive(Clone, Copy, Debug)]
enum Action {
    North(isize),
    East(isize),
    South(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}
impl FromStr for Action {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num: isize = s[1..].parse()?;
        let action = match s.chars().next().unwrap() {
            'N' => Action::North(num),
            'E' => Action::East(num),
            'S' => Action::South(num),
            'W' => Action::West(num),
            'L' => Action::Left(num),
            'R' => Action::Right(num),
            'F' => Action::Forward(num),
            _ => bail!("Invalid Action: {}", s),
        };
        Ok(action)
    }
}

fn get_facing(dir: Direction, val: isize) -> Action {
    match dir {
        Direction::North => Action::North(val),
        Direction::East => Action::East(val),
        Direction::South => Action::South(val),
        Direction::West => Action::West(val),
    }
}

#[derive(Default)]
struct Position {
    x: isize,
    y: isize,
    facing: Direction,
}
impl Position {
    fn apply(&mut self, action: Action) {
        match action {
            Action::North(val) => self.y += val,
            Action::South(val) => self.y -= val,
            Action::East(val) => self.x += val,
            Action::West(val) => self.x -= val,
            Action::Left(val) => {
                self.facing = DIRECTIONS
                    [((((offset(self.facing) as isize - (val / 90)) % 4) + 4) % 4) as usize]
            }
            Action::Right(val) => {
                self.facing = DIRECTIONS[((val as usize / 90) + offset(self.facing)) % 4]
            }
            Action::Forward(val) => self.apply(get_facing(self.facing, val)),
        }
    }

    fn apply_all(&mut self, actions: &[Action]) {
        for a in actions {
            self.apply(*a);
        }
    }

    fn adjust(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }
}

#[derive(Default)]
struct Waypoint {
    x: isize,
    y: isize,
}
impl Waypoint {
    fn apply(&mut self, action: Action) {
        match action {
            Action::North(val) => self.y += val,
            Action::South(val) => self.y -= val,
            Action::East(val) => self.x += val,
            Action::West(val) => self.x -= val,
            Action::Left(val) => {
                let times = val / 90;
                for _ in 0..times {
                    let (x, y) = (self.y * -1, self.x);
                    self.x = x;
                    self.y = y;
                }
            }
            Action::Right(val) => {
                let times = val / 90;
                for _ in 0..times {
                    let (x, y) = (self.y, self.x * -1);
                    self.x = x;
                    self.y = y;
                }
            }
            _ => panic!("A waypoint is not a ship!"),
        }
        // println!("Waypoint E/W: {} N/S: {}", self.x, self.y);
    }
}

#[derive(Default)]
struct Ship {
    position: Position,
    waypoint: Waypoint,
}
impl Ship {
    fn apply(&mut self, action: Action) {
        match action {
            Action::Forward(val) => {
                let Waypoint { x, y } = self.waypoint;
                self.position.adjust(x * val, y * val);
                // println!("Ship E/W: {} N/S: {}", self.position.x, self.position.y);
            }
            _ => self.waypoint.apply(action),
        }
    }

    fn apply_all(&mut self, actions: &[Action]) {
        for a in actions {
            self.apply(*a);
        }
    }
}

fn load_data() -> Vec<Action> {
    let datafile = "data/day12.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let actions: Vec<_> = read.lines().map(|x| Action::from_str(x).unwrap()).collect();
    actions
}

pub fn part1() {
    let data = load_data();
    println!("Got {} instructions", data.len());
    let mut position = Position {
        ..Default::default()
    };
    position.apply_all(&data);
    println!("Answer: {}", position.x.abs() + position.y.abs());
}

pub fn part2() {
    let data = load_data();
    let mut ship = Ship {
        waypoint: Waypoint { x: 10, y: 1 },
        ..Default::default()
    };
    ship.apply_all(&data);
    println!("Answer: {}", ship.position.x.abs() + ship.position.y.abs());
}
