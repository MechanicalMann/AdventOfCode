use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use crate::solver::Solver;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 4;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let mut game = self.input().get_as::<Game>()?;
        let (call, score) = game.play()?;
        Ok(call * score)
    }

    fn part_two(&self) -> Result<usize> {
        let mut game = self.input().get_as::<Game>()?;
        let (call, score) = game.play_all()?;
        Ok(call * score)
    }
}

#[derive(Clone, Copy, Debug)]
struct Square(usize, bool);

#[derive(Debug)]
struct BingoBoard {
    squares: [[Square; 5]; 5],
    numbers: HashMap<usize, (usize, usize)>,
}
impl FromStr for BingoBoard {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<BingoBoard, Self::Err> {
        let mut squares: [[Square; 5]; 5] = [[Square(0, false); 5]; 5];
        let mut numbers: HashMap<usize, (usize, usize)> = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, num) in line.split_whitespace().enumerate() {
                let number = num.parse::<usize>().unwrap();
                squares[y][x] = Square(number, false);
                numbers.insert(number, (x, y));
            }
        }
        Ok(BingoBoard { squares, numbers })
    }
}
impl BingoBoard {
    fn is_winner(&self) -> bool {
        for chk in 0..5 {
            if self.squares[chk].iter().all(|x| x.1) {
                return true;
            }
            if self.squares.iter().map(|row| row[chk]).all(|x| x.1) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> usize {
        self.squares.iter().fold(0, |acc, row| {
            acc + row
                .iter()
                .filter(|col| !col.1)
                .fold(0, |ac2, col| ac2 + col.0)
        })
    }

    fn mark(&mut self, number: usize) {
        match self.numbers.get(&number) {
            Some(&(x, y)) => self.squares[y][x].1 = true,
            None => (),
        }
    }
}

#[derive(Debug)]
struct Game {
    calls: Vec<usize>,
    boards: Vec<BingoBoard>,
}
impl FromStr for Game {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Game, Self::Err> {
        let mut groups = input.split("\n\n");
        let call_line = groups.next().unwrap();
        let calls: Vec<_> = call_line
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        let mut boards: Vec<BingoBoard> = vec![];
        loop {
            let group = match groups.next() {
                Some(g) => g,
                None => break,
            };
            boards.push(group.parse::<BingoBoard>().unwrap())
        }
        Ok(Game { calls, boards })
    }
}
impl Game {
    fn play(&mut self) -> Result<(usize, usize)> {
        for &call in &self.calls {
            for board in self.boards.iter_mut() {
                board.mark(call);
                if board.is_winner() {
                    return Ok((call, board.score()));
                }
            }
        }
        panic!("No winner!")
    }

    fn play_all(&mut self) -> Result<(usize, usize)> {
        let mut winners: HashSet<usize> = HashSet::new();
        let total_boards = self.boards.len();
        for &call in &self.calls {
            for (i, board) in self.boards.iter_mut().enumerate() {
                if winners.contains(&i) {
                    continue;
                }
                board.mark(call);
                if board.is_winner() {
                    winners.insert(i);
                    if winners.len() == total_boards {
                        return Ok((call, board.score()));
                    }
                }
            }
        }
        panic!("No winner!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_parse() {
        let input = "1,1,1,1

 1  1  1  1  1
 1  1  1  1  1
 1  1  1  1  1
 1  1  1  1  1
 1  1  1  1  1";
        let game = input.parse::<Game>().unwrap();
        assert!(game.calls.iter().all(|&num| num == 1));
        assert!(game.boards.len() == 1);
        assert!(game.boards[0]
            .squares
            .iter()
            .all(|row| row.iter().all(|col| col.0 == 1)))
    }

    #[test]
    fn should_detect_winning_row() {
        let mut squares: [[Square; 5]; 5] = [[Square(0, false); 5]; 5];
        squares[3].iter_mut().for_each(|x| x.1 = true);
        let board = BingoBoard {
            squares,
            numbers: HashMap::new(),
        };
        assert!(board.is_winner())
    }

    #[test]
    fn should_detect_winning_col() {
        let mut squares: [[Square; 5]; 5] = [[Square(0, false); 5]; 5];
        squares.iter_mut().for_each(|x| x[3].1 = true);
        let board = BingoBoard {
            squares,
            numbers: HashMap::new(),
        };
        assert!(board.is_winner())
    }

    #[test]
    fn should_not_detect_winner() {
        let squares: [[Square; 5]; 5] = [[Square(0, false); 5]; 5];
        let board = BingoBoard {
            squares,
            numbers: HashMap::new(),
        };
        assert!(!board.is_winner())
    }

    #[test]
    fn should_get_score() {
        let mut squares: [[Square; 5]; 5] = [[Square(1, false); 5]; 5];
        squares[3].iter_mut().for_each(|x| x.1 = true);
        let board = BingoBoard {
            squares,
            numbers: HashMap::new(),
        };
        assert_eq!(20, board.score())
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut game = input.parse::<Game>().unwrap();
        let (call, score) = game.play().unwrap();
        let final_score = call * score;
        assert_eq!(4512, final_score)
    }

    #[test]
    fn should_solve_part2_example() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut game = input.parse::<Game>().unwrap();
        let (call, score) = game.play_all().unwrap();
        let final_score = call * score;
        assert_eq!(1924, final_score)
    }
}
