use crate::{Plane, Sphere, Triangle};
use mini_math::{Point, Vector};

/// The result of a collision.
#[derive(PartialEq, Debug)]
pub struct Contact {
    /// The point at which the collision occurs.
    pub point: Point,
    /// The surface normal at the point of collision.
    pub normal: Vector,
    /// The distance by which the colliding shapes overlap.
    pub overlap: f32,
}

impl Contact {
    fn new(point: Point, normal: Vector, overlap: f32) -> Self {
        Self {
            point,
            normal,
            overlap,
        }
    }
}

/// Trait for determining the collision between two shapes.
pub trait Collision<Rhs> {
    /// Whether this shape collides with the other, and where.
    fn collides(&self, rhs: &Rhs) -> Option<Contact>;
}

impl Collision<Sphere> for Sphere {
    fn collides(&self, sphere: &Sphere) -> Option<Contact> {
        let combined_radius = self.radius + sphere.radius;
        let diff = self.center - sphere.center;
        let distance_squared = diff.magnitude_squared();
        if distance_squared > combined_radius * combined_radius {
            None
        } else {
            let distance = distance_squared.sqrt();
            let normal = diff / distance;

            Some(Contact::new(
                sphere.center + normal * sphere.radius,
                normal,
                combined_radius - distance,
            ))
        }
    }
}

impl Collision<Triangle> for Sphere {
    fn collides(&self, triangle: &Triangle) -> Option<Contact> {
        let plane = Plane::from(triangle);

        let p = plane.point_closest_to(self.center);
        let distance_from_plane_squared = (p - self.center).magnitude_squared();

        if distance_from_plane_squared > self.radius * self.radius {
            None
        } else {
            let q = triangle.point_closest_to(self.center);
            let diff = q - self.center;
            let overlap = self.radius - diff.magnitude();
            if overlap < 0.0 {
                None
            } else {
                Some(Contact::new(q, plane.normal, overlap))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mini_math::{Point, Vector};

    #[test]
    fn test_sphere_sphere_collision() {
        let a = Sphere::new(Point::zero(), 1.0);
        let b = Sphere::new(Point::new(0.0, 1.5, 0.0), 1.0);

        assert_eq!(
            b.collides(&a),
            Some(Contact::new(
                Point::new(0.0, 1.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                0.5
            ))
        );
    }

    #[test]
    fn test_sphere_triangle_collision() {
        let a = Triangle::new(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );
        let b = Sphere::new(Point::new(0.0, 0.75, 0.0), 1.0);

        assert_eq!(
            b.collides(&a),
            Some(Contact::new(
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
                0.25
            ))
        );

        let b = Sphere::new(Point::new(0.0, 1.75, 0.0), 1.0);
        assert_eq!(b.collides(&a), None);

        let b = Sphere::new(Point::new(0.0, -1.75, 0.0), 1.0);
        assert_eq!(b.collides(&a), None);

        let b = Sphere::new(Point::new(-3.0, 0.0, -3.0), 1.0);
        assert_eq!(b.collides(&a), None);
    }
}
