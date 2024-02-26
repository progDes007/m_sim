use m_engine::{Particle, Wall};

/// Represents information about displayed frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub particles: Vec<Particle>,
    pub walls: Vec<Wall>,
}

impl Frame {
    pub fn new(particles: Vec<Particle>, walls: Vec<Wall>) -> Self {
        Frame { particles, walls }
    }
}
