use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point2D<T> {
    x: T,
    y: T,
}

impl<T: Add<Output = T>> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Point2D<T>) -> Self::Output {
        Point2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl <T: Sub<Output = T>> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

fn main() {
    let a = Point2D { x: 1, y: 2 };
    let b = Point2D { x: 0, y: -1};
    assert_eq!(a + b, Point2D { x: 1, y: 1});
    assert_eq!(a - b, Point2D { x: 1, y: 3});
}
