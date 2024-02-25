use crate::Vec2;

#[derive(Debug, Clone, PartialEq)]
/// Represents polygon made of points points. Polygon is always closed.
/// Last edge is implied
pub struct Polygon {
    pub points : Vec<Vec2>
}

impl Polygon {
    pub fn new() -> Self {
        Polygon {
            points: Vec::new()
        }
    }

    /// Returns number of edges
    pub fn num_edges(&self) -> usize {
        self.points.len()
    }

    /// Returns edge. 
    pub fn edge(&self, index: usize) -> (Vec2, Vec2) {
        let p1 = self.points[index];
        let p2 = self.points[(index + 1) % self.points.len()];
        (p1, p2)
    }
}

impl From<Vec<Vec2>> for Polygon {
    fn from(points: Vec<Vec2>) -> Self {
        Polygon {
            points
        }
    }
}

impl From<&[Vec2]> for Polygon {
    fn from(points: &[Vec2]) -> Self {
        Polygon {
            points: points.to_vec()
        }
    }
}