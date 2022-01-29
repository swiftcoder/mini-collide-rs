use mini_math::{Point, Vector3};

use crate::{Capsule, ClosestPoint, Line, LineSegment, Plane, Ray, Sphere};

/// Trait for finding the distance between two objects
pub trait Distance<Other> {
    /// The distance between two objects
    fn distance(&self, other: &Other) -> f32;
}

impl Distance<Point> for Line {
    fn distance(&self, p: &Point) -> f32 {
        let cross = self.direction.cross(*p - self.point);
        cross.magnitude()
    }
}

impl Distance<Line> for Line {
    fn distance(&self, line: &Line) -> f32 {
        let w = self.point - line.point;
        let b = self.direction.dot(line.direction);
        let d = self.direction.dot(w);
        let e = line.direction.dot(w);
        let d_p = 1.0 - b * b;

        let (sc, tc) = if d_p < std::f32::EPSILON {
            (0.0, if b > 1.0 { d / b } else { e })
        } else {
            ((b * e - d) / d_p, (e - b * d) / d_p)
        };

        let p = w + (self.direction * sc) - (line.direction * tc);
        p.magnitude()
    }
}

impl Distance<Point> for LineSegment {
    fn distance(&self, p: &Point) -> f32 {
        let q = self.closest_point(p);

        (*p - q).magnitude()
    }
}

impl Distance<LineSegment> for LineSegment {
    fn distance(&self, l: &LineSegment) -> f32 {
        self.distance(&l.closest_point(self))
    }
}

impl Distance<Point> for Ray {
    fn distance(&self, p: &Point) -> f32 {
        let q = self.closest_point(p);
        (*p - q).magnitude()
    }
}

impl Distance<Ray> for Ray {
    fn distance(&self, r: &Ray) -> f32 {
        self.distance(&r.closest_point(self))
    }
}

impl Distance<Line> for Ray {
    fn distance(&self, l: &Line) -> f32 {
        self.distance(&l.closest_point(self))
    }
}

impl Distance<LineSegment> for Ray {
    fn distance(&self, l: &LineSegment) -> f32 {
        self.distance(&l.closest_point(self))
    }
}

impl Distance<Point> for Plane {
    fn distance(&self, p: &Point) -> f32 {
        self.normal.dot(Vector3::from(*p)) - self.d
    }
}

impl Distance<Point> for Sphere {
    fn distance(&self, p: &Point) -> f32 {
        (*p - self.center).magnitude() - self.radius
    }
}

impl Distance<Point> for Capsule {
    fn distance(&self, p: &Point) -> f32 {
        self.axis.distance(p) - self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_point() {
        let line = Line::from_points(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(line.distance(&p), 0.0);

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(line.distance(&p), 5.0);
    }

    #[test]
    fn test_line_line() {
        let line = Line::from_points(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let l = Line::from_points(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 10.0, 10.0));
        assert_eq!(line.distance(&l), 0.0);

        let l = Line::from_points(Point::new(0.0, 5.0, 5.0), Point::new(0.0, 5.0, 15.0));
        assert_eq!(line.distance(&l), 5.0);

        let l = Line::from_points(Point::new(0.0, 5.0, 0.0), Point::new(25.0, 5.0, 0.0));
        assert_eq!(line.distance(&l), 5.0);
    }

    #[test]
    fn test_ray_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(ray.distance(&p), 5.0);

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(ray.distance(&p), 5.0);
    }

    #[test]
    fn test_ray_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let r = Ray::new(Point::new(0.0, 5.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.distance(&r), 5.0);

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(&r), 5.0);

        let r = Ray::new(Point::new(0.0, 5.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.distance(&r), (5.0f32 * 5.0 + 5.0 * 5.0).sqrt());
    }

    #[test]
    fn test_ray_line() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let l = Line::new(Point::new(0.0, 5.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.distance(&l), 5.0);

        let l = Line::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(&l), 0.0);

        let l = Line::new(Point::new(0.0, 5.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.distance(&l), 5.0);
    }

    #[test]
    fn test_ray_line_segment() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let l = LineSegment::new(Point::new(0.0, 5.0, 0.0), Point::new(0.0, 5.0, 1.0));
        assert_eq!(ray.distance(&l), 5.0);

        let l = LineSegment::new(Point::new(0.0, 0.0, -5.0), Point::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(&l), 1.0);

        let l = LineSegment::new(Point::new(0.0, 5.0, -5.0), Point::new(0.0, 6.0, -5.0));
        assert_eq!(ray.distance(&l), (5.0f32 * 5.0 + 5.0 * 5.0).sqrt());
    }

    #[test]
    fn test_line_segment_point() {
        let line = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(line.distance(&p), 5.0);

        let p = Point::new(0.0, 0.0, 15.0);
        assert_eq!(line.distance(&p), 5.0);

        let p = Point::new(0.0, 5.0, 5.0);
        assert_eq!(line.distance(&p), 5.0);
    }

    #[test]
    fn test_line_segment_line_segment() {
        let line = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let l = LineSegment::new(Point::new(0.0, 0.0, 15.0), Point::new(0.0, 0.0, 20.0));
        assert_eq!(line.distance(&l), 5.0);

        let l = LineSegment::new(Point::new(0.0, 7.0, 5.0), Point::new(0.0, 7.0, 20.0));
        assert_eq!(line.distance(&l), 7.0);

        let l = LineSegment::new(Point::new(9.0, 0.0, 0.0), Point::new(9.0, 7.0, 0.0));
        assert_eq!(line.distance(&l), 9.0);

        let l = LineSegment::new(Point::new(9.0, 1.0, -9.0), Point::new(9.0, 7.0, -9.0));
        assert_eq!(
            line.distance(&l),
            (9.0f32 * 9.0 + 9.0 * 9.0 + 1.0 * 1.0).sqrt()
        );

        let l = LineSegment::new(Point::new(0.0, 0.0, -10.0), Point::new(0.0, 0.0, -1.0));
        assert_eq!(line.distance(&l), 1.0);
    }

    #[test]
    fn test_sphere_point() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 5.0);

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(sphere.distance(&p), 0.0);

        let p = Point::new(0.0, 0.0, 15.0);
        assert_eq!(sphere.distance(&p), 10.0);
    }

    #[test]
    fn test_capsule_point() {
        let cap = Capsule::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 5.0, 0.0), 1.0);

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(cap.distance(&p), 4.0);

        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(cap.distance(&p), 4.0);
    }

    #[test]
    fn test_plane_point() {
        let plane = Plane::from_points(
            Point::new(-1.0, 0.0, -1.0),
            Point::new(1.0, 0.0, -1.0),
            Point::new(0.0, 0.0, 1.0),
        );

        let p = Point::new(3.0, 1.0, 2.0);
        assert_eq!(plane.distance(&p), 1.0);

        let p = Point::new(-2.0, -1.0, -3.0);
        assert_eq!(plane.distance(&p), -1.0);
    }
}
