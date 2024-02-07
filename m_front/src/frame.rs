use m_engine::Particle;

/// Represents information about displayed frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub particles: Vec<Particle>,   
}

impl Frame {
    pub fn new(particles: Vec<Particle>) -> Self {
        Frame {
            particles,
        }
    }
}