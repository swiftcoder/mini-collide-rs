use mini_math::Point;

use crate::{Distance, LineSegment};

#[derive(Debug)]
pub struct Capsule {
    /// The central axis of the capsule
    pub axis: LineSegment,
    /// The radius of the capsule
    pub radius: f32,
}

impl Capsule {
    /// Construct a capsule from the end points of the central axis, and a radius
    pub fn new(a: Point, b: Point, radius: f32) -> Self {
        Self {
            axis: LineSegment::new(a, b),
            radius,
        }
    }
}

impl Distance<Point> for Capsule {
    /// Returns the distance between the sphere and a given point.
    fn distance(&self, p: Point) -> f32 {
        self.axis.distance(p) - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let cap = Capsule::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 5.0, 0.0), 1.0);

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(cap.distance(p), 4.0);

        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(cap.distance(p), 4.0);
    }
}
