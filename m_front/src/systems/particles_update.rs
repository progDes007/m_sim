use crate::components::{FramesTimeline, Particle, PlaybackControl};
use crate::resources::{GlobalMaterials, GlobalMeshes};

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub fn particle_spawn_despawn(
    current_particles: Query<Entity, With<Particle>>,
    playback_control: Query<&PlaybackControl>,
    timeline: Query<&FramesTimeline>,
    global_mesh_res: ResMut<GlobalMeshes>,
    global_materials_res: ResMut<GlobalMaterials>,
    mut commands: Commands,
) {
    // Get current time
    let current_time = playback_control.single().current_time();
    // Get current frame
    let current_frame = timeline.single().last_frame_for(current_time);

    // Required number of particles
    let required_count = match current_frame {
        Some((_, frame)) => frame.particles.len(),
        None => 0,
    };

    // Get current count of particles
    let current_count = current_particles.iter().count();
    // println!("Current count: {}, required count: {}", current_count, required_count);

    // Spawn or despawn particles based on the required count
    if current_count < required_count {
        let spawn_count = required_count - current_count;
        let mut particles = vec![];
        for _ in 0..spawn_count {
            particles.push((
                Particle::new(),
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle::from(global_mesh_res.unit_circle.clone().unwrap()),
                    material: global_materials_res.white_solid.clone().unwrap(),
                    ..MaterialMesh2dBundle::default()
                },
            ));
        }
        commands.spawn_batch(particles);
    } else if current_count > required_count {
        let despawn_count = current_count - required_count;
        for entity in current_particles.iter().take(despawn_count) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn particle_move(
    mut transforms: Query<&mut Transform, With<Particle>>,
    playback_control: Query<&PlaybackControl>,
    timeline: Query<&FramesTimeline>,
) {
        // Get current time
        let current_time = playback_control.single().current_time();
        // Get current frame
        let current_frame_opt = timeline.single().last_frame_for(current_time);
        if current_frame_opt.is_none() { return };
        let current_frame = current_frame_opt.as_ref().unwrap().1;
        // Check that spawn despawn worked as expected
        assert_eq!(current_frame.particles.len(), transforms.iter().count());
        // Now loop and copy positions
        for (i, mut transform) in transforms.iter_mut().enumerate() {
            let particle = &current_frame.particles[i];
            *transform = Transform::from_translation(Vec3::new(
                particle.position.x as f32, particle.position.y as f32, 0.0));

        }
}