use crate::{Plane, Ray, Sphere};
use mini_math::Vector;

/// Trait for determining whether two shapes intersect with one another.
pub trait Intersection<Rhs> {
    /// Whether this shape intersect with the other.
    fn intersects(&self, rhs: &Rhs) -> bool;
}

impl Intersection<Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let a = ray.direction.magnitude_squared();
        let b = 2.0
            * (Vector::from(ray.origin).dot(ray.direction) - ray.direction.dot(self.center.into()));
        let c = (self.center - ray.origin).magnitude_squared() - self.radius * self.radius;
        b * b - 4.0 * a * c >= 0.0
    }
}

impl Intersection<Sphere> for Ray {
    fn intersects(&self, sphere: &Sphere) -> bool {
        sphere.intersects(self)
    }
}

impl Intersection<Ray> for Plane {
    fn intersects(&self, ray: &Ray) -> bool {
        let t =
            -(self.d + Vector::from(ray.origin).dot(self.normal)) / ray.direction.dot(self.normal);
        t >= 0.0
    }
}

impl Intersection<Plane> for Ray {
    fn intersects(&self, plane: &Plane) -> bool {
        plane.intersects(self)
    }
}

impl Intersection<Sphere> for Plane {
    fn intersects(&self, sphere: &Sphere) -> bool {
        self.distance(sphere.center).abs() <= sphere.radius
    }
}

impl Intersection<Plane> for Sphere {
    fn intersects(&self, plane: &Plane) -> bool {
        plane.intersects(self)
    }
}

impl Intersection<Sphere> for Sphere {
    fn intersects(&self, sphere: &Sphere) -> bool {
        let combined_radius = self.radius + sphere.radius;
        (self.center - sphere.center).magnitude_squared() <= combined_radius * combined_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mini_math::{Point, Vector};

    #[test]
    fn test_ray_sphere_intersects() {
        let sphere = Sphere::new(Point::new(0.0, 20.0, 0.0), 10.0);

        let ray = Ray::new(Point::new(-20.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        assert!(!sphere.intersects(&ray));
        assert!(!ray.intersects(&sphere));

        let ray = Ray::new(Point::new(-20.0, 20.0, 0.0), Vector::new(1.0, 0.0, 0.0));
        assert!(sphere.intersects(&ray));
        assert!(ray.intersects(&sphere));
    }

    #[test]
    fn test_ray_plane_intersects() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        assert!(!plane.intersects(&ray));
        assert!(!ray.intersects(&plane));

        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        assert!(plane.intersects(&ray));
        assert!(ray.intersects(&plane));
    }

    #[test]
    fn test_sphere_plane_intersects() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let sphere = Sphere::new(Point::new(0.0, 10.0, 0.0), 5.0);
        assert!(!plane.intersects(&sphere));
        assert!(!sphere.intersects(&plane));

        let sphere = Sphere::new(Point::new(0.0, -10.0, 0.0), 5.0);
        assert!(!plane.intersects(&sphere));
        assert!(!sphere.intersects(&plane));

        let sphere = Sphere::new(Point::new(0.0, 2.0, 0.0), 5.0);
        assert!(plane.intersects(&sphere));
        assert!(sphere.intersects(&plane));
    }

    #[test]
    fn test_sphere_sphere_intersects() {
        let sphere1 = Sphere::new(Point::new(10.0, 0.0, 0.0), 5.0);

        let sphere2 = Sphere::new(Point::new(-10.0, 00.0, 0.0), 5.0);
        assert!(!sphere1.intersects(&sphere2));
        assert!(!sphere2.intersects(&sphere1));

        let sphere2 = Sphere::new(Point::new(0.0, 0.0, 0.0), 7.0);
        assert!(sphere1.intersects(&sphere2));
        assert!(sphere2.intersects(&sphere1));
    }
}
