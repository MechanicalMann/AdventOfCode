use crate::input::AdventInput;
use anyhow::{bail, Result};

const DAY: u8 = 10;

pub mod part1 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get()?;
        Ok(total_score(&data))
    }
}

pub mod part2 {
    use super::*;

    pub fn solve() -> Result<usize> {
        let data = AdventInput::for_day(DAY).get()?;
        Ok(0)
    }
}

#[derive(Debug)]
struct Unbalanced {
    c: char,
}
impl std::fmt::Display for Unbalanced {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}
impl std::error::Error for Unbalanced {}

fn balance(line: &str) -> Result<Vec<char>, Unbalanced> {
    let mut tokens: Vec<char> = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => tokens.push(c),
            ')' | ']' | '}' | '>' => {
                let t = match tokens.pop() {
                    Some(x) => x,
                    None => return Err(Unbalanced { c }),
                };
                match (t, c) {
                    ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => (),
                    _ => return Err(Unbalanced { c }),
                }
            }
            _ => return Err(Unbalanced { c }),
        }
    }
    Ok(tokens)
}

fn score_char(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_score(line: &str) -> usize {
    match balance(line) {
        Ok(_) => 0,
        Err(e) => score_char(e.c),
    }
}

fn total_score(input: &str) -> usize {
    input.lines().map(|l| get_score(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_balance() -> Result<()> {
        balance("()")?;
        balance("{()()()}")?;
        balance("<([{}])>")?;
        balance("[<>({}){}[([])<>]]")?;
        balance("(((((((((())))))))))")?;
        Ok(())
    }

    #[test]
    fn should_ignore_incomplete() -> Result<()> {
        balance("((((")?;
        Ok(())
    }

    #[test]
    fn should_score() {
        let score = get_score("(]");
        assert_eq!(57, score);
    }

    #[test]
    fn should_solve_part1_example() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let score = total_score(&input);
        assert_eq!(26397, score);
    }
}
