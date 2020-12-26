use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Not};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Black,
    White,
}
impl Not for Tile {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    q: i32,
    r: i32,
}
impl Point {
    fn new(q: i32, r: i32) -> Self {
        Point { q, r }
    }

    fn find(&self, directions: &[Direction]) -> Point {
        let mut point = self.clone();
        for d in directions {
            point = point + *d;
        }
        point
    }
}
impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::E => Point::new(self.q + 1, self.r),
            Direction::W => Point::new(self.q - 1, self.r),
            Direction::NE => Point::new(self.q + 1, self.r - 1),
            Direction::NW => Point::new(self.q, self.r - 1),
            Direction::SE => Point::new(self.q, self.r + 1),
            Direction::SW => Point::new(self.q - 1, self.r + 1),
        }
    }
}

struct Floor {
    tiles: HashMap<Point, Tile>,
}
impl Floor {
    fn new() -> Self {
        Floor {
            tiles: HashMap::new(),
        }
    }

    fn get_tile(&self, point: &Point) -> Tile {
        if let Some(tile) = self.tiles.get(point) {
            *tile
        } else {
            Tile::White
        }
    }

    fn flip_tile(&mut self, point: Point) {
        let tile = self.get_tile(&point);
        self.tiles.insert(point, !tile);
    }

    fn count_tiles(&self, color: Tile) -> usize {
        self.tiles.values().filter(|&t| t == &color).count()
    }
}

fn parse(s: &str) -> Vec<Direction> {
    let mut directions = vec![];
    let mut chars = s.chars();
    let mut buf = '\0';
    while let Some(c) = chars.next() {
        directions.push(match c {
            'n' => {
                buf = c;
                continue;
            }
            's' => {
                buf = c;
                continue;
            }
            'e' => match buf {
                'n' => Direction::NE,
                's' => Direction::SE,
                '\0' => Direction::E,
                _ => panic!("Invalid character buffer!"),
            },
            'w' => match buf {
                'n' => Direction::NW,
                's' => Direction::SW,
                '\0' => Direction::W,
                _ => panic!("Invalid character buffer!"),
            },
            _ => panic!("Invalid direction!"),
        });
        buf = '\0';
    }
    directions
}

fn load_data() -> Vec<Vec<Direction>> {
    let datafile = "data/day24.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    read.lines().map(|l| parse(l)).collect()
}

fn get_points(instructions: &Vec<Vec<Direction>>) -> Vec<Point> {
    let origin = Point::new(0, 0);
    instructions.iter().map(|i| origin.find(i)).collect()
}

pub fn part1() {
    let data = load_data();
    let points = get_points(&data);
    let mut floor = Floor::new();
    for p in points {
        floor.flip_tile(p);
    }
    println!("Black tiles: {}", floor.count_tiles(Tile::Black));
}
pub fn part2() {}
