use bevy::prelude::*;

use std::cmp::min;
use std::time::Duration;

/// This component stores the state of simulation playback
#[derive(Debug, Clone, Component)]
pub(crate) struct PlaybackControl {
    is_playing: bool,
    current_time: Duration,
}

impl PlaybackControl {
    pub fn new() -> Self {
        PlaybackControl {
            is_playing: false,
            current_time: Duration::new(0, 0),
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing
    }

    pub fn current_time(&self) -> Duration {
        self.current_time
    }

    pub fn set_playing(&mut self, is_playing: bool) {
        self.is_playing = is_playing;
    }

    pub fn _seek(&mut self, current_time: Duration) {
        self.current_time = current_time;
    }

    pub fn step(&mut self, time_step: Duration, soft_end: Duration, hard_end: Duration) {
        if self.is_playing() {
            self.current_time += time_step;
            let effective_end = min(soft_end, hard_end);
            self.current_time = min(self.current_time, effective_end);
            let stop = self.current_time >= hard_end;

            if stop {
                self.set_playing(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let mut playback = PlaybackControl::new();

        assert!(playback.is_playing() == false);
        assert_eq!(playback.current_time(), Duration::new(0, 0));
        // Any step while not playing does nothing
        playback.step(
            Duration::new(1, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert!(playback.is_playing() == false);

        playback.set_playing(true);
        assert!(playback.is_playing() == true);

        // Step while playing
        playback.step(
            Duration::new(1, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(1, 0));
        // And again
        playback.step(
            Duration::new(2, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(3, 0));

        // When hitting soft end, the time is clamped. But playback continues
        playback.step(
            Duration::new(10, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(10, 0));
        assert!(playback.is_playing() == true);

        // Hard end can't be reached until soft end and hard end are equal.
        // Therefore in this case - it still a soft_end case
        playback.step(
            Duration::new(10, 0),
            Duration::new(11, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(11, 0));
        assert!(playback.is_playing() == true);

        // Can continue playing, when soft end moves (i.e. more data is available)
        playback.step(
            Duration::new(10, 0),
            Duration::new(12, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(12, 0));
        assert!(playback.is_playing() == true);

        // When we reach the very end, playback stops
        playback.step(
            Duration::new(10, 0),
            Duration::new(20, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(20, 0));
        assert!(playback.is_playing() == false);
    }
}
