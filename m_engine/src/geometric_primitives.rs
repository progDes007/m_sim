use crate::Vec2;
use crate::math_core;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane
{
    /// Plane normal
    pub normal: Vec2,
    /// Plan distance from origin. Positive distance move away from origin
    pub distance: f64,
}

impl Plane
{
    pub fn new(normal: Vec2, distance: f64) -> Self
    {
        Plane { normal, distance }
    }

    pub fn approx_eq(&self, other: Self, dir_epsilon: f64, dist_epsilon: f64) -> bool {
        self.normal.approx_eq(other.normal, dir_epsilon) 
            && math_core::approx_eq(self.distance, other.distance, dist_epsilon)
    }

    pub fn distance(&self, point: Vec2) -> f64
    {
        self.normal.dot(point) - self.distance
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LineSegment
{
    pub begin: Vec2,
    pub end: Vec2,
}

impl LineSegment
{
    pub fn new(begin: Vec2, end: Vec2) -> Self
    {
        LineSegment { begin, end }
    }

    pub fn length(&self) -> f64
    {
        (self.end - self.begin).length()
    }

    pub fn direction(&self) -> Option<Vec2>
    {
        (self.end - self.begin).normalized()
    }

    pub fn normal(&self) -> Option<Vec2>
    {
        self.direction().map(|v| v.rotated_90_cw())
    }

    pub fn plane(&self) -> Option<Plane>
    {
        self.normal().map(|normal| Plane::new(normal, normal.dot(self.begin)))
    }

    /// Returns offseted version of this edge. Positive value offsets along normal
    pub fn offseted(&self, offset: f64) -> Option<Self>
    {
        let normal = self.normal()?;
        return Some(LineSegment::new(self.begin + normal * offset, self.end + normal * offset));
    }

    pub fn approx_eq(&self, other: Self, epsilon: f64) -> bool
    {
        self.begin.approx_eq(other.begin, epsilon) && self.end.approx_eq(other.end, epsilon)
    }
}


#[cfg(test)]
mod tests
{
    use crate::prelude::*;
    use super::*;

    #[test]
    fn test_plane_distance() {
        let plane = Plane::new(Vec2::new(0.0, 1.0), 2.0);
        assert!(math_core::approx_eq(plane.distance(Vec2::new(0.0, 2.0)), 0.0, DISTANCE_EPS));
        assert!(math_core::approx_eq(plane.distance(Vec2::new(100.0, 3.0)), 1.0, DISTANCE_EPS));
        assert!(math_core::approx_eq(plane.distance(Vec2::new(-100.0, 1.0)), -1.0, DISTANCE_EPS));
    }
    #[test]
    fn test_line_segment_plane()
    {
        let line = LineSegment::new(Vec2::new(1.0, 2.0), Vec2::new(2.0, 2.0));
        let plane = line.plane().unwrap();
        assert!(plane.approx_eq(Plane::new(Vec2::new(0.0, -1.0), -2.0),
            DOUBLE_COMPARE_EPS_STRICT, DISTANCE_EPS));
    
        // Funny angle. Use distance to point to verify
        let line = LineSegment::new(Vec2::new(1.0, 2.0), Vec2::new(2.0, 3.0));
        let plane = line.plane().unwrap();
        assert!(math_core::approx_eq(plane.distance(line.begin), 0.0, DISTANCE_EPS));
        assert!(math_core::approx_eq(plane.distance(line.end), 0.0, DISTANCE_EPS));

        // Zero line has no plane
        let line = LineSegment::new(Vec2::new(1.0, 2.0), Vec2::new(1.0, 2.0));
        assert!(line.plane().is_none());
    }

    #[test]
    fn test_line_segment_offseted()
    {
        // 0 line
        let line = LineSegment::new(Vec2::new(1.0, 2.0), Vec2::new(1.0, 2.0));
        assert!(line.offseted(1.0).is_none());

        // normal line
        let line = LineSegment::new(Vec2::new(1.0, 2.0), Vec2::new(2.0, 2.0));
        let offseted = line.offseted(1.0).unwrap();
        assert!(offseted.approx_eq(LineSegment::new(Vec2::new(1.0, 1.0), Vec2::new(2.0, 1.0)), DISTANCE_EPS));

        
    }
}