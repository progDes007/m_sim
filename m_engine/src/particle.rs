use std::fmt::Debug;
use crate::Vec2;
use crate::ClassId;

#[derive(Copy, Clone, Debug)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    class: ClassId,
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, class: ClassId) -> Self {
        Particle {
            position,
            velocity,
            class,
        }
    }

    pub fn class(&self) -> ClassId {
        self.class
    }
}
