use m_front::BevyFront;
use m_front::components::Frame;
use m_front::ParticleSkin;

use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (frames_tx, frames_rx) = mpsc::channel();   
    let mut front = BevyFront::new(frames_rx, Duration::from_secs(5));
    
    // add some skins for testing
    front.define_class_skin(0,  ParticleSkin::new(10.0, bevy::prelude::Color::RED));

    // add some frames for testing
    for i in 0..=500 {
        let particles = vec![m_engine::Particle::new(
            m_engine::Vec2::new(i as f64, i as f64), 
            m_engine::Vec2::new(i as f64, i as f64), 
            0)];
        frames_tx.send((std::time::Duration::from_millis(i * 10 as u64), Frame::new(particles))).unwrap();
    }
    
    front.run();
}
