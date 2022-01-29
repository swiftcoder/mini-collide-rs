use crate::{closest_point::ClosestPoint, Distance, Line, LineSegment};
use mini_math::{Point, Vector3};

/// An infinite ray.
#[derive(Debug)]
pub struct Ray {
    /// The starting point of the ray.
    pub origin: Point,
    /// The direction of the ray.
    pub direction: Vector3,
}

impl Ray {
    /// Construct a ray from a starting point and direction.
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }
}

impl Distance<Point> for Ray {
    /// Returns the distance between the ray and a given point.
    fn distance(&self, p: Point) -> f32 {
        let q = self.closest_point(&p);
        (p - q).magnitude()
    }
}

impl Distance<Ray> for Ray {
    /// Returns the distance between the ray and another ray.
    fn distance(&self, r: Ray) -> f32 {
        self.distance(r.closest_point(self))
    }
}

impl Distance<Line> for Ray {
    /// Returns the distance between the ray and a line.
    fn distance(&self, l: Line) -> f32 {
        self.distance(l.closest_point(self))
    }
}

impl Distance<LineSegment> for Ray {
    /// Returns the distance between the ray and a line segment.
    fn distance(&self, l: LineSegment) -> f32 {
        self.distance(l.closest_point(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_point() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let p = Point::new(0.0, 0.0, -5.0);
        assert_eq!(ray.distance(p), 5.0);

        let p = Point::new(0.0, 5.0, 25.0);
        assert_eq!(ray.distance(p), 5.0);
    }

    #[test]
    fn test_distance_to_ray() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let r = Ray::new(Point::new(0.0, 5.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.distance(r), 5.0);

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(r), 5.0);

        let r = Ray::new(Point::new(0.0, 5.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.distance(r), (5.0f32 * 5.0 + 5.0 * 5.0).sqrt());
    }

    #[test]
    fn test_distance_to_line() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let l = Line::new(Point::new(0.0, 5.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(ray.distance(l), 5.0);

        let l = Line::new(Point::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(l), 0.0);

        let l = Line::new(Point::new(0.0, 5.0, -5.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(ray.distance(l), 5.0);
    }

    #[test]
    fn test_distance_to_line_segment() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

        let l = LineSegment::new(Point::new(0.0, 5.0, 0.0), Point::new(0.0, 5.0, 1.0));
        assert_eq!(ray.distance(l), 5.0);

        let l = LineSegment::new(Point::new(0.0, 0.0, -5.0), Point::new(0.0, 0.0, -1.0));
        assert_eq!(ray.distance(l), 1.0);

        let l = LineSegment::new(Point::new(0.0, 5.0, -5.0), Point::new(0.0, 6.0, -5.0));
        assert_eq!(ray.distance(l), (5.0f32 * 5.0 + 5.0 * 5.0).sqrt());
    }
}
