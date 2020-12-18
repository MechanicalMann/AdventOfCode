use std::fs;

#[derive(Clone, Copy, Debug)]
enum Cube {
    Inactive,
    Active,
    //FourCornerDayNightCycle you can't prove me wrong
}

#[derive(Clone, Copy, Debug)]
struct Coordinates {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

struct CubeUpdate {
    cube: Cube,
    coordinates: Coordinates,
}

struct PocketDimension {
    cubes: Vec<Vec<Vec<Vec<Cube>>>>, // What's our vector, Victor?
    offsets: Coordinates,
}
impl PocketDimension {
    fn shift(&self, coordinates: &Coordinates) -> (usize, usize, usize, usize) {
        let w = (coordinates.w + self.offsets.w) as usize;
        let x = (coordinates.x + self.offsets.x) as usize;
        let y = (coordinates.y + self.offsets.y) as usize;
        let z = (coordinates.z + self.offsets.z) as usize;
        (w, x, y, z)
    }

    fn get_cube(&self, at: &Coordinates) -> Cube {
        let (w, x, y, z) = self.shift(at);

        if z >= self.cubes.len()
            || y >= self.cubes[z].len()
            || x >= self.cubes[z][y].len()
            || w >= self.cubes[z][y][x].len()
        {
            return Cube::Inactive;
        }

        return self.cubes[z][y][x][w];
    }

    fn get_change(&self, origin: &Coordinates) -> Option<Cube> {
        let state = self.get_cube(origin);
        let zs = [origin.z - 1, origin.z, origin.z + 1];
        let ys = [origin.y - 1, origin.y, origin.y + 1];
        let xs = [origin.x - 1, origin.x, origin.x + 1];
        let ws = [origin.w - 1, origin.w, origin.w + 1];

        let mut active = 0;
        for z in zs.iter().cloned() {
            for y in ys.iter().cloned() {
                for x in xs.iter().cloned() {
                    for w in ws.iter().cloned() {
                        if z == origin.z && y == origin.y && x == origin.x && w == origin.w {
                            continue;
                        }
                        let search = Coordinates { w, x, y, z };
                        if matches!(self.get_cube(&search), Cube::Active) {
                            active += 1;
                        }
                    }
                }
            }
        }
        match (state, active) {
            (Cube::Active, 2..=3) => None,
            (Cube::Active, _) => Some(Cube::Inactive),
            (Cube::Inactive, 3) => Some(Cube::Active),
            _ => None,
        }
    }

    fn get_changes(&self) -> Vec<CubeUpdate> {
        let mut changes: Vec<CubeUpdate> = vec![];
        let mut coordinates;
        // Each time we check, we check 1 cube further in every direction
        for z in 0..self.cubes.len() + 2 {
            let z = z as isize - 1;
            for y in 0..self.cubes[0].len() + 2 {
                let y = y as isize - 1;
                for x in 0..self.cubes[0][0].len() + 2 {
                    let x = x as isize - 1;
                    for w in 0..self.cubes[0][0][0].len() + 2 {
                        let w = w as isize - 1;
                        coordinates = Coordinates { w, x, y, z };
                        if let Some(cube) = self.get_change(&coordinates) {
                            changes.push(CubeUpdate { coordinates, cube })
                        }
                    }
                }
            }
        }
        changes
    }

    fn apply_changes(&mut self, changes: &Vec<CubeUpdate>) {
        let size_z = self.cubes.len();
        let size_y = self.cubes[0].len();
        let size_x = self.cubes[0][0].len();
        let size_w = self.cubes[0][0][0].len();
        let Coordinates {
            w: w_offset,
            x: x_offset,
            y: y_offset,
            z: z_offset,
        } = self.offsets;

        // Grow
        let (
            mut min_z,
            mut max_z,
            mut min_y,
            mut max_y,
            mut min_x,
            mut max_x,
            mut min_w,
            mut max_w,
        ) = (0, 0, 0, 0, 0, 0, 0, 0);
        for change in changes {
            if change.coordinates.z < 0 {
                min_z = change.coordinates.z;
            }
            if change.coordinates.z > max_z {
                max_z = change.coordinates.z;
            }
            if change.coordinates.y < min_y {
                min_y = change.coordinates.y;
            }
            if change.coordinates.y > max_y {
                max_y = change.coordinates.y;
            }
            if change.coordinates.x < min_x {
                min_x = change.coordinates.x;
            }
            if change.coordinates.x > max_x {
                max_x = change.coordinates.x;
            }
            if change.coordinates.w < min_w {
                min_w = change.coordinates.w;
            }
            if change.coordinates.w > max_w {
                max_w = change.coordinates.w;
            }
        }
        if min_z < 0 {
            self.cubes
                .insert(0, vec![vec![vec![Cube::Inactive; size_w]; size_x]; size_y]);
            self.offsets.z += 1;
        }
        if max_z >= (size_z as isize - z_offset) {
            self.cubes
                .push(vec![vec![vec![Cube::Inactive; size_w]; size_x]; size_y]);
        }
        if min_y < 0 {
            for i in 0..self.cubes.len() as usize {
                self.cubes[i].insert(0, vec![vec![Cube::Inactive; size_w]; size_x]);
            }
            self.offsets.y += 1;
        }
        if max_y >= (size_y as isize - y_offset) {
            for i in 0..self.cubes.len() {
                self.cubes[i].push(vec![vec![Cube::Inactive; size_w]; size_x]);
            }
        }
        if min_x < 0 {
            for i in 0..self.cubes.len() {
                for j in 0..self.cubes[i].len() {
                    self.cubes[i][j].insert(0, vec![Cube::Inactive; size_w]);
                }
            }
            self.offsets.x += 1;
        }
        if max_x >= (size_x as isize - x_offset) {
            for i in 0..self.cubes.len() {
                for j in 0..self.cubes[i].len() {
                    self.cubes[i][j].push(vec![Cube::Inactive; size_w]);
                }
            }
        }
        if min_w < 0 {
            for i in 0..self.cubes.len() {
                for j in 0..self.cubes[i].len() {
                    for k in 0..self.cubes[i][j].len() {
                        self.cubes[i][j][k].insert(0, Cube::Inactive);
                    }
                }
            }
            self.offsets.w += 1;
        }
        if max_w >= (size_w as isize - w_offset) {
            for i in 0..self.cubes.len() {
                for j in 0..self.cubes[i].len() {
                    for k in 0..self.cubes[i][j].len() {
                        self.cubes[i][j][k].push(Cube::Inactive);
                    }
                }
            }
        }

        // Update
        for change in changes {
            let (w, x, y, z) = self.shift(&change.coordinates);
            self.cubes[z][y][x][w] = change.cube;
        }
        // Reset offsets
        self.offsets = Coordinates {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        };
    }

    fn cycle(&mut self) {
        let changes = self.get_changes();
        self.apply_changes(&changes);
    }

    fn count_active(&self) -> usize {
        let mut total = 0;
        for z in 0..self.cubes.len() {
            for y in 0..self.cubes[z].len() {
                for x in 0..self.cubes[z][y].len() {
                    for w in 0..self.cubes[z][y][x].len() {
                        if matches!(self.cubes[z][y][x][w], Cube::Active) {
                            total += 1;
                        }
                    }
                }
            }
        }
        total
    }
}

fn load_data() -> PocketDimension {
    let datafile = "data/day17.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");

    let mut ys: Vec<Vec<Cube>> = vec![];
    for l in read.lines() {
        let mut cubes: Vec<Cube> = vec![];
        for c in l.chars() {
            cubes.push(match c {
                '.' => Cube::Inactive,
                '#' => Cube::Active,
                _ => panic!("Invalid input!"),
            });
        }
        ys.push(cubes);
    }
    PocketDimension {
        cubes: vec![vec![ys]],
        offsets: Coordinates {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        },
    }
}

pub fn part1() {
    // let mut pocket_dimension = load_data();
    // for cycle in 1..7 {
    //     pocket_dimension.cycle();
    //     // println!("After {} cycle(s):", cycle);
    //     // pocket_dimension.print();
    //     // println!();
    // }
    // println!("Answer: {}", pocket_dimension.count_active());
}

pub fn part2() {
    let mut pocket_dimension = load_data();
    for _ in 1..7 {
        pocket_dimension.cycle();
    }
    println!("Answer: {}", pocket_dimension.count_active());
}
