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
        return Polygon {
            points
        };
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

    /// Gets points as flat array with 2d coordinates
    pub fn points2d_flat(&self) -> Vec<f32> {
        self.points.iter().flat_map(|v| vec![v.x as f32, v.y as f32]).collect()
    }

    /// Gets points as array of 3d coordinates with z=0.
    /// Each item is array in format [x, y, z]
    pub fn points3d_arrays(&self) -> Vec<[f32; 3]> {
        self.points.iter().map(|v| [v.x as f32, v.y as f32, 0.0]).collect()
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