use crate::components::FramesTimeline;
use crate::components::PlaybackControl;
use crate::resources::graphic_resources::GlobalMaterials;
use crate::resources::GlobalMeshes;
use crate::resources::SimInfo;
use crate::systems;
use crate::Frame;
use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use bevy::DefaultPlugins;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use bevy::sprite::MaterialMesh2dBundle;

pub fn run(frames_rx: Receiver<(Duration, Frame)>, total_duration: Duration) {
    // Work around the known bevy bug:
    // https://github.com/bevyengine/bevy/issues/8395
    std::env::set_var("WGPU_BACKEND", "dx12");
    
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, setup);
    app.add_systems(PostStartup, systems::playback::start_playback);
    app.add_systems(
        PreUpdate,
        (
            systems::playback::poll_frames,
            systems::playback::advance_time.after(systems::playback::poll_frames),
            systems::particles_update::particle_spawn_despawn.after(systems::playback::advance_time),
        ),
    );
    app.add_systems(
        Update,
        systems::particles_update::particle_move
        
    );

    // Add resources
    app.insert_resource(SimInfo::new(total_duration));
    app.insert_resource(GlobalMeshes::new());
    app.insert_resource(GlobalMaterials::new());

    // Spawn entity for timeline
    app.world.spawn(FramesTimeline::new(frames_rx));

    // And run!
    app.run();
}

fn setup(
    mut global_mesh_res: ResMut<GlobalMeshes>,
    mut global_materials_res: ResMut<GlobalMaterials>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    // Spawn entity for playback control
    commands.spawn(PlaybackControl::new());

    // Spawn orthogonal camera
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 1.0 / 10.0;
    commands.spawn(camera_bundle);

    // Prepare global meshes
    let unit_circle_mesh = mesh_assets.add(Mesh::from(shape::Circle::new(1.0)));
    global_mesh_res.unit_circle = Some(unit_circle_mesh);

    // Prepare global materials
    let white_solid_material = material_assets.add(ColorMaterial::from(Color::WHITE));
    global_materials_res.white_solid = Some(white_solid_material);

}
