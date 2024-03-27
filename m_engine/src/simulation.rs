use crate::prelude::*;
use crate::{Particle, ParticleClass, Wall, WallClass};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Simulation {
    particle_classes: HashMap<ClassId, ParticleClass>,
    particles: Vec<Particle>,
    wall_classes: HashMap<ClassId, WallClass>,
    walls: Vec<Wall>,
    gravity: f64
}

impl Simulation {
    pub fn new(
        particle_classes: HashMap<ClassId, ParticleClass>,
        wall_classes: HashMap<ClassId, WallClass>,
        gravity: f64
    ) -> Self {
        Simulation {
            particle_classes,
            particles: Vec::new(),
            wall_classes,
            walls: Vec::new(),
            gravity,
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

    pub fn take_particles(&mut self) -> Vec<Particle> {
        std::mem::take(&mut self.particles)
    }

    pub fn put_particles(&mut self, particles: Vec<Particle>) {
        self.particles = particles;
    }

    pub fn wall_classes(&self) -> &HashMap<ClassId, WallClass> {
        &self.wall_classes
    }

    pub fn walls(&self) -> &[Wall] {
        &self.walls
    }

    pub fn gravity(&self) -> f64 {
        self.gravity
    }

    pub fn spawn_particle(&mut self, particle: Particle) {
        assert!(self.particle_classes.contains_key(&particle.class()));
        self.particles.push(particle);
    }

    pub fn spawn_particles(&mut self, particles: &[Particle]) {
        assert!(particles
            .iter()
            .all(|p| self.particle_classes.contains_key(&p.class())));
        self.particles.extend_from_slice(particles);
    }

    pub fn spawn_wall(&mut self, wall: Wall) {
        assert!(self.wall_classes.contains_key(&wall.class()));
        self.walls.push(wall);
    }

    pub fn spawn_walls(&mut self, walls: &[Wall]) {
        assert!(walls
            .iter()
            .all(|w| self.wall_classes.contains_key(&w.class())));
        self.walls.extend_from_slice(walls);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Polygon, Vec2};

    #[test]
    fn test_spawn_particles() {
        let mut classes = HashMap::new();
        classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
        classes.insert(20, ParticleClass::new("Class20", 2.0, 1.0));

        let mut simulation = Simulation::new(classes, HashMap::new(), 0.0);

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

    #[test]
    fn test_spawn_walls() {
        let mut classes = HashMap::new();
        classes.insert(1, WallClass::new("Class1", 100.0, 0.0));
        classes.insert(20, WallClass::new("Class20", 110.0, 1.0));

        let polygon = Polygon::new_rectangle(0.0, 0.0, 1.0, 1.0);

        let mut simulation = Simulation::new(HashMap::new(), classes, 0.0);

        // Spawn single
        simulation.spawn_wall(Wall::new(polygon.clone(), 1));
        assert_eq!(simulation.walls().len(), 1);

        // Spawn few more. Put them into vector first
        let walls = vec![
            Wall::new(polygon.clone(), 20),
            Wall::new(polygon.clone(), 1),
        ];
        simulation.spawn_walls(&walls);
        assert_eq!(simulation.walls().len(), 3);
        assert_eq!(simulation.walls()[0].class(), 1);
        assert_eq!(simulation.walls()[1].class(), 20);
        assert_eq!(simulation.walls()[2].class(), 1);
    }
}
