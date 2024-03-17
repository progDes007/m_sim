use m_engine::Statistics;
use m_engine::{Integrator, VelocityVerletIntegrator, SimulationSpec};
use m_front::{bevy_front, WallSkin};
use m_front::{Frame, ParticleSkin};

use bevy::prelude::Color;

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Duration;


fn main() {

    let (frames_tx, frames_rx) = mpsc::channel();

    let test_yaml = 
    "
    name: Test
    duration:
      secs: 30
      nanos: 0
    time_step:
      secs: 0
      nanos: 10000000
    particle_classes:
    - id: 0
      name: test
      mass: 2.0
      radius: 1.0
      color:
      - 1.0
      - 0.9
      - 0.8
      - 1.0
    wall_classes:
    - id: 0
      name: wall
      coefficient_of_restitution: 0.9
      color:
      - 0.5
      - 0.5
      - 0.5
      - 1.0
    particle_grids:
    - class_id: 0
      origin_x: -20.0
      origin_y: -20.0
      x_axis_angle: 0.0
      dim_x: 40.0
      dim_y: 40.0
      num_cells_x: 10
      num_cells_y: 10
      mean_speed: 30.0
    straight_walls:
    - class_id: 0
      from_x: -30.0
      from_y: -30.0
      to_x: 30.0
      to_y: -30.0
      width: 0.2
    - class_id: 0
      from_x: 30.0
      from_y: -30.0
      to_x: 30.0
      to_y: 30.0
      width: 0.2
    - class_id: 0
      from_x: 30.0
      from_y: 30.0
      to_x: -30.0
      to_y: 30.0
      width: 0.2
    - class_id: 0
      from_x: -30.0
      from_y: 30.0
      to_x: -30.0
      to_y: -30.0
      width: 0.2
    ";

    let spec_res = SimulationSpec::from_yaml(test_yaml);
    if let Err(e) = spec_res {
        println!("Error loading YAML simulation file: {}", e);
        return;
    }
    let spec = spec_res.unwrap();

    // Generate skins for particle
    let mut particle_skins = HashMap::new();
    for c in spec.particle_classes.iter() {
        let skin = ParticleSkin::new(c.radius as f32, 
            Color::rgba(c.color.0, c.color.1, c.color.2, c.color.3));
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

    bevy_front::run(frames_rx, spec.duration, particle_skins, wall_skins);

    handle.join().unwrap();
}
