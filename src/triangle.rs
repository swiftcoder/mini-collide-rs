use mini_math::{Point, Vector};

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
    pub fn barycentric_coordinates(&self, p: Point) -> Vector {
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

        Vector::new(u, v, w)
    }
}
