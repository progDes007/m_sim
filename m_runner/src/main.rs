use m_engine::{generators, Statistics, Wall, WallClass};
use m_engine::{Integrator, ParticleClass, Simulation, Vec2, VelocityVerletIntegrator};
use m_front::{bevy_front, WallSkin};
use m_front::{Frame, ParticleSkin};

use bevy::prelude::Color;

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;

static SIMULATION_LEGTH: Duration = Duration::new(60, 0);
static TIME_STEP: Duration = Duration::from_millis(20);

fn main() {
    let (frames_tx, frames_rx) = mpsc::channel();

    // define classes
    let mut particle_classes = HashMap::new();
    let mut particle_skins = HashMap::new();
    particle_classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
    particle_skins.insert(1, ParticleSkin::new(1.0, Color::RED));

    let mut wall_classes = HashMap::new();
    wall_classes.insert(1, WallClass::new("Wall", 1.0));
    let mut wall_skins = HashMap::new();
    wall_skins.insert(1, WallSkin::new(Color::WHITE));

    // make simulation
    let mut simulation = Simulation::new(particle_classes, wall_classes);

    // spawn particles
    simulation.spawn_particles(&generators::generate_grid(
        Vec2::new(-20.0, -20.0),
        Vec2::from_angle_rad(0.0),
        30.0,
        30.0,
        12,
        12,
        generators::random_velocity(30.0),
        1,
    ));

    // spawn walls
    simulation.spawn_walls(&Wall::make_box(-50.0, -50.0, 50.0, 50.0, 1.0, 1));

    // make integrator
    let integrator = VelocityVerletIntegrator::new();

    // Launch the thread that generate frames
    let handle = std::thread::spawn(move || {
        let mut current_time = Duration::new(0, 0);
        // Add 0 frame
        if let Err(_) = frames_tx.send((
            current_time.clone(),
            Frame::new(
                simulation.particles().to_vec(),
                simulation.walls().to_vec(),
                Statistics::build(&simulation.particles(), simulation.particle_classes()),
            ),
        )) {
            return;
        }

        // Generate frames in a separate thread
        while current_time < SIMULATION_LEGTH {
            // Take particles out to please borrow checker
            let mut tmp_particles = simulation.take_particles();
            // Update simulation
            integrator.step(
                &mut tmp_particles,
                simulation.particle_classes(),
                simulation.walls(),
                simulation.wall_classes(),
                TIME_STEP,
            );
            // Return particles back
            simulation.put_particles(tmp_particles);
            current_time += TIME_STEP;

            // Calc statistics
            let statistics =
                Statistics::build(simulation.particles(), simulation.particle_classes());

            // Send frame
            if let Err(_) = frames_tx.send((
                current_time.clone(),
                Frame::new(
                    simulation.particles().to_vec(),
                    simulation.walls().to_vec(),
                    statistics,
                ),
            )) {
                return;
            }

            // simulate slow work
            //std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    bevy_front::run(frames_rx, SIMULATION_LEGTH, particle_skins, wall_skins);

    handle.join().unwrap();
}
