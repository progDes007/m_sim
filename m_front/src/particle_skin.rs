use bevy::prelude::Color;
use m_engine::ParticleClass;

#[derive(Clone, Debug)] // no Copy, since I expect this class to grow into something more complex
pub struct ParticleSkin {
    pub radius: f32,
    pub color: Color,
}

impl ParticleSkin {
    pub fn new(radius: f32, color: Color) -> Self {
        ParticleSkin {
            radius,
            color,
        }
    }

    pub fn from( particle_class: &ParticleClass, color : &Color ) -> Self {
        ParticleSkin {
            radius: particle_class.radius() as f32,
            color: *color,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn color(&self) -> Color {
        self.color
    }
}