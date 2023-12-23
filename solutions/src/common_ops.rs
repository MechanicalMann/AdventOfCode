// This was getting out of hand to implement inline.
use crate::common::{IPoint, Point};
use impl_ops::*;
use std::ops;

type PointTuple = (usize, usize);
type IPointTuple = (isize, isize);

impl_op!(+ |a: Point, b: Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: Point, b: Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: Point, b: PointTuple| -> Point { Point::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: Point, b: PointTuple| -> Point { Point::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: Point, b: &Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: Point, b: &Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: Point, b: &PointTuple| -> Point { Point::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: Point, b: &PointTuple| -> Point { Point::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: &Point, b: Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: &Point, b: Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: &Point, b: PointTuple| -> Point { Point::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: &Point, b: PointTuple| -> Point { Point::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: &Point, b: &Point| -> Point { Point::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: &Point, b: &Point| -> Point { Point::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: &Point, b: &PointTuple| -> Point { Point::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: &Point, b: &PointTuple| -> Point { Point::new(a.x - b.0, a.y - b.1) });

impl_op!(+ |a: IPoint, b: IPoint| -> IPoint { IPoint::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: IPoint, b: IPoint| -> IPoint { IPoint::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: IPoint, b: &IPoint| -> IPoint { IPoint::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: IPoint, b: &IPoint| -> IPoint { IPoint::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: IPoint, b: &IPointTuple| -> IPoint { IPoint::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: IPoint, b: &IPointTuple| -> IPoint { IPoint::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: &IPoint, b: IPoint| -> IPoint { IPoint::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: &IPoint, b: IPoint| -> IPoint { IPoint::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: &IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: &IPoint, b: IPointTuple| -> IPoint { IPoint::new(a.x - b.0, a.y - b.1) });
impl_op!(+ |a: &IPoint, b: &IPoint| -> IPoint { IPoint::new(a.x + b.x, a.y + b.y) });
impl_op!(-|a: &IPoint, b: &IPoint| -> IPoint { IPoint::new(a.x - b.x, a.y - b.y) });
impl_op!(+ |a: &IPoint, b: &IPointTuple| -> IPoint { IPoint::new(a.x + b.0, a.y + b.1) });
impl_op!(-|a: &IPoint, b: &IPointTuple| -> IPoint { IPoint::new(a.x - b.0, a.y - b.1) });
