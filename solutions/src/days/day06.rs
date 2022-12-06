use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 6;
    const TITLE: &'static str = "Tuning Trouble";

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let input = self.input().get()?;
        Ok(find_packet(&input, 4).unwrap())
    }

    fn part_two(&self) -> Result<usize> {
        let input = self.input().get()?;
        Ok(find_packet(&input, 14).unwrap())
    }
}

fn find_packet(input: &str, size: usize) -> Option<usize> {
    let (mut start, mut offset) = (0, size);
    let (mut same, mut unique) = (0, 1);
    while offset <= input.len() {
        for (i, a) in input[start..(offset - 1)].chars().enumerate() {
            for b in input[(start + i + 1)..offset].chars() {
                if a == b {
                    same += 1;
                    break;
                }
            }
            if same == 0 {
                unique += 1;
            } else {
                break;
            }
        }
        if unique == size {
            return Some(offset);
        }
        start += 1;
        offset += 1;
        unique = 1;
        same = 0;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn should_find_unique_sequence() {
        let input = "aaaaabcd";
        let actual = find_packet(input, 4).unwrap();
        assert_eq!(8, actual);
    }

    #[test]
    fn should_solve_part_1() {
        let actual = find_packet(EXAMPLE_INPUT, 4).unwrap();
        assert_eq!(7, actual);
    }

    #[test]
    fn should_solve_part_2() {
        let actual = find_packet(EXAMPLE_INPUT, 14).unwrap();
        assert_eq!(19, actual);
    }
}
