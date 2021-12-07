use core::num;

use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 7;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<isize> {
        let mut data = AdventInput::for_day(DAY).get_csv_as::<isize>()?;
        Ok(total_distance_to_median(&mut data))
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<isize> {
        let data = AdventInput::for_day(DAY).get_csv_as::<isize>()?;
        Ok(0)
    }
}

fn total_distance_to_median(numbers: &mut [isize]) -> isize {
    numbers.sort();
    let median = integer_median(&numbers);
    numbers.iter().fold(0, |acc, &e| acc + (e - median).abs())
}

// This is not a good implementation of arithmetic median
fn integer_median(numbers: &[isize]) -> isize {
// This is not a good implementation of median
    let len = numbers.len();
    if len < 2 {
        numbers[0]
    } else if len % 2 == 0 {
        numbers[((len - 1) / 2)]
    } else {
        numbers[(len / 2)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_odd_median() {
        let med = integer_median(&vec![1, 2, 3, 6, 7, 8, 9]);
        assert_eq!(6, med)
    }

    #[test]
    fn should_get_even_median() {
        let med = integer_median(&vec![1, 2, 3, 4, 6, 7, 8, 9]);
        assert_eq!(4, med)
    }

    #[test]
    fn should_solve_part1_example() {
        let mut input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let total = total_distance_to_median(&mut input);
        assert_eq!(37, total)
    }
}
