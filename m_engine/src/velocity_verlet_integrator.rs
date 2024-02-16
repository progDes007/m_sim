use crate::prelude::*;
use crate::Integrator;
use crate::Particle;
use crate::ParticleClass;
use std::collections::HashMap;
use std::time::Duration;


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
    fn step(&self, particles: &mut [Particle],  _particle_classes: &HashMap<ClassId, ParticleClass>, time_step: Duration) {
        let time_step_sec = time_step.as_secs_f64();
        // for now just move particle based on velocity
        for particle in particles {
            particle.position += particle.velocity * time_step_sec;
        }
    }
}