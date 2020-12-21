use colored::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

type Pixels = HashMap<(usize, usize), char>;

#[derive(Clone)]
struct Tile {
    id: String,
    rotation: usize,
    size_x: usize,
    size_y: usize,
    flip: bool,
    pixels: Pixels,
}
impl Tile {
    fn get_pixel(&self, row: usize, col: usize) -> char {
        self.pixels[&self.get_coordinates(row, col, self.rotation, self.flip)]
    }

    fn get_coordinates(
        &self,
        row: usize,
        col: usize,
        rotation: usize,
        flip: bool,
    ) -> (usize, usize) {
        let (max_row, max_col) = (self.size_y - 1, self.size_x - 1);
        let mut coord = match rotation {
            0 => (row, col),
            1 => (col, max_row - row),
            2 => (max_row - row, max_col - col),
            3 => (max_col - col, row),
            _ => panic!(format!("Invalid rotation value: {}", self.rotation)),
        };
        // Because tiles can be rotated, we only need to flip in one dimension
        // (flipping then rotating 180 is equivalent)
        if flip {
            coord.1 = max_col - coord.1;
        }
        coord
    }

    fn flip(&mut self) {
        self.flip = !self.flip;
    }

    fn rotate(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for row in 0..10 {
            self.print_row(row);
            println!();
        }
    }

    fn print_row(&self, row: usize) {
        for col in 0..10 {
            print!("{}", self.get_pixel(row, col));
        }
    }

    fn print_image_row(&self, row: usize) {
        for col in 1..9 {
            print!("{}", self.get_pixel(row + 1, col));
        }
    }

    fn get_permutations(&self) -> Vec<(String, usize, bool, Vec<char>)> {
        let mut perms = vec![];
        for flip in &[false, true] {
            for rot in 0..4 {
                let mut border: Vec<char> = vec![];
                for i in 0..self.size_x {
                    border.push(self.pixels[&self.get_coordinates(0, i, rot, *flip)]);
                }
                perms.push((self.id.clone(), rot, *flip, border));
            }
        }
        perms
    }

    fn get_borders(&self) -> Vec<(String, usize, bool, Vec<char>)> {
        let mut borders = vec![];
        for ro in 0..4 {
            let rot = (self.rotation + ro) % 4;
            let mut border: Vec<char> = vec![];
            for i in 0..self.size_x {
                border.push(self.pixels[&self.get_coordinates(0, i, rot, self.flip)]);
            }
            borders.push((self.id.clone(), rot, self.flip, border));
        }
        borders
    }

    fn measure_choppiness(&self) -> usize {
        let mut choppiness = 0;
        for ((x, y), pixel) in &self.pixels {
            // ignore the borders
            let (x, y, pixel) = (*x, *y, *pixel);
            if x == 0 || x == self.size_x - 1 || y == 0 || y == self.size_y - 1 {
                continue;
            }
            if pixel == '#' {
                choppiness += 1;
            }
        }
        choppiness
    }
}

struct Map {
    tiles: HashMap<(isize, isize), Tile>,
    monsters: Vec<HashSet<(usize, usize)>>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
    pixels_x: usize,
    pixels_y: usize,
}
impl Map {
    fn add(&mut self, x: isize, y: isize, tile: Tile) {
        if x < self.min_x {
            self.min_x = x;
        } else if x > self.max_x {
            self.max_x = x;
        }
        if y < self.min_y {
            self.min_y = y;
        } else if y > self.max_y {
            self.max_y = y;
        }
        self.tiles.insert((x, y), tile);
        self.pixels_x = ((self.max_x + 1 - self.min_x).abs() * 8) as usize;
        self.pixels_y = ((self.max_y + 1 - self.min_y).abs() * 8) as usize;
    }

    fn checksum(&self) -> usize {
        let (tlc, trc, blc, brc) = (
            self.tiles.get(&(self.min_x, self.min_y)),
            self.tiles.get(&(self.max_x, self.min_y)),
            self.tiles.get(&(self.min_x, self.max_y)),
            self.tiles.get(&(self.max_x, self.max_y)),
        );
        match (tlc, trc, blc, brc) {
            (Some(a), Some(b), Some(c), Some(d)) => {
                let a = a.id.parse::<usize>().unwrap();
                let b = b.id.parse::<usize>().unwrap();
                let c = c.id.parse::<usize>().unwrap();
                let d = d.id.parse::<usize>().unwrap();
                a * b * c * d
            }
            _ => 0,
        }
    }

    fn print(&self) {
        println!(
            "Map of tiles from {}:{} to {}:{}",
            self.min_x, self.min_y, self.max_x, self.max_y
        );
        for y in self.min_y..self.max_y + 1 {
            println!();
            for row in 0..10 {
                if row == 0 {
                    for x in self.min_x..self.max_x + 1 {
                        if self.tiles.contains_key(&(x, y)) {
                            print!("Tile {}  ", self.tiles[&(x, y)].id);
                        }
                    }
                    println!();
                }
                for x in self.min_x..self.max_x + 1 {
                    let tile = (x, y);
                    if self.tiles.contains_key(&tile) {
                        self.tiles[&tile].print_row(row);
                        print!(" ");
                    } else {
                        print!("           ");
                    }
                }
                println!();
            }
        }
        println!("Checksum: {}", self.checksum())
    }

    fn print_image(&self) {
        for y in 0..self.pixels_y {
            for x in 0..self.pixels_x {
                let pixel = self.get_pixel(x, y).or(Some(' ')).unwrap();
                match pixel {
                    'O' => print!("{}", "X".black().on_green()),
                    '#' => print!("{}", "#".white().on_blue()),
                    _ => print!("{}", String::from(pixel).on_blue().dimmed()),
                }
            }
            println!();
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<char> {
        if self.monsters.iter().any(|m| m.contains(&(x, y))) {
            return Some('O');
        }
        let tile_x = (x / 8) as isize + self.min_x;
        let tile_y = (y / 8) as isize + self.min_y;
        let x = x % 8 + 1;
        let y = y % 8 + 1;
        if tile_x > self.max_x || tile_y > self.max_y {
            return None;
        }
        Some(self.tiles[&(tile_x, tile_y)].get_pixel(y, x))
    }

    fn measure_choppiness(&self) -> usize {
        let choppiness: usize = self.tiles.values().map(|t| t.measure_choppiness()).sum();
        choppiness - (self.monsters.len() * 15)
    }
}

fn get_monster(x: usize, y: usize) -> [(usize, usize); 15] {
    [
        (x, y),
        (x + 1, y + 1),
        (x + 4, y + 1),
        (x + 5, y),
        (x + 6, y),
        (x + 7, y + 1),
        (x + 10, y + 1),
        (x + 11, y),
        (x + 12, y),
        (x + 13, y + 1),
        (x + 16, y + 1),
        (x + 17, y),
        (x + 18, y),
        (x + 18, y - 1),
        (x + 19, y),
    ]
}

fn monster_scan(map: &Map) -> Vec<HashSet<(usize, usize)>> {
    let mut monsters = vec![];
    for y in 1..map.pixels_y {
        for x in 0..map.pixels_x - 20 {
            let tail = match map.get_pixel(x, y) {
                None => continue,
                Some(c) => c,
            };
            if tail != '#' {
                continue;
            }
            let monster = &get_monster(x, y);
            let found = monster.iter().map(|&(x, y)| map.get_pixel(x, y));
            let matching = found.filter(|p| p.is_some() && p.unwrap() == '#');
            if matching.count() == 15 {
                monsters.push(HashSet::from_iter(monster.iter().cloned()));
            }
        }
    }
    monsters
}

// Lord, forgive me, for I have sinned...
fn build_map(tiles: &mut Vec<Tile>) -> Vec<Map> {
    let mut maps = vec![];
    let mut seed_state = 0;
    // let mut seed_state = 4;
    loop {
        let mut map = Map {
            tiles: HashMap::new(),
            monsters: vec![],
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            pixels_x: 0,
            pixels_y: 0,
        };
        if seed_state > 8 {
            break;
        }
        let mut unplaced = tiles.clone();

        let mut seed = unplaced.remove(0);
        seed.rotation = seed_state % 4;
        seed.flip = seed_state > 3;
        map.add(0, 0, seed);

        while unplaced.len() > 0 {
            let mut changed = false;
            for i in 0..unplaced.len() {
                let permutations = unplaced[i].get_permutations();
                let mut tile_placed = false;
                for ((x, y), placed) in &map.tiles.clone() {
                    let borders = placed.get_borders();
                    // clockwise: 0 = 12, 1 = 3, 2 = 6, 3 = 9
                    for (pi, spot) in [(*x, y - 1), (x + 1, *y), (*x, y + 1), (x - 1, *y)]
                        .iter()
                        .enumerate()
                    {
                        if map.tiles.contains_key(spot) {
                            continue;
                        }
                        let border = &borders[pi];
                        for p in &permutations {
                            // println!(
                            //     "Border: {}, perm: {}",
                            //     border.3.iter().collect::<String>(),
                            //     p.3.iter().rev().collect::<String>()
                            // );
                            if border
                                .3
                                .iter()
                                .zip(p.3.iter().rev())
                                .filter(|&(l, r)| l == r)
                                .count()
                                == placed.size_x
                            {
                                let mut tile = unplaced.remove(i);
                                // println!(
                                //     "Placed tile {} at {:?} (rotation {}, flip {})",
                                //     tile.id, spot, p.1, p.2
                                // );
                                tile.rotation = ((6 - pi) + p.1) % 4;
                                tile.flip = p.2;
                                map.add(spot.0, spot.1, tile);
                                tile_placed = true;
                                changed = true;
                                break;
                            }
                        }
                        if tile_placed {
                            break;
                        }
                    }
                    if tile_placed {
                        break;
                    }
                }
                if tile_placed {
                    break;
                }
            }
            if !changed {
                break;
            }
        }

        if unplaced.len() == 0 {
            println!("Built a map!");
            maps.push(map);
            // break;
        }

        seed_state += 1;
    }
    maps
}

fn load_data() -> Vec<Tile> {
    let datafile = "data/day20.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let sections = read.split("\n\n");

    let mut tiles = vec![];
    for sec in sections {
        let mut lines = sec.lines();
        let id = lines.next().unwrap()[5..9].to_string();

        let mut pixels: Pixels = HashMap::new();
        for (row, l) in lines.enumerate() {
            for (col, c) in l.chars().enumerate() {
                pixels.insert((row, col), c);
            }
        }
        tiles.push(Tile {
            id,
            pixels,
            rotation: 0,
            flip: false,
            size_x: 10,
            size_y: 10,
        });
    }
    tiles
}

pub fn part1() {
    let mut data = load_data();
    let maps = build_map(&mut data);
    for map in maps {
        map.print();
        println!();
        println!();
    }
}

pub fn part2() {
    let mut data = load_data();
    let mut maps = build_map(&mut data);
    for (i, map) in maps.iter_mut().enumerate() {
        println!("Scanning permutation {} for monsters...", i + 1);
        map.monsters = monster_scan(&map);
        let num_monsters = map.monsters.len();
        println!("Found {} monsters!", num_monsters);
        if num_monsters != 0 {
            map.print_image();
            println!();
            println!("Roughness of water: {}", map.measure_choppiness());
            break;
        }
    }
}
