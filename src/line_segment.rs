use crate::Distance;
use mini_math::Point;

/// A finite line segment.
#[derive(Debug)]
pub struct LineSegment {
    /// The start point of the line segment.
    pub start: Point,
    /// The end point of the line segment.
    pub end: Point,
}

impl LineSegment {
    /// Construct a ray from a starting point and direction.
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

impl Distance<Point> for LineSegment {
    /// Returns the distance between the line segment and a given point.
    fn distance(&self, p: Point) -> f32 {
        let mut direction = self.end - self.start;
        let length = direction.magnitude();
        direction /= length;

        let diff = p - self.start;
        let dot = direction.dot(diff);
        if dot < 0.0 {
            return diff.magnitude();
        }
        if dot > length {
            return (p - self.end).magnitude();
        }
        let cross = direction.cross(diff);
        cross.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let line = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(line.distance(p), 5.0);

        let p = Point::new(0.0, 0.0, 15.0);
        assert_eq!(line.distance(p), 5.0);

        let p = Point::new(0.0, 5.0, 5.0);
        assert_eq!(line.distance(p), 5.0);
    }
}
