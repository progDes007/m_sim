use crate::components::{FramesTimeline, Particle, PlaybackControl};
use crate::resources::SkinGraphics;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

/// This system spawns or despawns particles based on number of particles this frame
pub fn particle_spawn_despawn(
    current_particles: Query<Entity, With<Particle>>,
    playback_control: Query<&PlaybackControl>,
    timeline: Query<&FramesTimeline>,
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
                MaterialMesh2dBundle::<ColorMaterial>::default(),
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

/// This system moves particles and update it's class
pub fn particle_update(
    mut query: Query<(&mut Transform, &mut Particle)>,
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
        assert_eq!(current_frame.particles.len(), query.iter().count());
        // Now loop and copy positions and particle class
        for (i, (mut transform, mut dst_particle)) in query.iter_mut().enumerate() {
            let src_particle = &current_frame.particles[i];
            *transform = Transform::from_translation(Vec3::new(
                src_particle.position.x as f32, src_particle.position.y as f32, 0.0));
            dst_particle.class = src_particle.class();

        }
}

/// This system updates particles skin based on the class
pub fn update_skins(
    mut query: Query<(&mut Mesh2dHandle, &mut Handle<ColorMaterial>, &Particle)>,
    skins: Res<SkinGraphics>) {
    
    for (mut mesh, mut material, particle) in query.iter_mut() {
        *mesh = skins.meshes.get(&particle.class).unwrap().clone().into();
        *material = skins.materials.get(&particle.class).unwrap().clone();
    }
}