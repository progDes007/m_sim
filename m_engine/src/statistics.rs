use crate::math_core;
use crate::prelude::*;
use crate::{Particle, ParticleClass};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Statistics {
    pub num_particles: usize,
    pub total_energy: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            num_particles: 0,
            total_energy: 0.0,
        }
    }
}

impl Statistics {
    pub fn build(
        particles: &[Particle],
        particle_classes: &HashMap<ClassId, ParticleClass>,
    ) -> Self {
        let mut res = Self::default();
        res.num_particles = particles.len();

        let get_energy = |p: &Particle| {
            let class = particle_classes.get(&p.class()).expect("Particle class expected in the map");
            return math_core::kinetic_energy(class.mass(), p.velocity.length());
        };
        res.total_energy = particles.iter().map(get_energy).sum();
        return res;
    }
}
