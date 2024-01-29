#[derive(Debug, Clone)]
pub struct ParticleClass {
    name: String,
    mass: f64,
    radius: f64,
}

impl ParticleClass {
    // Constructor
    pub fn new(name: String, mass: f64, radius: f64) -> Self {
        ParticleClass {
            name,
            mass,
            radius,
        }
    }
    
    // Getters
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
