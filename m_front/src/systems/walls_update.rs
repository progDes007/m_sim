use crate::components::{FramesTimeline, PlaybackControl, Wall};
use crate::resources::SkinGraphics;
use crate::utils;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

/// This system spawns or despawns walls based on number of walls in the frame
pub fn wall_spawn_despawn(
    current_walls: Query<Entity, With<Wall>>,
    playback_control: Query<&PlaybackControl>,
    timeline: Query<&FramesTimeline>,
    skins: Res<SkinGraphics>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    
    // Get current time
    let current_time = playback_control.single().current_time();
    // Get current frame
    let current_frame = timeline.single().last_frame_for(current_time);

    // Required number of particles
    let required_count = match current_frame {
        Some((_, frame)) => frame.walls.len(),
        None => 0,
    };

    // Get current count of particles
    let current_count = current_walls.iter().count();

    if current_count == required_count {
        return;
    }

    // Walls won't change often. Therefore we don't need optimized partial update.
    // Just despawn all walls and spawn new ones.
    for entity in current_walls.iter() {
        commands.entity(entity).despawn();
    }

    // Spawn new walls
    let src_walls = &current_frame.unwrap().1.walls;
    for src_wall in src_walls.iter() {
        let mesh = utils::create_mesh(&src_wall.polygon());
        commands.spawn((Wall::new(), MaterialMesh2dBundle {
            material: skins.wall_materials[&src_wall.class()].clone(),
            mesh: Mesh2dHandle(meshes.add(mesh)),
            ..Default::default()
        }));
    }
}
