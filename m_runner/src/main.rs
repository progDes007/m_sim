use m_engine::Statistics;
use m_engine::{Integrator, SimulationSpec, VelocityVerletIntegrator};
use m_front::{bevy_front, WallSkin};
use m_front::{Frame, ParticleSkin};

use bevy::prelude::Color;

use std::collections::HashMap;
use std::env;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: m_runner <path_to_yaml>");
        return;
    }

    // Try read yaml
    let file_contents = match std::fs::read_to_string(&args[1]) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Parse yaml
    let spec_res = SimulationSpec::from_yaml(&file_contents);
    if let Err(e) = spec_res {
        println!("Error loading YAML simulation file: {}", e);
        return;
    }
    let spec = spec_res.unwrap();

    // Generate skins for particle
    let mut particle_skins = HashMap::new();
    for c in spec.particle_classes.iter() {
        let skin = ParticleSkin::new(
            c.radius as f32,
            Color::rgba(c.color.0, c.color.1, c.color.2, c.color.3),
        );
        particle_skins.insert(c.id, skin);
    }
    // Generate skins for walls
    let mut wall_skins = HashMap::new();
    for c in spec.wall_classes.iter() {
        let skin = WallSkin::new(Color::rgba(c.color.0, c.color.1, c.color.2, c.color.3));
        wall_skins.insert(c.id, skin);
    }

    // Build simulation from spec
    let mut simulation = spec.build();

    // make integrator
    let integrator = VelocityVerletIntegrator::new();

    // Channel for communicating with working thread
    let (frames_tx, frames_rx) = mpsc::channel();

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
        while current_time < spec.duration {
            // Take particles out to please borrow checker
            let mut tmp_particles = simulation.take_particles();
            // Update simulation
            integrator.step(
                &mut tmp_particles,
                simulation.particle_classes(),
                simulation.walls(),
                simulation.wall_classes(),
                simulation.gravity(),
                spec.time_step,
            );
            // Return particles back
            simulation.put_particles(tmp_particles);
            current_time += spec.time_step;

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

    bevy_front::run(
        &spec.name,
        frames_rx,
        spec.duration,
        particle_skins,
        wall_skins,
    );

    handle.join().unwrap();
}
