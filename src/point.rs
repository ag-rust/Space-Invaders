use std::ops::Range;
use std::ops::Add;
use size::Size;
use default::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

pub trait RangeInclusion<T> {
    fn includes(&self, element: T) -> bool;
}

impl<T> RangeInclusion<T> for Range<T>
    where T: PartialOrd + Copy
{
    fn includes(&self, element: T) -> bool {
        self.start <= element && element < self.end
    }
}

impl Point {
    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    pub fn overlaps_with(&self, point: &Point, size: &Size) -> bool {
        (point.x..point.x+size.width+1).includes(self.x) &&
            (point.y..point.y+size.height+1).includes(self.y)
    }
}

impl Default for Point {
    fn default() -> Point {
        Point::zero()
    }
}
