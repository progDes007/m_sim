use crate::FramesTimeline;
use bevy::prelude::Query;

/// This system is polling frames from the incoming channel.
pub fn poll_frames(mut query: Query<&mut FramesTimeline>) {
    for mut timeline in &mut query {
        timeline.poll_frames();
    }
}