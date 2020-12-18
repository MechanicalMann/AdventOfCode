use std::fs;

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn load_data<'a>() -> Vec<String> {
    let datafile = "data/day18.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    read.lines().map(|x| x.to_string()).collect()
}

fn parse(s: &str) -> (isize, usize) {
    let (mut result, mut consumed, mut temp, mut op) =
        (0, 0, Option::<isize>::None, Option::<Operator>::None);
    let mut stack: Vec<isize> = vec![];
    let chars: Vec<_> = s.chars().collect();
    for i in 0..s.len() {
        if i < consumed {
            continue;
        }
        let c = chars[i];
        consumed += 1;
        match (c, temp) {
            (' ', Some(val)) => {
                stack.push(val);
                temp = None;
            }
            (' ', _) => continue,
            ('(', _) => {
                let (inner, ic) = parse(&s[i + 1..]);
                consumed += ic;
                stack.push(inner);
            }
            ('0'..='9', None) => temp = Some(c.to_digit(10).unwrap() as isize),
            ('0'..='9', Some(v)) => temp = Some(v * 10 + (c.to_digit(10).unwrap() as isize)),
            ('+', _) => op = Some(Operator::Add),
            ('-', _) => op = Some(Operator::Sub),
            ('*', _) => op = Some(Operator::Mul),
            ('/', _) => op = Some(Operator::Div),
            (')', _) => break,
            _ => panic!("Invalid character: {}", c),
        }
        if let Some(x) = eval(&op, &mut stack) {
            stack.push(x);
            op = None;
        }
    }
    if let Some(val) = temp {
        stack.push(val);
    }
    if let Some(x) = eval(&op, &mut stack) {
        result = x;
    } else if stack.len() == 1 {
        result = stack[0];
    }
    (result, consumed)
}

fn eval(op: &Option<Operator>, stack: &mut Vec<isize>) -> Option<isize> {
    if let Some(o) = op {
        if stack.len() < 2 {
            return None;
        }
        let left = stack.pop().unwrap();
        let right = stack.pop().unwrap();
        return match o {
            Operator::Add => Some(left + right),
            Operator::Sub => Some(left - right),
            Operator::Mul => Some(left * right),
            Operator::Div => Some(left / right),
        };
    }
    None
}

pub fn part1() {
    let data = load_data();
    let mut sum = 0;
    for problem in data {
        let (res, ..) = parse(&problem);
        sum += res;
    }
    println!("Answer: {}", sum);
}

pub fn part2() {}
