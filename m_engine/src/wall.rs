use crate::prelude::*;
use crate::Polygon;

#[derive(Clone, Debug)]
pub struct Wall {
    polygon: Polygon,
    class: ClassId,
}

impl Wall {
    pub fn new(polygon: Polygon, class: ClassId) -> Self {
        Wall {
            polygon,
            class,
        }
    }

    pub fn class(&self) -> ClassId {
        self.class
    }

    pub fn polygon(&self) -> &Polygon {
        &self.polygon
    }
}
