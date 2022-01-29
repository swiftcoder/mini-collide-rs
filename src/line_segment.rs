use mini_math::Point;

/// A finite line segment
#[derive(Debug)]
pub struct LineSegment {
    /// The start point of the line segment
    pub start: Point,
    /// The end point of the line segment
    pub end: Point,
}

impl LineSegment {
    /// Construct a ray from a starting point and direction
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}
