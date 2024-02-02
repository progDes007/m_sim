use std::collections::BTreeMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use m_engine::Particle;

/// Represents information about displayed frame
#[derive(Debug, Clone)]
pub struct Frame {
    pub particles: Vec<Particle>,   
}

impl Frame {
    pub fn new(particles: Vec<Particle>) -> Self {
        Frame {
            particles,
        }
    }
}

/// Buffer that stores all the calculated frames.
/// Frontend can display any of calculated frames. Frames are calculated
/// externally and sent to this buffer through a channel.
#[derive(Debug)]
pub struct FramesTimeline {
    frames: BTreeMap<Duration, Frame>,
    frames_rx: Receiver<(Duration, Frame)>,
}

impl FramesTimeline {
    pub fn new(frames_rx: Receiver<(Duration, Frame)>) -> Self {
        FramesTimeline {
            frames: BTreeMap::new(),
            frames_rx,
        }
    }

    pub fn poll_frames(&mut self) {
        while let Ok((timestamp, frame)) = self.frames_rx.try_recv() {
            self.frames.insert(timestamp, frame);
        }
    }

    pub fn num_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn last_frame(&self) -> Option<&Frame> {
        self.frames.values().last()
    }

    pub fn last_frame_for(&self, timestamp: Duration) -> Option<(Duration, &Frame)> {
        let last = self.frames.range(..=timestamp).last();
        // Get rid of reference to key
        return last.map(|(&ts, frame)| (ts, frame));
    }

    /// Time span [from, to] of all frames in the timeline. Returns None if there are
    /// no frames
    pub fn time_span(&self) -> Option<(Duration, Duration)> {
        if self.frames.is_empty() {
            return None;
        }
        let first = self.frames.keys().next().unwrap();
        let last = self.frames.keys().last().unwrap();
        Some((*first, *last))
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    use m_engine::prelude::ClassId;
    use m_engine::Particle;
    use m_engine::Vec2;
    use core::time;
    use std::sync::mpsc;
    use std::thread;

    // Helper function that makes single Frame for testing.
    pub fn make_test_frame(num_particles : usize) -> Frame {
        let mut particles = Vec::new();
        for i in 0..num_particles {
            particles.push(Particle::new(
                Vec2::new(i as f64, i as f64), 
                Vec2::new(i as f64, i as f64), 
                i as ClassId));
        }
        Frame::new(particles)
    }

    // Helper function that creates frame timeline and generate some frames.
    // For testing purposes first frame doesn't start from 0
    pub fn make_test_timeline(num_frames: usize, time_step: Duration) -> FramesTimeline {
        let (sender, receiver) = mpsc::channel();
        let mut timeline = FramesTimeline::new(receiver);

        let mut timestamp = time_step;
        for i in 0..num_frames {
            let frame = make_test_frame(i + 1);
            sender.send((timestamp, frame)).unwrap();
            timestamp += time_step;
        }
        timeline.poll_frames();
        timeline
    }

    #[test]
    fn test_async_transfer() {
        // Create a channel for sending frames
        let (sender, receiver) = mpsc::channel();

        // Create a FramesTimeline with the receiver
        let mut timeline = FramesTimeline::new(receiver);

        // Spawn a thread to simulate frame calculations and sending
        let handle = thread::spawn(move || {
            // Simulate frame calculations and sending
            for i in 0..5 {
                // Create a test frame
                let frame = make_test_frame(i + 1);

                // Simulate delay between frames
                thread::sleep(Duration::from_millis(10));

                // Send the frame through the channel
                sender.send((time::Duration::from_millis(i as u64), frame)).unwrap();
            }
        });

        // Wait for the thread to complete
        handle.join().unwrap();

        // Poll frames from the timeline
        timeline.poll_frames();

        // Assert that the timeline has received all frames
        assert_eq!(timeline.num_frames(), 5);
    }
    

    #[test]
    fn test_last_frame_for() {
        let timeline = make_test_timeline(5, Duration::from_secs(1));
        
        // Exact match
        let last_frame = timeline.last_frame_for(Duration::from_millis(3000));
        assert_eq!(last_frame.unwrap().0, Duration::from_secs(3));

        // In between frames
        let last_frame = timeline.last_frame_for(Duration::from_millis(2999));
        assert_eq!(last_frame.unwrap().0, Duration::from_millis(2000));

        // After last frame
        let last_frame = timeline.last_frame_for(Duration::from_millis(10000));
        assert_eq!(last_frame.unwrap().0, Duration::from_secs(5));

        // Before first frame (first frame is at 1 sec)
        let last_frame = timeline.last_frame_for(Duration::from_millis(500));
        assert!(last_frame.is_none());
    }

    #[test]
    fn test_time_span() {
        let timeline = make_test_timeline(5, Duration::from_secs(1));
        let time_span = timeline.time_span().unwrap();
        assert_eq!(time_span.0, Duration::from_secs(1));
        assert_eq!(time_span.1, Duration::from_secs(5));

        // Also test empty timeline
        let (_, receiver) = mpsc::channel();
        let timeline = FramesTimeline::new(receiver);
        assert!(timeline.time_span().is_none());
    }
}
