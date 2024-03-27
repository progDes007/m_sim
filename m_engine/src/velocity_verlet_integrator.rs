use crate::motion_resolver;
use crate::prelude::*;
use crate::{Integrator, Particle, ParticleClass, Vec2, Wall, WallClass};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct VelocityVerletIntegrator {}

impl VelocityVerletIntegrator {
    pub fn new() -> Self {
        VelocityVerletIntegrator {}
    }
}

impl Integrator for VelocityVerletIntegrator {
    fn step(
        &self,
        particles: &mut [Particle],
        particle_classes: &HashMap<ClassId, ParticleClass>,
        walls: &[Wall],
        wall_classes: &HashMap<ClassId, WallClass>,
        gravity: f64,
        time_step: Duration,
    ) {
        let time_step_sec = time_step.as_secs_f64();

        // apply gravity
        for particle in particles.iter_mut() {
            particle.velocity =
                particle.velocity - Vec2::new(0.0, gravity * time_step.as_secs_f64());
        }

        // Lamda that resolve velocity
        let particle_vs_particle_resolver =
            motion_resolver::default_particle_vs_particle_velocity_resovler(particle_classes);
        let particle_vs_wall_resolver = motion_resolver::default_particle_vs_wall_velocity_resolver(
            particle_classes,
            wall_classes,
        );

        motion_resolver::resolve(
            particles,
            particle_classes,
            walls,
            time_step_sec,
            &particle_vs_particle_resolver,
            &particle_vs_wall_resolver,
        );
    }
}
