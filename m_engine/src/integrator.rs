use crate::prelude::*;
use crate::Particle;
use crate::ParticleClass;
use std::collections::HashMap;
use std::time::Duration;

pub trait Integrator {
    fn step(&self, particles: &mut [Particle], particle_classes: &HashMap<ClassId, ParticleClass>, time_step: Duration);
}