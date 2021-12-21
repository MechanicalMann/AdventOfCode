use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::{bail, Result};
use itertools::Itertools;

use crate::solver::Solver;

pub struct Solution;
impl Solver<usize, usize> for Solution {
    const DAY: u8 = 18;

    fn new() -> Self {
        Solution {}
    }

    fn part_one(&self) -> anyhow::Result<usize> {
        Ok(0)
    }

    fn part_two(&self) -> anyhow::Result<usize> {
        Ok(0)
    }
}

// snail(fish) number... snumber
// effervescent
#[derive(Clone, Debug, PartialEq)]
enum Snumber {
    Number(usize),
    Pair(Box<Pair>),
}
impl Display for Snumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Snumber::Number(n) => write!(f, "{}", n),
            Snumber::Pair(p) => write!(f, "{}", p),
        }
    }
}
impl From<Pair> for Snumber {
    fn from(pair: Pair) -> Self {
        Snumber::Pair(Box::new(pair.to_owned()))
    }
}
impl Add<Snumber> for Snumber {
    type Output = Snumber;

    fn add(self, rhs: Snumber) -> Self::Output {
        if let (Snumber::Number(lv), Snumber::Number(rv)) = (self, rhs) {
            Snumber::Number(lv + rv)
        } else {
            panic!("Invalid operation: can only add regular number snumbers")
        }
    }
}
impl AddAssign<Snumber> for Snumber {
    fn add_assign(&mut self, rhs: Snumber) {
        match (self.clone(), rhs) {
            (Snumber::Number(l), Snumber::Number(r)) => {
                *self = Snumber::Number(l + r);
            }
            _ => panic!("Invalid operation: can only add regular number snumbers"),
        }
    }
}
impl Snumber {
    fn magnitude(&self) -> usize {
        match self {
            &Snumber::Number(n) => n,
            Snumber::Pair(p) => p.magnitude(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Pair {
    left: Snumber,
    right: Snumber,
}
impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}
impl Add<Pair> for Pair {
    type Output = Self;

    fn add(self, rhs: Pair) -> Self::Output {
        let pair = Pair {
            left: Snumber::from(self),
            right: Snumber::from(rhs),
        };
        pair.reduce().unwrap()
    }
}
impl FromStr for Pair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Pair::parse(s)
    }
}
impl Pair {
    fn magnitude(&self) -> usize {
        (self.left.magnitude() * 3) + (self.right.magnitude() * 2)
    }

    fn parse(s: &str) -> Result<Self> {
        _parse(s).and_then(|r| Ok(r.0))
    }

    fn reduce(&self) -> Result<Pair> {
        let mut p = self.clone();
        println!("Initial value:  {}", p);
        loop {
            if let Some((_, r)) = reduce_pair(&p, 0) {
                println!("    Reduced to: {}", r);
                p = r;
            } else {
                println!("    Fully reduced.");
                break;
            }
        }
        Ok(p)
    }
}

fn _parse(s: &str) -> Result<(Pair, usize)> {
    let chars = s.chars().collect_vec();
    if let Some(&c) = chars.get(0) {
        if c != '[' {
            bail!("Invalid pair: expected '[', got '{}'", c);
        }
    } else {
        bail!("Cannot parse an empty string");
    }

    let mut stack: Vec<Snumber> = vec![];
    let mut ptr = 1;

    while let Some(c) = chars.get(ptr) {
        match c {
            '[' => {
                let (pair, read) = _parse(&s[ptr..])?;
                stack.push(Snumber::Pair(Box::new(pair)));
                ptr += read;
            }
            d if c.is_digit(10) => {
                stack.push(Snumber::Number(d.to_digit(10).unwrap() as usize));
                ptr += 1;
            }
            ',' => {
                ptr += 1;
            }
            ']' => {
                ptr += 1;
                break;
            }
            _ => bail!("Invalid pair: unknown char {}", c),
        }
    }

    let (right, left) = (stack.pop().unwrap(), stack.pop().unwrap());

    Ok((Pair { left, right }, ptr))
}

fn reduce_snumber(snum: &mut Snumber) {}

enum Reduction {
    Explode(usize, usize),
    Split,
}

// I hate this.  But trying to generalize it led to all kinds of reborrowing problems.
// So fuck me I guess.
fn reduce_pair(pair: &Pair, level: usize) -> Option<(Reduction, Pair)> {
    println!("{:w$}Reducing: {}", "", pair, w = level);
    let mut left: Option<Snumber> = None;
    let mut right: Option<Snumber> = None;
    let mut red: Option<Reduction> = None;

    match &pair.left {
        &Snumber::Number(v) if v > 9 => {
            let nl = v / 2;
            let nr = (v / 2) + 1;
            left = Some(Snumber::from(Pair {
                left: Snumber::Number(nl),
                right: Snumber::Number(nr),
            }));
            red = Some(Reduction::Split);
        }
        Snumber::Pair(bpair) => {
            if level >= 3 {
                if let (Snumber::Number(l), Snumber::Number(r)) = (&bpair.left, &bpair.right) {
                    left = Some(Snumber::Number(0));
                    match &pair.right {
                        &Snumber::Number(val) => {
                            right = Some(Snumber::Number(val + r));
                            red = Some(Reduction::Explode(*l, 0));
                        }
                        Snumber::Pair(rp) => {
                            let mut ptr = rp;
                            loop {
                                if let (Snumber::Number(rpl), rpr) = (&ptr.left, &ptr.right) {
                                    right = Some(Snumber::from(Pair {
                                        left: Snumber::Number(rpl + r),
                                        right: rpr.clone(),
                                    }));
                                    break;
                                }
                            }
                            red = Some(Reduction::Explode(*l, 0));
                        }
                        _ => {
                            red = Some(Reduction::Explode(*l, *r));
                        }
                    }
                } else {
                    panic!("Invalid snumber!");
                }
            } else {
                match reduce_pair(bpair, level + 1) {
                    Some((Reduction::Explode(l, r), inner)) => {
                        match &pair.right {
                            Snumber::Number(val) => {
                                right = Some(Snumber::Number(val + r));
                            }
                            Snumber::Pair(rp) => {
                                if let (Snumber::Number(rpl), rpr) = (&rp.left, &rp.right) {
                                    right = Some(Snumber::from(Pair {
                                        left: Snumber::Number(rpl + r),
                                        right: rpr.clone(),
                                    }));
                                }
                            }
                            _ => panic!("Exploding to nested pairs, halp"),
                        }
                        left = Some(Snumber::from(inner));
                        red = Some(Reduction::Explode(l, 0));
                    }
                    Some((Reduction::Split, inner)) => {
                        left = Some(Snumber::from(inner));
                        red = Some(Reduction::Split);
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    if let Some(r) = red {
        return Some((
            r,
            Pair {
                left: left.or(Some(pair.left.clone()))?,
                right: right.or(Some(pair.right.clone()))?,
            },
        ));
    }

    match &pair.right {
        &Snumber::Number(v) if v > 9 => {
            let nl = v / 2;
            let nr = (v / 2) + 1;
            right = Some(Snumber::from(Pair {
                left: Snumber::Number(nl),
                right: Snumber::Number(nr),
            }));
            red = Some(Reduction::Split);
        }
        Snumber::Pair(bpair) => {
            if level >= 3 {
                if let (Snumber::Number(l), Snumber::Number(r)) = (&bpair.left, &bpair.right) {
                    right = Some(Snumber::Number(0));
                    match &pair.left {
                        &Snumber::Number(val) => {
                            left = Some(Snumber::Number(val + l));
                            red = Some(Reduction::Explode(0, *r));
                        }
                        Snumber::Pair(lp) => {
                            if let (lpl, Snumber::Number(lpr)) = (&lp.left, &lp.right) {
                                left = Some(Snumber::from(Pair {
                                    left: lpl.clone(),
                                    right: Snumber::Number(lpr + l),
                                }));
                            }
                            red = Some(Reduction::Explode(0, *r));
                        }
                        _ => {
                            red = Some(Reduction::Explode(*l, *r));
                        }
                    }
                } else {
                    panic!("Invalid snumber!");
                }
            } else {
                match reduce_pair(bpair, level + 1) {
                    Some((Reduction::Explode(l, r), inner)) => {
                        match &pair.left {
                            Snumber::Number(val) => {
                                left = Some(Snumber::Number(val + l));
                            }
                            Snumber::Pair(lp) => {
                                if let (lpl, Snumber::Number(lpr)) = (&lp.left, &lp.right) {
                                    left = Some(Snumber::from(Pair {
                                        left: lpl.clone(),
                                        right: Snumber::Number(lpr + l),
                                    }));
                                }
                            }
                            _ => panic!("Exploding to nested pairs, halp"),
                        }
                        right = Some(Snumber::from(inner));
                        red = Some(Reduction::Explode(0, r));
                    }
                    Some((Reduction::Split, inner)) => {
                        right = Some(Snumber::from(inner));
                        red = Some(Reduction::Split);
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    if let Some(r) = red {
        return Some((
            r,
            Pair {
                left: left.or(Some(pair.left.clone()))?,
                right: right.or(Some(pair.right.clone()))?,
            },
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse() -> Result<()> {
        let input = "[1,2]";
        let pair = Pair::parse(input)?;
        assert_eq!(Snumber::Number(1), pair.left);
        assert_eq!(Snumber::Number(2), pair.right);
        Ok(())
    }

    #[test]
    fn should_parse_nested() -> Result<()> {
        let input = "[[1,2],[3,4]]";
        let pair = Pair::parse(input)?;
        let l_expected = Snumber::from(Pair {
            left: Snumber::Number(1),
            right: Snumber::Number(2),
        });
        let r_expected = Snumber::from(Pair {
            left: Snumber::Number(3),
            right: Snumber::Number(4),
        });
        assert_eq!(l_expected, pair.left);
        assert_eq!(r_expected, pair.right);
        Ok(())
    }

    #[test]
    fn should_add() -> Result<()> {
        let l = Pair::parse("[1,2]")?;
        let r = Pair::parse("[3,4]")?;

        let res = l + r;

        let l_expected = Snumber::from(Pair {
            left: Snumber::Number(1),
            right: Snumber::Number(2),
        });
        let r_expected = Snumber::from(Pair {
            left: Snumber::Number(3),
            right: Snumber::Number(4),
        });
        let expected = Pair {
            left: l_expected,
            right: r_expected,
        };
        assert_eq!(expected, res);
        Ok(())
    }

    #[test]
    fn should_explode() -> Result<()> {
        let pair = Pair::parse("[[[[[9,8],1],2],3],4]")?;
        let reduced = pair.reduce()?;
        let expected = Pair::parse("[[[[0,9],2],3],4]")?;
        assert_eq!(expected, reduced);

        let pair = Pair::parse("[7,[6,[5,[4,[3,2]]]]]")?;
        let reduced = pair.reduce()?;
        let expected = Pair::parse("[7,[6,[5,[7,0]]]]")?;
        assert_eq!(expected, reduced);

        let pair = Pair::parse("[[6,[5,[4,[3,2]]]],1]")?;
        let reduced = pair.reduce()?;
        let expected = Pair::parse("[[6,[5,[7,0]]],3]")?;
        assert_eq!(expected, reduced);

        // Step 1
        let pair = Pair::parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?;
        let (_, reduced) = reduce_pair(&pair, 0).unwrap();
        let expected = Pair::parse("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?;
        assert_eq!(expected, reduced);

        // Step 2
        let (_, reduced) = reduce_pair(&reduced, 0).unwrap();
        let expected = Pair::parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")?;
        assert_eq!(expected, reduced);
        Ok(())
    }

    #[test]
    fn should_split() -> Result<()> {
        let pair = Pair {
            left: Snumber::Number(11),
            right: Snumber::Number(0),
        };
        let actual = pair.reduce()?;
        let expected = Pair::parse("[[5,6],0]")?;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn should_solve_part1_example1() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]";
        let sum = input
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .reduce(|a, b| a + b)
            .unwrap();
        let expected = Pair::parse("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
        assert_eq!(expected, sum);
    }

    #[test]
    fn should_solve_part1_example2() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
        let sum = input
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .reduce(|a, b| a + b)
            .unwrap();
        let expected = Pair::parse("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
        assert_eq!(expected, sum);
    }

    #[test]
    fn should_solve_part1_example3() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        let sum = input
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .reduce(|a, b| a + b)
            .unwrap();
        let expected = Pair::parse("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
        assert_eq!(expected, sum);
    }

    #[test]
    fn should_solve_part1_example4() {
        let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]";
        let sum = input
            .lines()
            .map(|l| l.parse::<Pair>().unwrap())
            .reduce(|a, b| a + b)
            .unwrap();
        let expected =
            Pair::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]").unwrap();
        println!("Expected: {}", expected);
        println!("Actual:   {}", sum);
        assert_eq!(expected, sum);
    }
}
