use mini_math::{Point, Vector};

/// An infinite plane.
pub struct Plane {
    /// The normal that liest perpendicular to the plane.
    pub normal: Vector,
    /// The distance from the plane to the origin.
    pub d: f32,
}

impl Plane {
    /// Construct a plane given the components of the plan equation.
    pub fn new(normal: Vector, d: f32) -> Self {
        Self { normal, d }
    }

    /// Constructs a plane from three points that lie on the plane.
    pub fn from_points(p0: Point, p1: Point, p2: Point) -> Self {
        let normal = (p1 - p0).cross(p1 - p2).normalized();
        let d = -Vector::from(p1).dot(normal);
        Self { normal, d }
    }

    /// Returns the signed distance between the plane and a given point.
    pub fn distance(&self, p: Point) -> f32 {
        self.normal.dot(Vector::from(p)) - self.d
    }

    /// Returns the point on the plane that is closest to the given point.
    pub fn point_closest_to(&self, p: Point) -> Point {
        let distance = self.distance(p);
        p - self.normal * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let p = Point::new(1.0, 1.0, 1.0);
        assert_eq!(plane.distance(p), 1.0);
    }

    #[test]
    fn test_closest_point() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let p = Point::new(1.0, 1.0, 1.0);
        assert_eq!(plane.point_closest_to(p), Point::new(1.0, 0.0, 1.0));
    }
}
