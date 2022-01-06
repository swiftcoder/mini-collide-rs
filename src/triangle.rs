use crate::Plane;
use mini_math::{Point, Vector3};

/// A triangle.
#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    /// Construct a new triangle from three vertices.
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    /// Barycentric coordinates of the given point.
    pub fn barycentric_coordinates(&self, p: Point) -> Vector3 {
        let e0 = self.b - self.a;
        let e1 = self.c - self.a;
        let e2 = p - self.a;

        let d00 = e0.dot(e0);
        let d01 = e0.dot(e1);
        let d11 = e1.dot(e1);
        let d20 = e2.dot(e0);
        let d21 = e2.dot(e1);
        let denom = 1.0 / (d00 * d11 - d01 * d01);
        let v = (d11 * d20 - d01 * d21) * denom;
        let w = (d00 * d21 - d01 * d20) * denom;
        let u = 1.0 - v - w;

        Vector3::new(u, v, w)
    }

    /// Test if a coplanar point is inside the triangle
    pub fn coplanar_point_inside(&self, p: Point) -> bool {
        let plane = Plane::from(self);

        let edge_cross = (self.b - self.a).cross(p - self.a);
        // reject if intersection is outside of edge
        if plane.normal.dot(edge_cross) > 0.0 {
            return false;
        }

        let edge_cross = (self.c - self.b).cross(p - self.b);
        // reject if intersection is outside of edge
        if plane.normal.dot(edge_cross) > 0.0 {
            return false;
        }

        let edge_cross = (self.a - self.c).cross(p - self.c);
        // reject if intersection is outside of edge
        if plane.normal.dot(edge_cross) > 0.0 {
            return false;
        }

        true
    }

    /// Returns the point on the triangle that is closest to the given point.
    pub fn point_closest_to(&self, p: Point) -> Point {
        let plane = Plane::from(self);
        let q = plane.point_closest_to(p);

        let coordinates = self.barycentric_coordinates(q);
        if coordinates.x >= 0.0 && coordinates.y >= 0.0 && coordinates.z >= 0.0 {
            return q;
        }

        let p0 = Self::point_closest_to_edge(self.a, self.b, p);
        let p1 = Self::point_closest_to_edge(self.b, self.c, p);
        let p2 = Self::point_closest_to_edge(self.c, self.a, p);

        let d0 = (p0 - p).magnitude_squared();
        let d1 = (p1 - p).magnitude_squared();
        let d2 = (p2 - p).magnitude_squared();

        if d0 < d1 && d0 < d2 {
            p0
        } else if d1 < d0 && d1 < d2 {
            p1
        } else {
            p2
        }
    }

    fn point_closest_to_edge(e0: Point, e1: Point, p: Point) -> Point {
        let edge = e1 - e0;
        let edge_length = edge.magnitude();
        let edge_direction = edge / edge_length;
        let diff = p - e0;
        let d = diff.dot(edge_direction);
        if d < 0.0 {
            e0
        } else if d > edge_length {
            e1
        } else {
            e0 + edge_direction * d
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_point() {
        let triangle = Triangle::new(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let p = Point::new(0.0, 1.0, 0.0);
        assert_eq!(triangle.point_closest_to(p), Point::new(0.0, 0.0, 0.0));

        let p = Point::new(0.0, 1.0, 2.0);
        assert_eq!(triangle.point_closest_to(p), Point::new(0.0, 0.0, 1.0));

        let p = Point::new(0.0, -1.0, -2.0);
        assert_eq!(triangle.point_closest_to(p), Point::new(0.0, 0.0, -1.0));
    }
}
