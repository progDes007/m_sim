use crate::components::FramesTimeline;
use crate::components::PlaybackControl;
use crate::resources::SimInfo;
use crate::systems;
use crate::Frame;
use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use std::sync::mpsc::Receiver;
use std::time::Duration;

pub fn run(frames_rx: Receiver<(Duration, Frame)>, total_duration: Duration) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, systems::playback::start_playback);
    app.add_systems(
        PreUpdate,
        (
            systems::playback::poll_frames,
            systems::playback::advance_time,
            systems::particles_update::particle_spawn_despawn,
        ),
    );

    // Add resources
    app.insert_resource(SimInfo::new(total_duration));

    // Spawn entity for timeline
    app.world.spawn(FramesTimeline::new(frames_rx));
    // Spawn entity for playback control
    app.world.spawn(PlaybackControl::new());

    // And run!
    app.run();
}
