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

    pub(crate) fn point_closest_to_edge(e0: Point, e1: Point, p: Point) -> Point {
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
