use crate::Distance;
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

impl Distance<Point> for Sphere {
    /// Returns the distance between the sphere and a given point.
    fn distance(&self, p: Point) -> f32 {
        (p - self.center).magnitude() - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 5.0);

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(sphere.distance(p), 0.0);

        let p = Point::new(0.0, 0.0, 15.0);
        assert_eq!(sphere.distance(p), 10.0);
    }
}
