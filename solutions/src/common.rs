use impl_ops::*;
use std::ops;

type PointTuple = (usize, usize);
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
impl_op!(+ |a: Point, b: Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: Point, b: Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: Point, b: PointTuple| -> Point { Point::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: Point, b: PointTuple| -> Point { Point::new(a.x - b.0, a.y - b.1) });

type IPointTuple = (isize, isize);
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
impl_op!(+ |a: IPoint, b: IPoint| -> IPoint { IPoint::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: IPoint, b: IPoint| -> IPoint { IPoint::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x - b.0, a.y - b.1) });
