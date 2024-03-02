use crate::prelude::*;
use crate::{collision_utils, motion_resolver};
use crate::{Integrator, Particle, ParticleClass};
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
        time_step: Duration,
    ) {
        let time_step_sec = time_step.as_secs_f64();

        // Lamda that resolve velocity
        let resolve_velocity = |p1: &Particle, p2: &Particle| {
            let velocities = collision_utils::particles_collision_separation_velocity(
                p1.position,
                p1.velocity,
                particle_classes.get(&p1.class()).unwrap().mass(),
                p2.position,
                p2.velocity,
                particle_classes.get(&p2.class()).unwrap().mass(),
                1.0,
            );
            return velocities.unwrap();
        };

        motion_resolver::resolve(particles, particle_classes, time_step_sec, resolve_velocity);
    }
}
