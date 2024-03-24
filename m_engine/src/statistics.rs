use crate::math_core;
use crate::prelude::*;
use crate::{Particle, ParticleClass};
use std::collections::HashMap;
use std::fmt;
use statrs::statistics;

#[derive(Debug, Clone)]
pub struct Statistics {
    pub num_particles: usize,
    pub total_energy: f64,
    pub temperature: f64,
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            num_particles: 0,
            total_energy: 0.0,
            temperature: 0.0,
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
        let energies : Vec<f64> = particles.iter().map(get_energy).collect();
        let temps : Vec<f64> = energies.iter().map(|&e| math_core::temp_from_energy(e)).collect();
        // Calc mean and variance
        res.temperature = statistics::Statistics::mean(&temps);
        res.total_energy = energies.iter().sum();

        return res;
    }

    pub fn to_strings(&self) -> Vec<String> {
        vec![
            format!("Number of particles: {}", self.num_particles),
            format!("Total energy: {}", self.total_energy),
            format!("Temperature: {} simuK", self.temperature),
            // Add more strings as needed
        ]
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings = self.to_strings();
        write!(f, "{}", strings.join(", "))
    }
}