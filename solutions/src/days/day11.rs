use crate::solver::Solver;
use anyhow::Result;
use std::str::FromStr;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 11;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut grid = self.input().get_as::<Grid>()?;
        Ok(grid.run_for(100))
    }

    fn part_two(&self) -> Result<usize> {
        let mut grid = self.input().get_as::<Grid>()?;
        Ok(grid.get_sync())
    }
}

struct Grid {
    octopi: Vec<Vec<u8>>,
    width: isize,
    height: isize,
}
impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octopi = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let height = octopi.len() as isize;
        let width = octopi[0].len() as isize;
        Ok(Grid {
            octopi,
            width,
            height,
        })
    }
}
impl Grid {
    fn get_octopus<'a>(&'a mut self, x: isize, y: isize) -> Option<&'a mut u8> {
        let in_grid = x >= 0 && x < self.width && y >= 0 && y < self.height;
        in_grid.then(|| &mut self.octopi[y as usize][x as usize])
    }

    fn charge(&mut self, x: isize, y: isize) -> Option<((isize, isize), u8)> {
        self.get_octopus(x, y).and_then(|val| {
            *val += 1;
            Some(((x, y), *val))
        })
    }

    fn flash(&mut self, x: isize, y: isize) -> usize {
        let mut flashes = 1;
        let adjacent = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];
        let mut triggered: Vec<(isize, isize)> = vec![];
        for (x, y) in adjacent {
            if let Some(o) = self.get_octopus(x, y) {
                if *o == 0 {
                    continue; // already flashed
                }
                *o += 1;
                if *o == 10 {
                    triggered.push((x, y));
                }
            }
        }
        self.octopi[y as usize][x as usize] = 0;
        for (x, y) in triggered {
            flashes += self.flash(x, y);
        }
        flashes
    }

    fn step(&mut self) -> usize {
        let mut instigators: Vec<(isize, isize)> = vec![];
        for y in 0..self.height {
            for x in 0..self.height {
                let (pos, val) = self.charge(x, y).unwrap();
                if val > 9 {
                    instigators.push(pos);
                }
            }
        }
        instigators.iter().map(|&(x, y)| self.flash(x, y)).sum()
    }

    fn run_for(&mut self, iterations: usize) -> usize {
        let mut flashes = 0;
        for _ in 0..iterations {
            flashes += self.step();
        }
        flashes
    }

    fn get_sync(&mut self) -> usize {
        let total = (self.width * self.height) as usize;
        let mut step = 1;
        loop {
            if self.step() == total {
                break;
            }
            step += 1;
        }
        step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() {
        let input = "111
111
111";
        let grid = input.parse::<Grid>().unwrap();
        assert_eq!(3, grid.width);
        assert_eq!(3, grid.height);
        assert_eq!(
            vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            grid.octopi
        );
    }

    #[test]
    fn should_step() {
        let input = "111
111
111";
        let mut grid = input.parse::<Grid>().unwrap();
        grid.step();
        assert_eq!(
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]],
            grid.octopi
        );
    }

    #[test]
    fn should_flash() {
        let input = "111
191
111";
        let mut grid = input.parse::<Grid>().unwrap();
        grid.step();
        assert_eq!(
            vec![vec![3, 3, 3], vec![3, 0, 3], vec![3, 3, 3]],
            grid.octopi
        );
    }

    #[test]
    fn should_propagate_flash() {
        let input = "11111
19991
19191
19991
11111";
        let mut grid = input.parse::<Grid>().unwrap();
        grid.step();
        assert_eq!(
            vec![
                vec![3, 4, 5, 4, 3],
                vec![4, 0, 0, 0, 4],
                vec![5, 0, 0, 0, 5],
                vec![4, 0, 0, 0, 4],
                vec![3, 4, 5, 4, 3]
            ],
            grid.octopi
        );

        grid.step();
        assert_eq!(
            vec![
                vec![4, 5, 6, 5, 4],
                vec![5, 1, 1, 1, 5],
                vec![6, 1, 1, 1, 6],
                vec![5, 1, 1, 1, 5],
                vec![4, 5, 6, 5, 4]
            ],
            grid.octopi
        );
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut grid = input.parse::<Grid>().unwrap();
        let flashes = grid.run_for(100);
        assert_eq!(1656, flashes);
    }

    #[test]
    fn should_get_sync() {
        let input = "999
999
999";
        let mut grid = input.parse::<Grid>().unwrap();
        let step = grid.get_sync();
        assert_eq!(1, step);
    }

    #[test]
    fn should_solve_part2_example() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut grid = input.parse::<Grid>().unwrap();
        let step = grid.get_sync();
        assert_eq!(195, step);
    }
}
