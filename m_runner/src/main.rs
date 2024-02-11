use m_front::bevy_front;
use m_front::Frame;
//use m_front::ParticleSkin;

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (frames_tx, frames_rx) = mpsc::channel();   
    
    // Launch the thread that generate frames
    std::thread::spawn(move || {
        // Generate frames in a separate thread
        for i in 0..=500 {
            let mut particles = Vec::new();
            for j in 0..10 {
                let x = ((i as f64) / 10.0).sin() * 10.0;
                let y = ((j as f64) / 10.0).cos() * 10.0;
                particles.push(m_engine::Particle::new(
                    m_engine::Vec2::new(x, y),
                    m_engine::Vec2::new(0.0, 0.0),
                    0,
                ));
            }
            frames_tx
                .send((std::time::Duration::from_millis(i * 10 as u64), Frame::new(particles)))
                .unwrap();
            // simulate slow work
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    });
    
    bevy_front::run(frames_rx, Duration::from_secs(5));
}
