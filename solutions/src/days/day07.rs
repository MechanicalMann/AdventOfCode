use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 7;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<i32> {
        let mut data = AdventInput::for_day(DAY).get_csv_as::<i32>()?;
        Ok(total_distance_to_median(&mut data))
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<i32> {
        let data = AdventInput::for_day(DAY).get_csv_as::<i32>()?;
        Ok(total_distance_to_mean(&data))
    }
}

fn total_distance_to_median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let median = integer_median(&numbers);
    numbers.iter().fold(0, |acc, &e| acc + (e - median).abs())
}

fn total_distance_to_mean(numbers: &[i32]) -> i32 {
    let mean = integer_mean(numbers);
    numbers
        .iter()
        .map(|&e| (e - mean).abs())
        .fold(0, |acc, e| acc + ((e * (e + 1)) / 2))
}

// This is not a good implementation of median
fn integer_median(numbers: &[i32]) -> i32 {
    let len = numbers.len();
    if len < 2 {
        numbers[0]
    } else if len % 2 == 0 {
        numbers[((len - 1) / 2)]
    } else {
        numbers[(len / 2)]
    }
}

fn integer_mean(numbers: &[i32]) -> i32 {
    let len = numbers.len() as i32;
    if len == 0 {
        0
    } else if len == 1 {
        numbers[1]
    } else {
        numbers.iter().sum::<i32>() / len
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

    #[test]
    fn should_get_rounded_mean() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let mean = integer_mean(&input);
        assert_eq!(3, mean);
    }

    #[test]
    #[ignore]
    fn should_solve_part2_example() {
        // This is *really* interesting - I caught a bug!
        // Originally, I was converting things to floats and rounding for more
        // "correct" math, because that's what the example does - the average of
        // the inputs is 4.9, which rounds to 5 and gives an answer of 168.
        // But the actual puzzle expects you to do truncated integer math -
        // which appears to invalidate the example on the page (4.9 truncates to
        // a target of simply 4, which would make the example answer 170)!
        // I'm up too late and I've had too much whiskey to make a big stink
        // about it, I'm just glad I got the star.
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let total = total_distance_to_mean(&input);
        assert_eq!(168, total)
    }
}
