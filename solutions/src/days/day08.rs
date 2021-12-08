use std::{collections::HashMap, str::FromStr};

use crate::input::AdventInput;
use anyhow::Result;

const DAY: u8 = 8;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get_lines_as::<Display>()?;
        Ok(count_unique_digits(&data))
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<u16> {
        let data = AdventInput::for_day(DAY).get_lines_as::<Display>()?;
        Ok(0)
    }
}

// Too lazy to figure out lifetimes
struct Display {
    signals: [String; 10],
    outputs: [String; 4],
}
impl FromStr for Display {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<_> = s.split(' ').map(|str| str.to_owned()).collect();
        let mut signals: [String; 10] = Default::default();
        let mut outputs: [String; 4] = Default::default();
        signals.clone_from_slice(&components[0..10]);
        outputs.clone_from_slice(&components[11..]);
        Ok(Display { signals, outputs })
    }
}

fn count_unique_digits(displays: &[Display]) -> usize {
    let unique = vec![2, 3, 4, 7];
    displays
        .iter()
        .map(|d| {
            d.outputs
                .iter()
                .filter(|o| unique.contains(&o.chars().count()))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() -> Result<()> {
        let input = "a b c d e f g h i j | k l m n";
        let display = input.parse::<Display>()?;
        let signals = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (i, &s) in signals.iter().enumerate() {
            assert_eq!(s, &display.signals[i]);
        }
        let outputs = vec!["k", "l", "m", "n"];
        for (i, &o) in outputs.iter().enumerate() {
            assert_eq!(o, &display.outputs[i]);
        }
        Ok(())
    }

    #[test]
    fn should_get_count() {
        let input =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";
        let displays: Vec<_> = input
            .lines()
            .map(|l| l.parse::<Display>().unwrap())
            .collect();
        let count = count_unique_digits(&displays);
        assert_eq!(26, count);
    }
}
