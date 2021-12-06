use crate::input::AdventInput;
use anyhow::Result;
use std::collections::HashMap;

const DAY: u8 = 6;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let mut data = AdventInput::for_day(DAY).get_csv_as::<u8>()?;
        game_of_fish(&mut data, 80);
        Ok(data.len())
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get_csv_as::<u8>()?;
        gonna_need_a_bigger_boat(&data, 256)
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

fn game_of_fish(fish: &mut Vec<u8>, iterations: usize) {
    for _i in 1..=iterations {
        tick(fish);
        // println!("After {:2} days: {:?}", _i, fish.len());
    }
}

fn gonna_need_a_bigger_boat(fish: &[u8], iterations: usize) -> Result<usize> {
    let mut tracker: HashMap<u8, usize> = HashMap::from_iter((0u8..=8u8).map(|v| (v, 0)));
    for &f in fish {
        let count = tracker.get_mut(&f).unwrap();
        *count += 1;
    }
    for _i in 1..=iterations {
        let spawn = tracker[&0];
        for j in 1..=8 {
            tracker.insert(j - 1, tracker[&j]);
        }
        tracker.insert(6, tracker[&6] + spawn);
        tracker.insert(8, spawn);
        // println!("After {:3} days: {}", _i, tracker.values().sum::<usize>());
    }
    Ok(tracker.values().sum())
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
    fn should_solve_example() {
        let mut fish = vec![3, 4, 3, 1, 2];
        game_of_fish(&mut fish, 18);
        let expected = vec![
            6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
        ];
        for (&e, a) in expected.iter().zip(fish) {
            assert_eq!(e, a);
        }
    }

    #[test]
    fn should_solve_part2_example() -> Result<()> {
        let fish = vec![3, 4, 3, 1, 2];
        let count = gonna_need_a_bigger_boat(&fish, 256)?;
        assert_eq!(26984457539, count);
        Ok(())
    }
}
