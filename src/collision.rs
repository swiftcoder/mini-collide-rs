use crate::{LineSegment, Plane, Ray, Sphere, Triangle};
use mini_math::{NearlyEqual, Point, Vector3};

/// The result of a collision.
#[derive(PartialEq, Debug)]
pub struct Contact {
    /// The point at which the collision occurs.
    pub point: Point,
    /// The surface normal at the point of collision.
    pub normal: Vector3,
    /// The distance by which the colliding shapes overlap.
    pub overlap: f32,
}

impl NearlyEqual for &Contact {
    fn nearly_equals(self, rhs: Self) -> bool {
        self.point.nearly_equals(&rhs.point)
            && self.normal.nearly_equals(&rhs.normal)
            && self.overlap.nearly_equals(rhs.overlap)
    }
}

impl Contact {
    fn new(point: Point, normal: Vector3, overlap: f32) -> Self {
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

impl Collision<Triangle> for Ray {
    fn collides(&self, triangle: &Triangle) -> Option<Contact> {
        let plane = Plane::from(triangle);

        let n_dot_r = plane.normal.dot(self.direction);
        // early exit if ray parallel to plane
        if n_dot_r.abs() < std::f32::EPSILON {
            return None;
        }

        let d = plane.normal.dot(Vector3::from(triangle.a));
        let e = plane.normal.dot(Vector3::from(self.origin));
        let t = (e + d) / n_dot_r;

        // early exit if triangle entirely behind ray
        if t > 0.0 {
            return None;
        }

        let intersection_point = self.origin + self.direction * -t;
        if triangle.coplanar_point_inside(intersection_point) {
            Some(Contact::new(intersection_point, plane.normal, 0.0))
        } else {
            None
        }
    }
}

impl Collision<Triangle> for LineSegment {
    fn collides(&self, triangle: &Triangle) -> Option<Contact> {
        let plane = Plane::from(triangle);

        let mut direction = self.end - self.start;
        let length = direction.magnitude();
        direction /= length;

        let n_dot_r = plane.normal.dot(direction);
        // early exit if line parallel to plane
        if n_dot_r.abs() < std::f32::EPSILON {
            return None;
        }

        let d = plane.normal.dot(Vector3::from(triangle.a));
        let e = plane.normal.dot(Vector3::from(self.start));
        let t = (e + d) / n_dot_r;

        // early exit if triangle is entirely in fornt or behind of the line segment
        if t > 0.0 || t < -length {
            return None;
        }

        let intersection_point = self.start + direction * -t;
        if triangle.coplanar_point_inside(intersection_point) {
            Some(Contact::new(intersection_point, plane.normal, 0.0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mini_math::{Point, Vector3};

    #[test]
    fn test_sphere_sphere_collision() {
        let a = Sphere::new(Point::zero(), 1.0);
        let b = Sphere::new(Point::new(0.0, 1.5, 0.0), 1.0);

        assert_eq!(
            b.collides(&a),
            Some(Contact::new(
                Point::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
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
                Vector3::new(0.0, 1.0, 0.0),
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

    #[test]
    fn test_triangle_ray_collision() {
        let triangle = Triangle::new(
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );

        // parallel
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.collides(&triangle), None);

        // in front
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.collides(&triangle), None);

        // behind
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        assert_eq!(ray.collides(&triangle), None);

        // past
        let ray = Ray::new(Point::new(3.0, 1.0, 3.0), Vector3::new(0.0, -1.0, 0.0));
        assert_eq!(ray.collides(&triangle), None);

        // straight through
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        assert_eq!(
            ray.collides(&triangle),
            Some(Contact::new(
                Point::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                0.0
            ))
        );

        // diagonally through
        let ray = Ray::new(
            Point::new(-0.5, -1.0, 0.0),
            Vector3::new(0.5, 1.0, 0.0).normalized(),
        );
        assert_eq!(
            ray.collides(&triangle),
            Some(Contact::new(
                Point::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                0.0
            ))
        );
    }
}
