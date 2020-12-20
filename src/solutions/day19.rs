use anyhow::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

// Is this easier than using structs, the way I did yesterday?
// Let's find out
#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    Ref(Node),
    Double(Node, Node),
    Triple(Node, Node, Node),
    Or(Node, Node),
}
impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Rule::Char(c) => c.to_string(),
                Rule::Ref(r) => r.to_string(),
                Rule::Double(one, two) => format!("{} {}", one, two),
                Rule::Triple(one, two, three) => format!("{} {} {}", one, two, three),
                Rule::Or(left, right) => format!("{} | {}", left, right),
            }
        )
    }
}

type Node = Box<Rule>;
impl Validator for Node {
    fn is_valid(&self, message: &str) -> bool {
        match evaluate(self.to_owned(), &message.chars().collect(), 0) {
            None => false,
            Some(valid_chars) => valid_chars == message.len(),
        }
    }
}
trait Validator {
    fn is_valid(&self, message: &str) -> bool;
}

fn evaluate(r: Node, chars: &Vec<char>, start: usize) -> Option<usize> {
    if start >= chars.len() {
        return None;
    }
    let i = start;
    let mut result = None;
    // let temp = r.clone();
    match *r {
        Rule::Char(c) => {
            if c == chars[i] {
                result = Some(1);
            }
        }
        Rule::Ref(r) => result = evaluate(r, chars, i),
        Rule::Or(l, r) => {
            if let res @ Some(_) = evaluate(l, chars, i) {
                result = res;
            } else if let res @ Some(_) = evaluate(r, chars, i) {
                result = res;
            }
        }
        Rule::Double(one, two) => {
            let mut matched = 0;
            for rule in &[one, two] {
                matched += match evaluate(rule.to_owned(), chars, i + matched) {
                    Some(c) => c,
                    None => {
                        result = None;
                        break;
                    }
                };
                result = Some(matched);
            }
        }
        Rule::Triple(one, two, three) => {
            let mut matched = 0;
            for rule in &[one, two, three] {
                matched += match evaluate(rule.to_owned(), chars, i + matched) {
                    Some(c) => c,
                    None => {
                        result = None;
                        break;
                    }
                };
                result = Some(matched);
            }
        }
    }
    // println!("{:indent$}{}", "", matches!(result, Some(_)), indent = i);
    // if result.is_some() {
    //     println!(
    //         "{:indent$}Rule {} matches {} chars",
    //         "",
    //         temp,
    //         result?,
    //         indent = i
    //     );
    // }
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

fn get_rule_by_index(idx: usize, rules: &HashMap<usize, String>) -> Option<Node> {
    let text = &rules.get(&idx)?;
    return get_rule(text, rules);
}

fn get_rule(text: &str, rules: &HashMap<usize, String>) -> Option<Node> {
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
        return Some(Box::new(Rule::Char(c)));
    } else if cap.get(2).is_some() {
        let i = cap[2].parse::<usize>().unwrap();
        let node = get_rule_by_index(i, rules).expect(&format!("Invalid index in rule: {}", text));
        return Some(Box::new(Rule::Ref(node)));
    } else if cap.get(3).is_some() {
        let one = cap[3].parse::<usize>().unwrap();
        let two = cap[4].parse::<usize>().unwrap();
        let one =
            get_rule_by_index(one, rules).expect(&format!("Invalid first index in rule: {}", text));
        let two = get_rule_by_index(two, rules)
            .expect(&format!("Invalid second index in rule: {}", text));
        return Some(Box::new(Rule::Double(one, two)));
    } else if cap.get(5).is_some() {
        let one = cap[5].parse::<usize>().unwrap();
        let two = cap[6].parse::<usize>().unwrap();
        let three = cap[7].parse::<usize>().unwrap();
        let one =
            get_rule_by_index(one, rules).expect(&format!("Invalid first index in rule: {}", text));
        let two = get_rule_by_index(two, rules)
            .expect(&format!("Invalid second index in rule: {}", text));
        let three = get_rule_by_index(three, rules)
            .expect(&format!("Invalid third index in rule: {}", text));
        return Some(Box::new(Rule::Triple(one, two, three)));
    } else if cap.get(8).is_some() {
        let left = get_rule(&cap[8], rules)
            .expect(&format!("Invalid left-side expression in rule: {}", text));
        let right = get_rule(&cap[9], rules)
            .expect(&format!("Invalid right-side expression in rule: {}", text));
        return Some(Box::new(Rule::Or(left, right)));
    }
    None
}

pub fn part1() {
    let (rules, messages) = load_data();
    let root = get_rule_by_index(0, &rules).expect("No rule found!");
    let valid = messages.iter().filter(|m| root.is_valid(m)).count();
    println!("Valid messages: {}", valid);
}
pub fn part2() {}
