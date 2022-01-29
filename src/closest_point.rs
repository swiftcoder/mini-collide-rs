use mini_math::Point;

use crate::{Line, LineSegment, Ray, Sphere};

pub trait ClosestPoint<Other> {
    /// The closest point to another object
    fn closest_point(&self, other: &Other) -> Point;
}

impl ClosestPoint<Point> for Sphere {
    fn closest_point(&self, other: &Point) -> Point {
        self.center + (*other - self.center).normalized() * self.radius
    }
}

impl ClosestPoint<Sphere> for Sphere {
    fn closest_point(&self, other: &Sphere) -> Point {
        self.closest_point(&other.center)
    }
}

impl ClosestPoint<Point> for Line {
    fn closest_point(&self, other: &Point) -> Point {
        let dot = self.direction.dot(*other - self.point);
        self.point + self.direction * dot
    }
}

impl ClosestPoint<Line> for Line {
    fn closest_point(&self, other: &Line) -> Point {
        let w = self.point - other.point;
        let b = self.direction.dot(other.direction);
        let d = self.direction.dot(w);
        let e = other.direction.dot(w);
        let d_p = 1.0 - b * b;

        if d_p < std::f32::EPSILON {
            return self.point;
        }

        let sc = (b * e - d) / d_p;

        self.point + self.direction * sc
    }
}

impl ClosestPoint<Point> for Ray {
    fn closest_point(&self, other: &Point) -> Point {
        let dot = (*other - self.origin).dot(self.direction);

        if dot <= 0.0 {
            self.origin
        } else {
            self.origin + self.direction * dot
        }
    }
}

impl ClosestPoint<Line> for Ray {
    fn closest_point(&self, other: &Line) -> Point {
        let p = Line::new(self.origin, self.direction).closest_point(other);
        self.closest_point(&p)
    }
}

impl ClosestPoint<Ray> for Line {
    fn closest_point(&self, other: &Ray) -> Point {
        self.closest_point(&other.closest_point(self))
    }
}

impl ClosestPoint<LineSegment> for Ray {
    fn closest_point(&self, other: &LineSegment) -> Point {
        let p = Line::new(self.origin, self.direction)
            .closest_point(&Line::from_points(other.start, other.end));
        let p = other.closest_point(&p);
        self.closest_point(&p)
    }
}

impl ClosestPoint<Ray> for LineSegment {
    fn closest_point(&self, other: &Ray) -> Point {
        self.closest_point(&other.closest_point(self))
    }
}

impl ClosestPoint<Ray> for Ray {
    fn closest_point(&self, other: &Ray) -> Point {
        let p = Line::new(other.origin, other.direction)
            .closest_point(&Line::new(self.origin, self.direction));
        let p = other.closest_point(&p);
        self.closest_point(&p)
    }
}

impl ClosestPoint<Point> for LineSegment {
    fn closest_point(&self, other: &Point) -> Point {
        let mut direction = self.end - self.start;
        let length = direction.magnitude();
        direction /= length;

        let dot = (*other - self.start).dot(direction);

        if dot < 0.0 {
            self.start
        } else {
            self.start + direction * dot.min(length)
        }
    }
}

impl ClosestPoint<LineSegment> for LineSegment {
    fn closest_point(&self, other: &LineSegment) -> Point {
        let p = Line::from_points(other.start, other.end)
            .closest_point(&Line::from_points(self.start, self.end));
        let p = other.closest_point(&p);
        self.closest_point(&p)
    }
}

#[cfg(test)]
mod tests {
    use mini_math::Vector3;

    use super::*;

    #[test]
    fn test_line_line() {
        let line = Line::from_points(Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 10.0));

        let l = Line::from_points(Point::new(0.0, 0.0, 1.0), Point::new(0.0, 10.0, 10.0));
        assert_eq!(line.closest_point(&l), Point::new(0.0, 0.0, 1.0));

        let l = Line::from_points(Point::new(0.0, 5.0, 5.0), Point::new(0.0, 5.0, 15.0));
        assert_eq!(line.closest_point(&l), Point::new(0.0, 0.0, 0.0));

        let l = Line::from_points(Point::new(0.0, 5.0, 0.0), Point::new(25.0, 5.0, 0.0));
        assert_eq!(line.closest_point(&l), Point::new(0.0, 0.0, 0.0));

        let l = Line::from_points(Point::new(0.0, 5.0, 10.0), Point::new(25.0, 5.0, 10.0));
        assert_eq!(line.closest_point(&l), Point::new(0.0, 0.0, 10.0));
    }

    #[test]
    fn test_ray_point() {
        let ray = Ray::new(Point::zero(), Vector3::new(0.0, 0.0, 1.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(ray.closest_point(&p), Point::zero());

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(ray.closest_point(&p), Point::new(0.0, 0.0, 25.0));
    }

    #[test]
    fn test_ray_line() {
        let ray = Ray::new(Point::zero(), Vector3::new(0.0, 0.0, 1.0));

        let l = Line::new(Point::new(0.0, 5.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.closest_point(&l), Point::new(0.0, 0.0, 0.0));

        let l = Line::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(ray.closest_point(&l), Point::new(0.0, 0.0, 0.0));

        let l = Line::new(Point::new(0.0, 5.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.closest_point(&l), Point::new(0.0, 0.0, 0.0));

        let l = Line::new(Point::new(0.0, 5.0, 5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.closest_point(&l), Point::new(0.0, 0.0, 5.0));
    }
}