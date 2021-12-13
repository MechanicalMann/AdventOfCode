use std::{collections::HashMap, str::FromStr};

use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 8;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let data = self.input().get_lines_as::<Display>()?;
        Ok(count_unique_digits(&data))
    }

    fn part_two(&self) -> Result<usize> {
        let data = self.input().get_lines_as::<Display>()?;
        get_total_output(&data)
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

fn get_total_output(displays: &[Display]) -> Result<usize> {
    Ok(displays.iter().map(|d| decode(d).unwrap()).sum())
}

const SEGMENTS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn decode(display: &Display) -> Result<usize> {
    let mut output = 0;
    let mut map: HashMap<char, char> = HashMap::new();

    // Step one: count char occurences
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in display.signals.iter().flat_map(|s| s.chars()) {
        *counts.entry(c).or_insert(0) += 1;
    }

    // Step two: get two digits that can eliminate segments with the same counts
    let one = display
        .signals
        .iter()
        .find(|s| s.chars().count() == 2)
        .expect("No digit found for 1!");
    let four = display
        .signals
        .iter()
        .find(|s| s.chars().count() == 4)
        .expect("No digit found for 4!");

    // Step three: make the easy guesses for segments with unique counts
    for (c, count) in &counts {
        let guess = match count {
            9 => Some('f'),
            6 => Some('b'),
            4 => Some('e'),
            _ => None,
        };
        if let Some(g) = guess {
            map.insert(*c, g);
        }
    }

    // Step four: make the other guesses
    for (c, count) in &counts {
        let c = *c;
        let guess = match count {
            8 => {
                if one.contains(c) {
                    Some('c')
                } else {
                    Some('a')
                }
            }
            7 => {
                if four.contains(c) {
                    Some('d')
                } else {
                    Some('g')
                }
            }
            _ => None,
        };
        if let Some(g) = guess {
            map.insert(c, g);
        }
    }

    // We should now have all seven guesses
    assert_eq!(7, map.len());

    // And with that we can decode the output digits
    let digits: HashMap<&str, usize> = HashMap::from_iter(DIGITS.iter().map(|&s| s).zip(0..));
    for (i, o) in display.outputs.iter().enumerate() {
        let mut chars = o.chars().map(|c| map[&c]).collect::<Vec<_>>();
        chars.sort();
        let sevseg: &str = &chars.iter().collect::<String>()[..];
        let digit = digits[&sevseg];
        output += digit * 10usize.pow(3u32 - i as u32);
    }
    Ok(output)
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

    #[test]
    fn should_decode() -> Result<()> {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let display = input.parse::<Display>()?;
        let output = decode(&display)?;
        assert_eq!(5353, output);
        Ok(())
    }

    #[test]
    fn should_solve_part2_example() {
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
        let output = get_total_output(&displays).unwrap();
        assert_eq!(61229, output);
    }
}
