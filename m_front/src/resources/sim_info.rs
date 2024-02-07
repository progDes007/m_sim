use bevy::prelude::*;

use std::time::Duration;

/// This component contains general simulation information
#[derive(Debug, Clone, Resource)]
pub(crate) struct SimInfo {
    total_duration: Duration,
}

impl SimInfo{
    pub fn new(total_duration: Duration) -> Self {
        Self {
            total_duration,
        }
    }

    pub fn get_total_duration(&self) -> Duration {
        self.total_duration
    }
}