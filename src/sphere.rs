use mini_math::Point;

/// A sphere
#[derive(Debug)]
pub struct Sphere {
    /// The center of the sphere
    pub center: Point,
    /// The radius of the sphere
    pub radius: f32,
}

impl Sphere {
    /// Construct a sphere from a center point and a radius
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}
