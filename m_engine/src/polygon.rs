use crate::{LineSegment, Vec2};

#[derive(Debug, Clone, PartialEq)]
/// Represents polygon made of points points. Polygon is always closed.
/// Last edge is implied
pub struct Polygon {
    pub points: Vec<Vec2>,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon { points: Vec::new() }
    }

    pub fn new_rectangle(mut x1: f64, mut y1: f64, mut x2: f64, mut y2: f64) -> Self {
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }
        let points = vec![
            Vec2::new(x1, y1),
            Vec2::new(x2, y1),
            Vec2::new(x2, y2),
            Vec2::new(x1, y2),
        ];
        return Polygon { points };
    }

    /// Returns number of edges
    pub fn num_edges(&self) -> usize {
        self.points.len()
    }

    /// Returns edge.
    pub fn edge(&self, index: usize) -> LineSegment {
        let p1 = self.points[index];
        let p2 = self.points[(index + 1) % self.points.len()];
        LineSegment::new(p1, p2)
    }

    // Returns if point is inside (or outside) of the corner formed by vertex
    // The corner is formed by extending the neighboor edges to inifinity
    pub fn is_point_outside_corner(&self, vertex_index: usize, point: Vec2) -> bool {
        let num_edges = self.num_edges();
        let edge1 = self.edge((vertex_index + num_edges - 1) % num_edges);
        let edge2 = self.edge(vertex_index);
        let plane1 = edge1.plane().expect("Edge should not be degenerate");
        let plane2 = edge2.plane().expect("Edge should not be degenerate");
        let convex = (edge1.end - edge1.begin).cross(edge2.end - edge2.begin) > 0.0;

        return if convex {
            plane1.distance(point) > 0.0 || plane2.distance(point) > 0.0
        } else {
            plane1.distance(point) > 0.0 && plane2.distance(point) > 0.0
        };
    }

    /// Gets points as flat array with 2d coordinates
    pub fn points2d_flat_iter(&self) -> impl Iterator<Item = f32> + '_ {
        self.points
            .iter()
            .flat_map(|v| vec![v.x as f32, v.y as f32])
            .into_iter()
    }

    /// Gets points as array of 3d coordinates with z=0.
    /// Each item is array in format [x, y, z]
    pub fn points3d_arrays_iter(&self) -> impl Iterator<Item = [f32; 3]> + '_ {
        self.points
            .iter()
            .map(|v| [v.x as f32, v.y as f32, 0.0])
            .into_iter()
    }

    /// Get all edges as Iterator
    pub fn edges_iter(&self) -> impl Iterator<Item = LineSegment> + '_ {
        (0..self.num_edges()).map(|index| self.edge(index).clone())
    }
}

impl From<Vec<Vec2>> for Polygon {
    fn from(points: Vec<Vec2>) -> Self {
        Polygon { points }
    }
}

impl From<&[Vec2]> for Polygon {
    fn from(points: &[Vec2]) -> Self {
        Polygon {
            points: points.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::DISTANCE_EPS;

    use super::*;

    #[test]
    fn test_point_outside_polygon_corner() {
        // L shape
        let polygon = Polygon::from(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(3.0, 1.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(2.0, 0.0),
        ]);

        // Test convex corner
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(3.1, 0.0)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(3.1, 0.1)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(3.1, -0.1)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(2.9, -0.1)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(2.9, 0.1)),
            false
        );
        assert_eq!(
            polygon.is_point_outside_corner(1, Vec2::new(2.9, 0.1)),
            false
        );

        // Test concave corner
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(2.1, 1.1)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(2.2, 1.1)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(2.1, 1.2)),
            true
        );
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(1.9, 0.9)),
            false
        );
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(2.1, 0.9)),
            false
        );
        assert_eq!(
            polygon.is_point_outside_corner(3, Vec2::new(1.9, 1.2)),
            false
        );
    }

    #[test]
    fn test_edges_iter() {
        let p0 = Vec2::new(0.0, 0.0);
        let p1 = Vec2::new(1.0, 0.0);
        let p2 = Vec2::new(1.0, 1.0);
        let polygon = Polygon::from(vec![p0, p1, p2]);
        let edges: Vec<LineSegment> = polygon.edges_iter().collect();
        assert_eq!(edges.len(), 3);
        assert!(edges[0].approx_eq(LineSegment::new(p0, p1), DISTANCE_EPS));
        assert!(edges[1].approx_eq(LineSegment::new(p1, p2), DISTANCE_EPS));
        assert!(edges[2].approx_eq(LineSegment::new(p2, p0), DISTANCE_EPS));
    }
}
