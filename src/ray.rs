use mini_math::{Point, Vector3};

/// An infinite ray.
#[derive(Debug)]
pub struct Ray {
    /// The starting point of the ray.
    pub origin: Point,
    /// The direction of the ray.
    pub direction: Vector3,
}

impl Ray {
    /// Construct a ray from a starting point and direction.
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}
