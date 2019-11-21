use crate::{Distance, LineSegment, Plane, Ray, Sphere, Triangle};
use mini_math::Vector3;

/// Trait for determining whether two shapes intersect with one another.
pub trait Intersection<Rhs> {
    /// Whether this shape intersect with the other.
    fn intersects(&self, rhs: &Rhs) -> bool;
}

impl Intersection<Ray> for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let a = ray.direction.magnitude_squared();
        let b = 2.0
            * (Vector3::from(ray.origin).dot(ray.direction)
                - ray.direction.dot(self.center.into()));
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
            -(self.d + Vector3::from(ray.origin).dot(self.normal)) / ray.direction.dot(self.normal);
        t >= 0.0
    }
}

impl Intersection<Plane> for Ray {
    fn intersects(&self, plane: &Plane) -> bool {
        plane.intersects(self)
    }
}

impl Intersection<LineSegment> for Sphere {
    fn intersects(&self, line: &LineSegment) -> bool {
        let direction = line.end - line.start;
        let diff = self.center - line.end;
        let t = direction.dot(diff);
        let closest = if t >= 0.0 {
            line.end
        } else {
            let diff = self.center - line.start;
            let t = direction.dot(diff);
            if t <= 0.0 {
                line.start
            } else {
                let length_squared = direction.magnitude_squared();
                if length_squared > 0.0 {
                    line.start + direction * (t / length_squared)
                } else {
                    line.start
                }
            }
        };

        (self.center - closest).magnitude_squared() - self.radius * self.radius <= 0.0
    }
}

impl Intersection<Sphere> for LineSegment {
    fn intersects(&self, sphere: &Sphere) -> bool {
        sphere.intersects(self)
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

impl Intersection<Sphere> for Triangle {
    fn intersects(&self, sphere: &Sphere) -> bool {
        let plane = Plane::from(self);

        let p = plane.point_closest_to(sphere.center);
        let distance_from_plane_squared = (p - sphere.center).magnitude_squared();

        if distance_from_plane_squared > sphere.radius * sphere.radius {
            return false;
        }

        let radius_on_plane = (sphere.radius * sphere.radius - distance_from_plane_squared).sqrt();
        let coordinates = self.barycentric_coordinates(p);

        coordinates.x > -radius_on_plane
            && coordinates.y > -radius_on_plane
            && coordinates.z > -radius_on_plane
    }
}

impl Intersection<Triangle> for Sphere {
    fn intersects(&self, triangle: &Triangle) -> bool {
        triangle.intersects(self)
    }
}

impl Intersection<Ray> for Triangle {
    fn intersects(&self, ray: &Ray) -> bool {
        let plane = Plane::from(self);

        let n_dot_r = plane.normal.dot(ray.direction);
        // early exit if ray parallel to plane
        if n_dot_r.abs() < std::f32::EPSILON {
            return false;
        }

        let d = plane.normal.dot(Vector3::from(self.a));
        let e = plane.normal.dot(Vector3::from(ray.origin));
        let t = (e + d) / n_dot_r;

        // early exit if triangle entirely behind ray
        if t > 0.0 {
            return false;
        }

        let intersection_point = ray.origin + ray.direction * -t;
        self.coplanar_point_inside(intersection_point)
    }
}

impl Intersection<Triangle> for Ray {
    fn intersects(&self, triangle: &Triangle) -> bool {
        triangle.intersects(self)
    }
}

impl Intersection<LineSegment> for Triangle {
    fn intersects(&self, line: &LineSegment) -> bool {
        let plane = Plane::from(self);

        let mut direction = line.end - line.start;
        let length = direction.magnitude();
        direction /= length;

        let n_dot_r = plane.normal.dot(direction);
        // early exit if line parallel to plane
        if n_dot_r.abs() < std::f32::EPSILON {
            return false;
        }

        let d = plane.normal.dot(Vector3::from(self.a));
        let e = plane.normal.dot(Vector3::from(line.start));
        let t = (e + d) / n_dot_r;

        // early exit if triangle is entirely in fornt or behind of the line segment
        if t > 0.0 || t < -length {
            return false;
        }

        let intersection_point = line.start + direction * -t;
        self.coplanar_point_inside(intersection_point)
    }
}

impl Intersection<Triangle> for LineSegment {
    fn intersects(&self, triangle: &Triangle) -> bool {
        triangle.intersects(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mini_math::{Point, Vector3};

    #[test]
    fn test_ray_sphere_intersects() {
        let sphere = Sphere::new(Point::new(0.0, 20.0, 0.0), 10.0);

        let ray = Ray::new(Point::new(-20.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(!sphere.intersects(&ray));
        assert!(!ray.intersects(&sphere));

        let ray = Ray::new(Point::new(-20.0, 20.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        assert!(sphere.intersects(&ray));
        assert!(ray.intersects(&sphere));
    }

    #[test]
    fn test_segment_sphere_intersects() {
        let sphere = Sphere::new(Point::new(0.0, 20.0, 0.0), 10.0);

        let segment = LineSegment::new(Point::new(-20.0, 0.0, 0.0), Point::new(-10.0, 0.0, 0.0));
        assert!(!sphere.intersects(&segment));
        assert!(!segment.intersects(&sphere));

        let segment = LineSegment::new(Point::new(10.0, 0.0, 0.0), Point::new(20.0, 0.0, 0.0));
        assert!(!sphere.intersects(&segment));
        assert!(!segment.intersects(&sphere));

        let segment = LineSegment::new(Point::new(-20.0, 20.0, 0.0), Point::new(20.0, 0.0, 0.0));
        assert!(sphere.intersects(&segment));
        assert!(segment.intersects(&sphere));
    }

    #[test]
    fn test_ray_plane_intersects() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(!plane.intersects(&ray));
        assert!(!ray.intersects(&plane));

        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
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

    #[test]
    fn test_triangle_sphere_intersects() {
        let triangle = Triangle::new(
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );

        // distance from plane in the positive direction
        let sphere = Sphere::new(Point::new(0.0, 1.0, 0.0), 0.5);
        assert!(!triangle.intersects(&sphere));
        assert!(!sphere.intersects(&triangle));

        // distance from plane in the negative direction
        let sphere = Sphere::new(Point::new(0.0, -1.0, 0.0), 0.5);
        assert!(!triangle.intersects(&sphere));
        assert!(!sphere.intersects(&triangle));

        // distance from the plane in the direction of each edge
        let sphere = Sphere::new(Point::new(0.0, 0.0, -3.0), 0.5);
        assert!(!triangle.intersects(&sphere));
        assert!(!sphere.intersects(&triangle));

        let sphere = Sphere::new(Point::new(3.0, 0.0, 3.0), 0.5);
        assert!(!triangle.intersects(&sphere));
        assert!(!sphere.intersects(&triangle));

        let sphere = Sphere::new(Point::new(-3.0, 0.0, 3.0), 0.5);
        assert!(!triangle.intersects(&sphere));
        assert!(!sphere.intersects(&triangle));

        // diagonally from an edge
        let sphere = Sphere::new(Point::new(0.0, 0.3, -0.3), 0.5);
        assert!(triangle.intersects(&sphere));
        assert!(sphere.intersects(&triangle));

        // in the middle of the triangle
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 0.5);
        assert!(triangle.intersects(&sphere));
        assert!(sphere.intersects(&triangle));
    }

    #[test]
    fn test_triangle_ray_intersects() {
        let triangle = Triangle::new(
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );

        // parallel
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert!(!triangle.intersects(&ray));

        // in front
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        assert!(!triangle.intersects(&ray));

        // behind
        let ray = Ray::new(Point::new(0.0, -1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        assert!(!triangle.intersects(&ray));

        // past
        let ray = Ray::new(Point::new(3.0, 1.0, 3.0), Vector3::new(0.0, -1.0, 0.0));
        assert!(!triangle.intersects(&ray));

        // straight through
        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        assert!(triangle.intersects(&ray));

        // diagonally through
        let ray = Ray::new(
            Point::new(-0.5, -1.0, 0.0),
            Vector3::new(0.5, 1.0, 0.0).normalized(),
        );
        assert!(triangle.intersects(&ray));
    }

    #[test]
    fn test_triangle_line_segment_intersects() {
        let triangle = Triangle::new(
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
        );

        // parallel
        let line = LineSegment::new(Point::new(0.0, 1.0, 0.0), Point::new(0.0, 1.0, 1.0));
        assert!(!triangle.intersects(&line));

        // in front
        let line = LineSegment::new(Point::new(0.0, 1.0, 0.0), Point::new(0.0, 4.0, 0.0));
        assert!(!triangle.intersects(&line));

        // behind
        let line = LineSegment::new(Point::new(0.0, -8.0, 0.0), Point::new(0.0, -1.0, 0.0));
        assert!(!triangle.intersects(&line));

        // past
        let line = LineSegment::new(Point::new(3.0, 1.0, 3.0), Point::new(3.0, -1.0, 3.0));
        assert!(!triangle.intersects(&line));

        // straight through
        let line = LineSegment::new(Point::new(0.0, 1.0, 0.0), Point::new(0.0, -1.0, 0.0));
        assert!(triangle.intersects(&line));

        // diagonally through
        let line = LineSegment::new(Point::new(-0.5, -2.0, 0.0), Point::new(0.5, 2.0, 0.0));
        assert!(triangle.intersects(&line));
    }
}
