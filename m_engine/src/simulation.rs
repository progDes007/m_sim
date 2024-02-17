use crate::prelude::*;
use crate::{ParticleClass, Particle};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Simulation {
    particle_classes: HashMap<ClassId, ParticleClass>,
    particles: Vec<Particle>,
}

impl Simulation {
    pub fn new(particle_classes: HashMap<ClassId, ParticleClass>) -> Self {
        Simulation {
            particle_classes,
            particles: Vec::new(),
        }
    }

    pub fn particle_classes(&self) -> &HashMap<ClassId, ParticleClass> {
        &self.particle_classes
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn particles_mut(&mut self) -> &mut [Particle] {
        &mut self.particles
    }

    pub fn spawn_particle(&mut self, particle: Particle) {
        assert!(self.particle_classes.contains_key(&particle.class()));
        self.particles.push(particle);
    }
    
    pub fn spawn_particles(&mut self, particles: &[Particle]) {
        assert!(particles.iter().all(|p| self.particle_classes.contains_key(&p.class())));
        self.particles.extend_from_slice(particles);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Vec2;
    
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_spawn_particles() {
            let mut classes = HashMap::new();
            classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
            classes.insert(20, ParticleClass::new("Class20", 2.0, 1.0));

            let mut simulation = Simulation::new(classes);

            // Spawn single
            simulation.spawn_particle(Particle::new(Vec2::ZERO, Vec2::ZERO, 1));
            assert_eq!(simulation.particles().len(), 1);
           
            // Spawn few more. Put them into vector first
            let particles = vec![
                Particle::new(Vec2::ZERO, Vec2::ZERO, 20),
                Particle::new(Vec2::ZERO, Vec2::ZERO, 1),
            ];
            simulation.spawn_particles(&particles);
            assert_eq!(simulation.particles().len(), 3);
            assert_eq!(simulation.particles()[0].class(), 1);
            assert_eq!(simulation.particles()[1].class(), 20);
            assert_eq!(simulation.particles()[2].class(), 1);
        }

      
    }
}