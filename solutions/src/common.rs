#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct IPoint {
    pub x: isize,
    pub y: isize,
}
impl IPoint {
    pub fn new(x: isize, y: isize) -> Self {
        IPoint { x, y }
    }
}

// Still vaguely wondering why these algos aren't in the standard lib tbh
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(input: &[usize]) -> usize {
    if input.len() == 0 {
        return 0;
    }
    if input.len() == 1 {
        return input[0];
    }
    let a = input[0];
    let b = lcm(&input[1..]);
    a * b / gcd(a, b)
}
