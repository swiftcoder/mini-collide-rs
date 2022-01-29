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
