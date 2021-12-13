use crate::solver::Solver;
use anyhow::Result;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 10;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> Result<usize> {
        let data = self.input().get()?;
        Ok(total_syntax_score(&data))
    }

    fn part_two(&self) -> Result<usize> {
        let data = self.input().get()?;
        Ok(total_completion_score(&data))
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

fn score_syntax_err(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_syntax_score(line: &str) -> usize {
    match balance(line) {
        Ok(_) => 0,
        Err(e) => score_syntax_err(e.c),
    }
}

fn total_syntax_score(input: &str) -> usize {
    input.lines().map(|l| get_syntax_score(l)).sum()
}

fn get_completion_score(line: &str) -> usize {
    let mut score = 0;
    let mut orphans = match balance(line) {
        Ok(o) => o,
        Err(_) => return score,
    };
    while let Some(o) = orphans.pop() {
        score *= 5;
        score += match o {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => 0,
        };
    }
    score
}

fn total_completion_score(input: &str) -> usize {
    let mut scores = input
        .lines()
        .map(|l| get_completion_score(l))
        .filter(|&s| s > 0)
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
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
        let score = get_syntax_score("(]");
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
        let score = total_syntax_score(&input);
        assert_eq!(26397, score);
    }

    #[test]
    fn should_score_incomplete() {
        let mut score = get_completion_score("(");
        assert_eq!(1, score);

        score = get_completion_score("([");
        assert_eq!(7, score);

        score = get_completion_score("([{");
        assert_eq!(38, score);

        score = get_completion_score("([{<");
        assert_eq!(194, score);
    }

    #[test]
    fn should_solve_part2_example() {
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
        let score = total_completion_score(&input);
        assert_eq!(288957, score);
    }
}
