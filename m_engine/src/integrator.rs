use crate::Particle;
use crate::ParticleClass;

pub trait Integrator {
    fn step(&mut self, particles: &mut [Particle], particle_classes: &[ParticleClass], time_step: f64);
}