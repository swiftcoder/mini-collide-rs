use mini_math::Point;

use crate::LineSegment;

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
