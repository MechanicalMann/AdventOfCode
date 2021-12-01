use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 1;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<i32> {
        let data = AdventInput::for_day(DAY).get_lines::<i32>()?;
        count_increments(&data)
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<i32> {
        let data = AdventInput::for_day(DAY).get_lines::<i32>()?;
        count_window_increments(&data, 3)
    }
}

fn count_increments(numbers: &Vec<i32>) -> Result<i32> {
    let mut incr = 0;
    let mut last_element: Option<i32> = None;
    for &depth in numbers {
        if let Some(prev) = last_element {
            if prev < depth {
                incr += 1;
            }
        }
        last_element = Some(depth);
    }
    Ok(incr)
}

fn count_window_increments(numbers: &Vec<i32>, window: usize) -> Result<i32> {
    if numbers.len() < window {
        panic!("Invalid input: window larger than input vector")
    }
    let mut incr = 0;
    let mut last_sum: Option<i32> = None;
    for start in 0..(numbers.len() - (window - 1)) {
        let depth: i32 = numbers[start..(start + window)].iter().sum();
        if let Some(prev) = last_sum {
            if prev < depth {
                incr += 1;
            }
        }
        last_sum = Some(depth);
    }
    Ok(incr)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_count_increments() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = count_increments(&input).unwrap();
        assert_eq!(7, result);
    }

    #[test]
    fn should_count_window_incr() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = count_window_increments(&input, 3).unwrap();
        assert_eq!(5, result);
    }
}
