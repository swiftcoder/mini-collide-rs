use crate::Distance;
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

impl Distance<Point> for Ray {
    /// Returns the distance between the ray and a given point.
    fn distance(&self, p: Point) -> f32 {
        let diff = p - self.origin;
        let dot = self.direction.dot(diff);
        if dot < 0.0 {
            return diff.magnitude();
        }
        let cross = self.direction.cross(diff);
        cross.magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(ray.distance(p), 5.0);

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(ray.distance(p), 5.0);
    }
}
