use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

#[cfg(test)]
mod tests {
    use super::{Point, Meters, Millimeters};

    #[test]
    fn test_adding_points() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 0, y: 1 },
            Point { x: 1, y: 1 }
        );
    }

    #[test]
    fn test_adding_meters_and_millimeters() {
        assert_eq!(
            Millimeters(50) + Meters(1),
            Millimeters(1050)
        );
    }
}