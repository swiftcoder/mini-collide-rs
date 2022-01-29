use crate::Distance;
use mini_math::{Point, Vector3};

/// An infinite line.
#[derive(Debug)]
pub struct Line {
    /// An arbitrary point on the line.
    pub point: Point,
    /// The direction of the line.
    pub direction: Vector3,
}

impl Line {
    /// Construct a line from a point on the line and its direction.
    pub fn new(point: Point, direction: Vector3) -> Self {
        Self { point, direction }
    }

    /// Construct a line from two points on the line.
    pub fn from_points(start: Point, end: Point) -> Self {
        Self {
            point: start,
            direction: (end - start).normalized(),
        }
    }
}

impl Distance<Point> for Line {
    /// Returns the distance between the line and a given point.
    fn distance(&self, p: Point) -> f32 {
        let cross = self.direction.cross(p - self.point);
        cross.magnitude()
    }
}

impl Distance<&Line> for Line {
    /// Returns the distance between the line and another line.
    fn distance(&self, line: &Line) -> f32 {
        let w = self.point - line.point;
        let b = self.direction.dot(line.direction);
        let d = self.direction.dot(w);
        let e = line.direction.dot(w);
        let d_p = 1.0 - b * b;

        let (sc, tc) = if d_p < std::f32::EPSILON {
            (0.0, if b > 1.0 { d / b } else { e })
        } else {
            ((b * e - d) / d_p, (e - b * d) / d_p)
        };

        let p = w + (self.direction * sc) - (line.direction * tc);
        p.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_point() {
        let line = Line::from_points(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(line.distance(p), 0.0);

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(line.distance(p), 5.0);
    }

    #[test]
    fn test_distance_to_line() {
        let line = Line::from_points(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let l = Line::from_points(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 10.0, 10.0));
        assert_eq!(line.distance(&l), 0.0);

        let l = Line::from_points(Point::new(0.0, 5.0, 5.0), Point::new(0.0, 5.0, 15.0));
        assert_eq!(line.distance(&l), 5.0);

        let l = Line::from_points(Point::new(0.0, 5.0, 0.0), Point::new(25.0, 5.0, 0.0));
        assert_eq!(line.distance(&l), 5.0);
    }
}
