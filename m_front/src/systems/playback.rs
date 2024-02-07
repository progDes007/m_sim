use crate::components::PlaybackControl;
use crate::components::FramesTimeline;
use crate::resources::SimInfo;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Time;

/// This system is polling frames from the incoming channel.
pub fn poll_frames(mut query: Query<&mut FramesTimeline>) {
    for mut timeline in &mut query {
        timeline.poll_frames();
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
        let hard_end = sim_info.get_total_duration();
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
