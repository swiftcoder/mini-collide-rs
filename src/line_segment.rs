use crate::{Distance, Line};
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

    pub(crate) fn clip_point_to_segment(&self, p: Point) -> Point {
        let mut direction = self.end - self.start;
        let length = direction.magnitude();
        direction /= length;

        let dot = (p - self.start).dot(direction);

        if dot < 0.0 {
            self.start
        } else {
            self.start + direction * dot.min(length)
        }
    }
}

impl Distance<Point> for LineSegment {
    /// Returns the distance between the line segment and a given point.
    fn distance(&self, p: Point) -> f32 {
        let q = Line::from_points(self.start, self.end).closest_point(p);
        let q = self.clip_point_to_segment(q);

        (p - q).magnitude()
    }
}

impl Distance<LineSegment> for LineSegment {
    /// Returns the distance between the line segment and another line segment.
    fn distance(&self, l: LineSegment) -> f32 {
        let p = Line::from_points(l.start, l.end)
            .closest_point_to_line(&Line::from_points(self.start, self.end));

        let p = l.clip_point_to_segment(p);

        self.distance(p).min(l.distance(self.start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_point() {
        let line = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(line.distance(p), 5.0);

        let p = Point::new(0.0, 0.0, 15.0);
        assert_eq!(line.distance(p), 5.0);

        let p = Point::new(0.0, 5.0, 5.0);
        assert_eq!(line.distance(p), 5.0);
    }

    #[test]
    fn test_distance_to_line_segment() {
        let line = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let l = LineSegment::new(Point::new(0.0, 0.0, 15.0), Point::new(0.0, 0.0, 20.0));
        assert_eq!(line.distance(l), 5.0);

        let l = LineSegment::new(Point::new(0.0, 7.0, 5.0), Point::new(0.0, 7.0, 20.0));
        assert_eq!(line.distance(l), 7.0);

        let l = LineSegment::new(Point::new(9.0, 0.0, 0.0), Point::new(9.0, 7.0, 0.0));
        assert_eq!(line.distance(l), 9.0);

        let l = LineSegment::new(Point::new(9.0, 1.0, -9.0), Point::new(9.0, 7.0, -9.0));
        assert_eq!(
            line.distance(l),
            (9.0f32 * 9.0 + 9.0 * 9.0 + 1.0 * 1.0).sqrt()
        );

        let l = LineSegment::new(Point::new(0.0, 0.0, -10.0), Point::new(0.0, 0.0, -1.0));
        assert_eq!(line.distance(l), 1.0);
    }
}
