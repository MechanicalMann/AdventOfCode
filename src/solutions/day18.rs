use anyhow::*;
use std::fmt::Debug;
use std::fs;

trait Expression {
    fn eval(&self) -> isize;
}
type Node = Box<dyn Expression>;

struct Const {
    val: isize,
}
impl Expression for Const {
    fn eval(&self) -> isize {
        self.val
    }
}

struct Binary {
    operator: Operator,
    left: Node,
    right: Node,
}
impl Expression for Binary {
    fn eval(&self) -> isize {
        let (left, right) = (self.left.eval(), self.right.eval());
        eval_operator(&self.operator, left, right)
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
impl Operator {
    fn get_precendence(&self) -> usize {
        match self {
            Operator::Add => 4,
            Operator::Mul => 3,
            Operator::Div => 2,
            Operator::Sub => 1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    EOF,
    Const(isize),
    Op(Operator),
    LParen,
    RParen,
}

fn load_data<'a>() -> Vec<String> {
    let datafile = "data/day18.txt";
    let read = fs::read_to_string(datafile).expect("Failed to read data file!");
    read.lines().map(|x| x.to_string()).collect()
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut chars = s.chars().peekable();
    loop {
        let c = match chars.next() {
            Some(ch) => ch,
            None => break,
        };
        match c {
            ' ' => continue,
            '+' => tokens.push(Token::Op(Operator::Add)),
            '-' => tokens.push(Token::Op(Operator::Sub)),
            '*' => tokens.push(Token::Op(Operator::Mul)),
            '/' => tokens.push(Token::Op(Operator::Div)),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => {
                let mut num = String::from(c);
                loop {
                    match chars.peek() {
                        Some(n) if n.is_digit(10) => {
                            num.push(*n);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                match num.parse::<isize>() {
                    Ok(v) => tokens.push(Token::Const(v)),
                    Err(_) => panic!("Parser error: expected numeric value, got {}", num),
                }
            }
            _ => panic!("Invalid character: {}", c),
        }
    }
    tokens.push(Token::EOF);
    tokens
}

fn parse_expr(s: &str) -> Result<isize> {
    let tokens = tokenize(s);

    let mut operators: Vec<Token> = vec![];
    let mut operands: Vec<Node> = vec![];
    let mut prev = Token::EOF;

    for cur in tokens {
        match cur {
            Token::EOF => break,
            Token::Const(val) => operands.push(Box::new(Const { val })),
            Token::LParen => operators.push(cur),
            Token::RParen => {
                if matches!(prev, Token::LParen) {
                    bail!("Empty parentheses")
                }
                while operators.len() > 0
                    && !matches!(operators[operators.len() - 1], Token::LParen)
                {
                    let node = get_node(operators.pop().unwrap(), &mut operands)?;
                    operands.push(node);
                }
                if operators.len() == 0 {
                    bail!("Mismatched parentheses")
                }
                operators.pop(); // Pop the open paren off the stack
            }
            Token::Op(o) => {
                while operators.len() > 0 {
                    if let Token::Op(peek) = operators[operators.len() - 1] {
                        if peek.get_precendence() > o.get_precendence() {
                            let node = get_node(operators.pop().unwrap(), &mut operands)?;
                            operands.push(node);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                operators.push(cur);
            }
        }
        prev = cur;
    }
    while operators.len() > 0 {
        let op = operators.pop().unwrap();
        if matches!(op, Token::LParen) {
            bail!("Mismatched parentheses")
        }
        let node = get_node(op, &mut operands)?;
        operands.push(node);
    }
    if operands.len() != 1 {
        bail!("Expression is missing an operator");
    }

    let expr = operands.pop().unwrap();
    Ok(expr.eval())
}

fn get_node(token: Token, operands: &mut Vec<Node>) -> Result<Node> {
    match token {
        Token::Const(val) => Ok(Box::new(Const { val })),
        Token::Op(o) => {
            let right = operands.pop().ok_or(anyhow!("Missing an operand"))?;
            let left = operands.pop().ok_or(anyhow!("Missing an operand"))?;
            Ok(Box::new(Binary {
                operator: o,
                left,
                right,
            }))
        }
        _ => bail!("Unknown operator: {:?}", token),
    }
}

fn eval_operator(op: &Operator, left: isize, right: isize) -> isize {
    match op {
        Operator::Add => (left + right),
        Operator::Sub => (left - right),
        Operator::Mul => (left * right),
        Operator::Div => (left / right),
    }
}

// Leaving this in for old time's sake
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
        return Some(eval_operator(o, left, right));
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

pub fn part2() {
    let data = load_data();
    let mut sum = 0;
    for problem in data {
        let res = parse_expr(&problem).unwrap();
        sum += res;
    }
    println!("Answer: {}", sum);
}
