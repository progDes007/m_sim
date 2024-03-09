use crate::prelude::*;
use crate::{collision_utils, motion_resolver};
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
        time_step: Duration,
    ) {
        let time_step_sec = time_step.as_secs_f64();

        // Lamda that resolve velocity
        let particle_vs_particle_resolver =
            |p1: &Particle, p2: &Particle, collision_normal: Vec2| {
                return collision_utils::particles_collision_separation_velocity(
                    p1.velocity,
                    particle_classes.get(&p1.class()).unwrap().mass(),
                    p2.velocity,
                    particle_classes.get(&p2.class()).unwrap().mass(),
                    collision_normal,
                    1.0,
                );
            };
        let particle_vs_wall_resolver = |p: &Particle, w: &Wall, collision_normal: Vec2| {
            let c = wall_classes
                .get(&w.class())
                .unwrap()
                .coefficient_of_restitution();
            return collision_utils::particles_vs_wall_collision_separation_velocity(
                p.velocity,
                collision_normal,
                c,
            );
        };

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
