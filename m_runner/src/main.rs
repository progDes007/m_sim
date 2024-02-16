use m_front::bevy_front;
use m_front::Frame;
use m_engine::{Vec2, ParticleClass, Simulation, Integrator, VelocityVerletIntegrator };
use m_engine::generators;
//use m_front::ParticleSkin;

use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let (frames_tx, frames_rx) = mpsc::channel();   
    
    // define classes
    let mut classes = HashMap::new();
    classes.insert(1, ParticleClass::new("Class1", 1.0, 1.0));

    // make simulation
    let mut simulation = Simulation::new(classes);

    // spawn particles
    simulation.spawn_particles(&generators::generate_grid(
        Vec2::ZERO, Vec2::from_angle_rad(2.22),
        5.0, 3.0, 5, 3,
        generators::random_velocity(3.0), 1));
    
    // make integrator
    let integrator = VelocityVerletIntegrator::new();
    
    // Launch the thread that generate frames
    let handle = std::thread::spawn(move || {
        let time_step = Duration::from_millis(20);
        let mut current_time = Duration::new(0, 0);
        let classes = simulation.particle_classes().clone(); // to please borrow checker
        // Add 0 frame
        if let Err(_) = frames_tx.send((current_time.clone(), Frame::new(simulation.particles().to_vec()))) {
            return;
        }
        

        // Generate frames in a separate thread
        for _i in 0..=500 {
            // Update simulation
            integrator.step(simulation.particles_mut(), &classes, time_step);
            current_time += time_step;
            // Send frame
            if let Err(_) = frames_tx.send((current_time.clone(), Frame::new(simulation.particles().to_vec()))) {
                return;
            }

            // simulate slow work
           // std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    
    bevy_front::run(frames_rx, Duration::from_secs(5));

    handle.join().unwrap();
}
