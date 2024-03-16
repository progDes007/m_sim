use crate::components::{FramesTimeline, PlaybackControl, StatisticsReport, TimeIndicator};
use crate::resources::{GlobalMaterials, GlobalMeshes, SimInfo, SkinGraphics, TextStyles};
use crate::systems;
use crate::{Frame, ParticleSkin, WallSkin};
use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use bevy::DefaultPlugins;
use m_engine::prelude::*;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn run(
    frames_rx: Receiver<(Duration, Frame)>,
    total_duration: Duration,
    particle_skins: HashMap<ClassId, ParticleSkin>,
    wall_skins: HashMap<ClassId, WallSkin>,
) {
    // Work around the known bevy bug:
    // https://github.com/bevyengine/bevy/issues/8395
    std::env::set_var("WGPU_BACKEND", "dx12");

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, (setup, generate_skin_graphics));
    app.add_systems(PostStartup, systems::playback::start_playback);
    app.add_systems(
        PreUpdate,
        (
            systems::playback::poll_frames,
            systems::playback::read_user_input,
            systems::playback::advance_time
                .after(systems::playback::poll_frames)
                .after(systems::playback::read_user_input),
            systems::playback::update_time_indicator.after(systems::playback::advance_time),
            systems::statistics_update::update_statistics.after(systems::playback::advance_time),
            systems::particles_update::particle_spawn_despawn
                .after(systems::playback::advance_time),
            systems::walls_update::wall_spawn_despawn.after(systems::playback::advance_time),
        ),
    );
    app.add_systems(
        Update,
        (
            systems::particles_update::particle_update,
            systems::particles_update::update_skins
                .after(systems::particles_update::particle_update),
        ),
    );

    // Add resources
    app.insert_resource(SimInfo::new(total_duration, particle_skins, wall_skins));
    app.insert_resource(GlobalMeshes::new());
    app.insert_resource(GlobalMaterials::new());
    app.insert_resource(SkinGraphics::new());
    app.insert_resource(TextStyles::new());

    // Spawn entity for timeline
    app.world.spawn(FramesTimeline::new(frames_rx));

    // And run!
    app.run();
}

/// This system sets up the initial state of the app
fn setup(
    mut global_mesh_res: ResMut<GlobalMeshes>,
    mut global_materials_res: ResMut<GlobalMaterials>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
    text_styles: Res<TextStyles>,
    mut commands: Commands,
) {
    // Spawn orthogonal camera
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 1.0 / 5.0;
    commands.spawn(camera_bundle);

    // Spawn entity for playback control
    commands.spawn(PlaybackControl::new());

    // Spawn time indicator text
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section("Time: 0:00.000", text_styles.main_style.clone())
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..default()
            }),
        TimeIndicator {},
    ));

    // Spawn entity for statistics report
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section("Statistics", text_styles.main_style.clone())
            .with_text_alignment(TextAlignment::Left)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            }),
        StatisticsReport {},
    ));

    // Spawn text for instructions
    commands.spawn(
        TextBundle::from_section(
            "Controls: [Space] - play/pause, [Left]/[Right] - rewind/forward",
            text_styles.main_style.clone(),
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
    );

    // Prepare global meshes
    let unit_circle_mesh = mesh_assets.add(Mesh::from(shape::Circle::new(1.0)));
    global_mesh_res.unit_circle = Some(unit_circle_mesh);

    // Prepare global materials
    let white_solid_material = material_assets.add(ColorMaterial::from(Color::WHITE));
    global_materials_res.white_solid = Some(white_solid_material);
}

/// This system generates graphics for all skins
fn generate_skin_graphics(
    sim_info: Res<SimInfo>,
    mut skin_graphics_res: ResMut<SkinGraphics>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<ColorMaterial>>,
) {
    // Generate graphics for particles
    for (class_id, skin) in sim_info.particle_skins.iter() {
        let mesh = mesh_assets.add(Mesh::from(shape::Circle::new(skin.radius())));
        let material = material_assets.add(ColorMaterial::from(skin.color()));
        skin_graphics_res.particle_meshes.insert(*class_id, mesh);
        skin_graphics_res
            .particle_materials
            .insert(*class_id, material);
    }
    // Generate graphics for walls
    for (class_id, skin) in sim_info.wall_skins.iter() {
        let material = material_assets.add(ColorMaterial::from(skin.color()));
        skin_graphics_res.wall_materials.insert(*class_id, material);
    }
}
