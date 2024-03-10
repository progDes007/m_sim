use bevy::prelude::*;

use std::cmp::min;
use std::time::Duration;

/// This component stores the state of simulation playback
#[derive(Debug, Clone, Component)]
pub(crate) struct PlaybackControl {
    is_playing: bool,
    rewind: Option<f64>,
    current_time: Duration,
}

impl PlaybackControl {
    pub fn new() -> Self {
        PlaybackControl {
            is_playing: false,
            rewind: None,
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

    pub fn set_rewind(&mut self, rewind: Option<f64>) {
        self.rewind = rewind;
    }

    pub fn _seek(&mut self, current_time: Duration) {
        self.current_time = current_time;
    }

    pub fn step(&mut self, time_step: Duration, soft_end: Duration, hard_end: Duration) {
        // Select the update speed and direction
        let mult = if let Some(rewind) = self.rewind {
            rewind
        } else {
            if self.is_playing() {
                1.0
            } else {
                0.0
            }
        };

        if mult < 0.0 {
            self.current_time = self.current_time.saturating_sub(time_step.mul_f64(-mult));
        } else if mult > 0.0 {
            self.current_time += time_step.mul_f64(mult);
        }
        let effective_end = min(soft_end, hard_end);
        self.current_time = min(self.current_time, effective_end);
        let stop = self.current_time >= hard_end;

        if stop {
            self.set_playing(false);
        }
    }
}

/// This component is marker for the time indicator text
#[derive(Debug, Clone, Component)]
pub(crate) struct TimeIndicator;

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
        // Rewind back to 1
        playback.rewind = Some(-2.0);
        playback.step(
            Duration::new(1, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(1, 0));
        // Rewind back a lot. Clamps to 0
        playback.rewind = Some(-2.0);
        playback.step(
            Duration::new(10, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        assert_eq!(playback.current_time(), Duration::new(0, 0));
        // Fast forward to 3
        playback.rewind = Some(1.0);
        playback.step(
            Duration::new(3, 0),
            Duration::new(10, 0),
            Duration::new(20, 0),
        );
        playback.rewind = None;

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
