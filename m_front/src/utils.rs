use m_engine::Polygon;
use bevy::render::mesh::{Mesh, PrimitiveTopology};

use earcutr::earcut;

/// Triangulates the polygon. Returns vertices and indices
pub(crate) fn triangulate_polygon(polygon: &Polygon) -> Vec<usize> {
    // Map polygon points to different format
    let vertices: Vec<f32> = polygon.points2d_flat();
    let res = earcut(&vertices, &[], 2);
    return match res {
        Ok(indices) => indices,
        Err(e) => {
            println!("Error triangulating polygon: {:?}", e);
            vec![]
        }
    };
}

/// Creates mesh from polygn

pub(crate) fn create_mesh(polygon: &Polygon) -> Mesh {
    let indices = triangulate_polygon(polygon);
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, polygon.points3d_arrays());
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(
        indices.into_iter().map(|i| i as u32).collect())));
    return mesh;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use m_engine::Vec2;

    #[test]
    fn test_triangulate_polygon()
    {
        let polygon = Polygon::from(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);

        let indices = triangulate_polygon(&polygon);
        assert_eq!(indices.len(), 6);
        // There are multiple valid triangulations for square
        // so we can't check the exact indices
    }
}