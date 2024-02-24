use m_front::bevy_front;
use m_front::{Frame, ParticleSkin};
use m_engine::{Vec2, ParticleClass, Simulation, Integrator, VelocityVerletIntegrator };
use m_engine::generators;

use bevy::prelude::Color;

use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

static SIMULATION_LEGTH: Duration = Duration::new(10, 0);
static TIME_STEP: Duration = Duration::from_millis(20);

fn main() {
    let (frames_tx, frames_rx) = mpsc::channel();   
    
    // define classes
    let mut classes = HashMap::new();
    let mut skins = HashMap::new();
    classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));
    skins.insert(1, ParticleSkin::new(1.0, Color::RED));

    // make simulation
    let mut simulation = Simulation::new(classes);

    // spawn particles
    simulation.spawn_particles(&generators::generate_grid(
        Vec2::ZERO, Vec2::from_angle_rad(2.22),
        30.0, 30.0, 12, 12,
        generators::random_velocity(60.0), 1));
    
    // make integrator
    let integrator = VelocityVerletIntegrator::new();
    
    // Launch the thread that generate frames
    let handle = std::thread::spawn(move || {
        let mut current_time = Duration::new(0, 0);
        let classes = simulation.particle_classes().clone(); // to please borrow checker
        // Add 0 frame
        if let Err(_) = frames_tx.send((current_time.clone(), Frame::new(simulation.particles().to_vec()))) {
            return;
        }
        

        // Generate frames in a separate thread
        while current_time < SIMULATION_LEGTH{
            // Update simulation
            integrator.step(simulation.particles_mut(), &classes, TIME_STEP);
            current_time += TIME_STEP;
            // Send frame
            if let Err(_) = frames_tx.send((current_time.clone(), Frame::new(simulation.particles().to_vec()))) {
                return;
            }

            // simulate slow work
            //std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
    
    bevy_front::run(frames_rx, SIMULATION_LEGTH, skins);

    handle.join().unwrap();
}
