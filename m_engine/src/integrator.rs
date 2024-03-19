use crate::prelude::*;
use crate::{Particle, ParticleClass, Wall, WallClass};
use std::collections::HashMap;
use std::time::Duration;

pub trait Integrator {
    fn step(
        &self,
        particles: &mut [Particle],
        particle_classes: &HashMap<ClassId, ParticleClass>,
        walls: &[Wall],
        wall_classes: &HashMap<ClassId, WallClass>,
        gravity: f64,
        time_step: Duration,
    );
}
