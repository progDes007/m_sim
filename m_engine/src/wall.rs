use crate::prelude::*;
use crate::{Polygon, Vec2};

#[derive(Clone, Debug)]
pub struct Wall {
    polygon: Polygon,
    class: ClassId,
}

impl Wall {
    pub fn new(polygon: Polygon, class: ClassId) -> Self {
        Wall { polygon, class }
    }

    pub fn make_box(
        xmin: f64,
        ymin: f64,
        xmax: f64,
        ymax: f64,
        thickness: f64,
        class: ClassId,
    ) -> Vec<Wall> {
        // Box is a shape with holes. We don't support holes. Therefore create 4 walls instead
        // All polygons ccw
        let mut walls = Vec::new();
        // Bottom
        walls.push(Wall::new(
            Polygon::new_rectangle(xmin, ymin, xmax, ymin + thickness),
            class,
        ));
        // Right
        walls.push(Wall::new(
            Polygon::new_rectangle(xmax - thickness, ymin + thickness, xmax, ymax - thickness),
            class,
        ));
        // Top
        walls.push(Wall::new(
            Polygon::new_rectangle(xmin, ymax - thickness, xmax, ymax),
            class,
        ));
        // Left
        walls.push(Wall::new(
            Polygon::new_rectangle(xmin, ymin + thickness, xmin + thickness, ymax - thickness),
            class,
        ));

        return walls;
    }

    // Makes straight wall from `from` to `to` with given width and class
    // May return None if from == to
    pub fn make_straight_wall(from: Vec2, to: Vec2, width: f64, class: ClassId) -> Option<Wall> {
        let normal = (to - from).rotated_90_cw().normalized()?;
        let half_width = width / 2.0;
        return Some(Wall::new(
            Polygon::from(vec![
                from + normal * half_width,
                to + normal * half_width,
                to - normal * half_width,
                from - normal * half_width,
            ]),
            class,
        ));
    }

    pub fn class(&self) -> ClassId {
        self.class
    }

    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }
}
