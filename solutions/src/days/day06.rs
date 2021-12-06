use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 6;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let mut data = AdventInput::for_day(DAY).get_csv_as::<u8>()?;
        game_of_fish(&mut data, 80)?;
        Ok(data.len())
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let mut data = AdventInput::for_day(DAY).get_csv_as::<u8>()?;
        Ok(0)
    }
}

fn tick(fish: &mut Vec<u8>) {
    let mut spawn = 0;
    for f in fish.iter_mut() {
        if f == &0 {
            spawn += 1;
            *f = 6;
        } else {
            *f -= 1;
        }
    }
    fish.extend(vec![8; spawn]);
}

fn game_of_fish(fish: &mut Vec<u8>, iterations: usize) -> Result<usize> {
    for _i in 1..=iterations {
        tick(fish);
        println!("After {:2} days: {:?}", _i, fish.len());
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_update_fish() {
        let mut fish = vec![3, 2, 1];
        tick(&mut fish);
        let expected = vec![2, 1, 0];
        for (&e, a) in expected.iter().zip(fish) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn should_spawn_new_fish() {
        let mut fish = vec![2, 1, 0];
        tick(&mut fish);
        let expected = vec![1, 0, 6, 8];
        for (&e, a) in expected.iter().zip(fish) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn should_solve_example() -> Result<()> {
        let mut fish = vec![3, 4, 3, 1, 2];
        game_of_fish(&mut fish, 18)?;
        let expected = vec![
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ];
        for (&e, a) in expected.iter().zip(fish) {
            assert_eq!(e, a);
        }
        Ok(())
    }
}
