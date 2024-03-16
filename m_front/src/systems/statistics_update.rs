use bevy::prelude::*;
use crate::components::{StatisticsReport, PlaybackControl, FramesTimeline};


// System that updates statistics text
pub fn update_statistics(
    mut query: Query<(&StatisticsReport, &mut Text)>,
    playback_query: Query<&PlaybackControl>,
    timeline_query: Query<&FramesTimeline>,
) {

    // Get current time
    let current_time = playback_query.single().current_time();
    // Get current frame
    let current_frame_opt = timeline_query.single().last_frame_for(current_time);
    if current_frame_opt.is_none() { return };

    let mut text = query.single_mut().1;
    // Combine all statistics into one string
    let combined = current_frame_opt.unwrap().1.statistics.to_strings().join("\n");
    text.sections[0].value = combined;

}
