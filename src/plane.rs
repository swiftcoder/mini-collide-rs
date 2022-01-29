use crate::Triangle;
use mini_math::{Point, Vector3};

/// An infinite plane.
#[derive(Debug)]
pub struct Plane {
    /// The normal that liest perpendicular to the plane.
    pub normal: Vector3,
    /// The distance from the plane to the origin.
    pub d: f32,
}

impl Plane {
    /// Construct a plane given the components of the plan equation.
    pub fn new(normal: Vector3, d: f32) -> Self {
        Self { normal, d }
    }

    /// Constructs a plane from three points that lie on the plane.
    pub fn from_points(p0: Point, p1: Point, p2: Point) -> Self {
        let normal = -(p1 - p0).cross(p2 - p0).normalized();
        let d = Vector3::from(p0).dot(normal);
        Self { normal, d }
    }

    /// Constructs a plane from a point that lies on the plane, and the normal to the plane.
    pub fn from_point_and_normal(p: Point, normal: Vector3) -> Self {
        Self {
            normal,
            d: Vector3::from(p).dot(normal),
        }
    }
}

impl From<&Triangle> for Plane {
    /// Convert a vector into a point
    fn from(t: &Triangle) -> Self {
        Plane::from_points(t.a, t.b, t.c)
    }
}
