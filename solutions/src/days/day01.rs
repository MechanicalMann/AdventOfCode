use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 1;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<i32> {
        let input = AdventInput::for_day(DAY);
        let data = input.get_lines::<i32>()?;
        count_increments(&data)
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_count_increments() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let result = count_increments(&input).unwrap();
        assert_eq!(7, result);
    }
}
