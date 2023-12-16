#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}
