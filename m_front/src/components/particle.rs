use bevy::prelude::Component;

#[derive(Debug, Clone, Component)]
pub(crate) struct Particle {}

impl Particle {
    pub fn new() -> Self {
        Particle {}
    }
}
