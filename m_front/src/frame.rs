use m_engine::{Particle, Statistics, Wall};

/// Represents information about displayed frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub particles: Vec<Particle>,
    pub walls: Vec<Wall>,
    pub statistics: Statistics,
}

impl Frame {
    pub fn new(particles: Vec<Particle>, walls: Vec<Wall>, statistics: Statistics) -> Self {
        Frame {
            particles,
            walls,
            statistics,
        }
    }
}
