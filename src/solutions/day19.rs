use anyhow::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

// Is this easier than using structs, the way I did yesterday?
// Let's find out
#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    Ref(Vec<usize>),
    Or(Box<Rule>, Box<Rule>),
}
impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Rule::Char(c) => c.to_string(),
                Rule::Ref(r) => format!("{:?}", r),
                Rule::Or(left, right) => format!("{} | {}", left, right),
            }
        )
    }
}

type Ruleset = HashMap<usize, Rule>;

fn is_valid(rule: &Rule, message: &str, ruleset: &Ruleset) -> bool {
    println!();
    println!("Testing {}...", message);
    let chunks = evaluate(rule, &message.chars().collect(), 0, ruleset);
    return chunks.contains(&message.len());
}

fn evaluate(r: &Rule, chars: &Vec<char>, start: usize, ruleset: &Ruleset) -> Vec<usize> {
    let mut result = vec![];
    if start >= chars.len() {
        return result;
    }
    let i = start;
    let temp = r.clone();
    match r {
        Rule::Char(c) => {
            if c == &chars[i] {
                result.push(1);
            }
        }
        Rule::Ref(rules) => {
            let mut offsets: Vec<Vec<usize>> = vec![vec![]; rules.len() + 1];
            offsets[0].push(0);
            for (idx, rule) in rules.iter().enumerate() {
                for o in offsets[idx].clone() {
                    let ri = evaluate(ruleset.get(rule).unwrap(), chars, i + o, ruleset);
                    offsets[idx + 1].extend(ri.iter().map(|x| x + o));
                }
            }
            result = offsets[rules.len()].clone();
        }
        Rule::Or(left, right) => {
            result.extend(evaluate(left, chars, i, ruleset));
            result.extend(evaluate(right, chars, i, ruleset));
        }
    }
    if result.len() > 0 {
        println!(
            "{:indent$}Rule {} matches {:?} chars",
            "",
            temp,
            result,
            indent = i
        );
    } else {
        println!("{:indent$}Rule {} does not match", "", temp, indent = i);
    }
    result
}

fn load_data() -> (HashMap<usize, String>, Vec<String>) {
    let datafile = "data/day19.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    let mut lines = read.lines();

    let mut rules: HashMap<usize, String> = HashMap::new();
    let mut messages: Vec<String> = vec![];

    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let mut parts = line.split(": ");
        let idx = parts.next().unwrap().parse::<usize>().unwrap();
        rules.insert(idx, parts.next().unwrap().to_string());
    }

    for line in lines {
        messages.push(line.to_string());
    }

    (rules, messages)
}

fn build_ruleset(rules: &HashMap<usize, String>) -> Option<Ruleset> {
    let mut ruleset = HashMap::new();
    for (idx, rule) in rules {
        ruleset.insert(*idx, get_rule(rule)?);
    }
    Some(ruleset)
}

fn get_rule(text: &str) -> Option<Rule> {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(
            r#"^(?:"(.)"|(\d+)|(?:(\d+) (\d+))|(?:(\d+) (\d+) (\d+))|(?:(.+) \| (.+)))$"#
        )
        .unwrap();
    }
    let cap = RULE_RE
        .captures(text)
        .expect(&format!("Invalid rule expression: {}", text));
    if cap.get(1).is_some() {
        let c = cap[1].chars().next()?;
        return Some(Rule::Char(c));
    } else if cap.get(2).is_some() {
        let i = cap[2].parse::<usize>().unwrap();
        return Some(Rule::Ref(vec![i]));
    } else if cap.get(3).is_some() {
        let one = cap[3].parse::<usize>().unwrap();
        let two = cap[4].parse::<usize>().unwrap();
        return Some(Rule::Ref(vec![one, two]));
    } else if cap.get(5).is_some() {
        let one = cap[5].parse::<usize>().unwrap();
        let two = cap[6].parse::<usize>().unwrap();
        let three = cap[7].parse::<usize>().unwrap();
        return Some(Rule::Ref(vec![one, two, three]));
    } else if cap.get(8).is_some() {
        let left =
            get_rule(&cap[8]).expect(&format!("Invalid left-side expression in rule: {}", text));
        let right =
            get_rule(&cap[9]).expect(&format!("Invalid right-side expression in rule: {}", text));
        return Some(Rule::Or(Box::new(left), Box::new(right)));
    }
    None
}

pub fn part1() {
    let (rules, messages) = load_data();
    let ruleset = build_ruleset(&rules).unwrap();
    let root = ruleset.get(&0).unwrap();
    let valid = messages
        .iter()
        .filter(|m| is_valid(root, m, &ruleset))
        .count();
    println!("Valid messages: {}", valid);
}
pub fn part2() {
    let (mut rules, messages) = load_data();
    rules.insert(8, "42 | 42 8".to_string());
    rules.insert(11, "42 31 | 42 11 31".to_string());
    let ruleset = build_ruleset(&rules).unwrap();
    let root = ruleset.get(&0).unwrap();
    let valid = messages
        .iter()
        .filter(|m| is_valid(root, m, &ruleset))
        .count();
    println!("Valid messages: {}", valid);
}
