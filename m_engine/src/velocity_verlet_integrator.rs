use crate::Integrator;
use crate::Particle;
use crate::ParticleClass;


#[derive(Debug)]
pub struct VelocityVerletIntegrator {
    
}

impl VelocityVerletIntegrator {
    pub fn new() -> Self {
        VelocityVerletIntegrator {
            
        }
    }
}

impl Integrator for VelocityVerletIntegrator {
    fn step(&mut self, particles: &mut [Particle], _particle_classes: &[ParticleClass], time_step: f64) {

        // for now just move particle based on velocity
        for particle in particles {
            particle.position += particle.velocity * time_step;
        }
    }
}