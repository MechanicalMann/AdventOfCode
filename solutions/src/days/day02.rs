use crate::input::AdventInput;
use anyhow::Result;
use std::str::FromStr;
use std::string::ParseError;

const DAY: u8 = 2;

pub mod part1 {
    use super::*;
    pub fn solve() -> Result<isize> {
        let data = AdventInput::for_day(DAY).get_lines_as::<Command>()?;
        let final_pos = navigate(&data)?;
        Ok(final_pos.x * final_pos.y)
    }
}

pub mod part2 {
    use super::*;
    pub fn solve() -> Result<isize> {
        let data = AdventInput::for_day(DAY).get_lines_as::<Command>()?;
        let final_pos = navigate_with_aim(&data)?;
        Ok(final_pos.x * final_pos.y)
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}
impl FromStr for Direction {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: isize,
}
impl FromStr for Command {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let split: Vec<_> = input.split(" ").collect();
        if split.len() != 2 {
            panic!("Invalid navigation instruction")
        }
        let direction = Direction::from_str(split[0])?;
        let distance = split[1].parse::<isize>().unwrap();
        Ok(Command {
            direction,
            distance,
        })
    }
}

struct Coordinate {
    x: isize,
    y: isize,
}

fn navigate(instructions: &Vec<Command>) -> Result<Coordinate> {
    navigate_from(&Coordinate { x: 0, y: 0 }, instructions)
}

fn navigate_from(start: &Coordinate, instructions: &Vec<Command>) -> Result<Coordinate> {
    let mut x = start.x;
    let mut y = start.y;
    for instr in instructions {
        match instr.direction {
            Direction::Forward => x += instr.distance,
            Direction::Up => y -= instr.distance,
            Direction::Down => y += instr.distance,
        }
    }
    Ok(Coordinate { x, y })
}

fn navigate_with_aim(instructions: &Vec<Command>) -> Result<Coordinate> {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut pitch: isize = 0;
    for instr in instructions {
        match instr.direction {
            Direction::Forward => {
                x += instr.distance;
                y += instr.distance * pitch;
            }
            Direction::Up => pitch -= instr.distance,
            Direction::Down => pitch += instr.distance,
        }
    }
    Ok(Coordinate { x, y })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_move_forward() {
        let instr = vec![Command::from_str("forward 1").unwrap()];
        let val = navigate(&instr).unwrap();
        assert_eq!(val.x, 1);
        assert_eq!(val.y, 0);
    }
    #[test]
    fn should_move_up() {
        let instr = vec![Command::from_str("up 1").unwrap()];
        let val = navigate(&instr).unwrap();
        assert_eq!(val.x, 0);
        assert_eq!(val.y, -1);
    }
    #[test]
    fn should_move_down() {
        let instr = vec![Command::from_str("down 1").unwrap()];
        let val = navigate(&instr).unwrap();
        assert_eq!(val.x, 0);
        assert_eq!(val.y, 1);
    }
    #[test]
    fn should_solve_example() {
        let instr: Vec<_> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|&s| Command::from_str(s).unwrap())
        .collect();
        let val = navigate(&instr).unwrap();
        assert_eq!(val.x, 15);
        assert_eq!(val.y, 10);
    }

    #[test]
    fn should_nav_with_aim() {
        let instr: Vec<_> = vec!["up 2", "forward 2"]
            .iter()
            .map(|&s| Command::from_str(s).unwrap())
            .collect();
        let val = navigate_with_aim(&instr).unwrap();
        assert_eq!(val.x, 2);
        assert_eq!(val.y, -4);
    }

    #[test]
    fn should_solve_part_2_example() {
        let instr: Vec<_> = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|&s| Command::from_str(s).unwrap())
        .collect();
        let val = navigate_with_aim(&instr).unwrap();
        assert_eq!(val.x, 15);
        assert_eq!(val.y, 60);
    }
}
