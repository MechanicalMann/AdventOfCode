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

type Ruleset = HashMap<usize, Rule>;

fn is_valid(rule: &Rule, message: &str, ruleset: &Ruleset) -> bool {
    let chunks = evaluate(rule, &message.chars().collect(), 0, ruleset);
    return chunks.contains(&message.len());
}

fn evaluate(r: &Rule, chars: &Vec<char>, start: usize, ruleset: &Ruleset) -> Vec<usize> {
    let mut result = vec![];
    if start >= chars.len() {
        return result;
    }
    let i = start;
    match r {
        Rule::Char(c) => {
            if c == &chars[i] {
                result.push(1);
            }
        }
        Rule::Ref(rules) => {
            // Rule refs can contain more than one reference in sequence.
            // In that case, starting with no offset from the current character,
            // each rule must be evaluated for an offset of each return value
            // from its prior sibling, which represent valid paths that matched
            // a certain number of characters.  This is how we avoid early
            // termination - by considering each possible valid path, rather
            // than just the first one that matches.
            let mut offsets: Vec<Vec<usize>> = vec![vec![]; rules.len() + 1];
            offsets[0].push(0);
            for (idx, rule) in rules.iter().enumerate() {
                for o in offsets[idx].clone() {
                    let ri = evaluate(ruleset.get(rule).unwrap(), chars, i + o, ruleset);
                    offsets[idx + 1].extend(ri.iter().map(|x| x + o));
                }
            }
            // The last result contains the numbers of valid characters for the
            // whole sequence (since they must *all* match), if any.
            result = offsets[rules.len()].clone();
        }
        Rule::Or(left, right) => {
            // Another way we avoid early termination is by evaluating both
            // branches of OR rules and considering the results of each, rather
            // than just the left (or right)
            result.extend(evaluate(left, chars, i, ruleset));
            result.extend(evaluate(right, chars, i, ruleset));
        }
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
