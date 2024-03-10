use crate::components::FramesTimeline;
use crate::components::PlaybackControl;
use crate::resources::SimInfo;
use bevy::prelude::*;

/// This system is polling frames from the incoming channel.
pub fn poll_frames(mut query: Query<&mut FramesTimeline>) {
    for mut timeline in &mut query {
        timeline.poll_frames();
    }
}

/// Reads the keyboard input and sets playback parameters
pub fn read_user_input(
    mut playback_query: Query<&mut PlaybackControl>,
    input: Res<Input<KeyCode>>,
) {
    let mut playback_control = playback_query.single_mut();
    if input.just_pressed(KeyCode::Space) {
        let p = playback_control.is_playing();
        playback_control.set_playing(!p);
    }

    if input.pressed(KeyCode::Right) {
        playback_control.set_rewind(Some(2.0));
    }
    else if input.pressed(KeyCode::Left) {
        playback_control.set_rewind(Some(-2.0));
    }
    else {
        playback_control.set_rewind(None);
    }
}

/// Advances the playback time
pub fn advance_time(
    time: Res<Time>,
    mut playback_query: Query<&mut PlaybackControl>,
    sim_info: Res<SimInfo>,
    timeline_query: Query<&FramesTimeline>,
) {
    // get passed time
    let time_passed = std::time::Duration::from_secs_f64(time.delta_seconds_f64());

    // Collect sim info, timeline and playback control
    let mut playback_control = playback_query.single_mut();
    let timeline = timeline_query.single();

    // Update only if there are any frames in timeline
    if let Some((last_frame_timestamp, _)) = timeline.last_frame() {
        // last frame timetamp is soft stop.
        let soft_end = last_frame_timestamp;
        // Hard stop is total simulation length
        let hard_end = sim_info.total_duration;
        // Step time is time passed since last frame
        playback_control.step(time_passed, soft_end, hard_end);

        // println!("Current time: {:?}", playback_control.current_time());
    }
}

// System that start playback
pub fn start_playback(mut playback_query: Query<&mut PlaybackControl>) {
    for mut playback_control in &mut playback_query {
        playback_control.set_playing(true);
    }
}
