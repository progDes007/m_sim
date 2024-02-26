use bevy::prelude::Component;
use m_engine::prelude::ClassId;

#[derive(Debug, Clone, Component)]
pub(crate) struct Particle {
    pub class : ClassId
}

impl Particle {
    pub fn new() -> Self {
        Particle {
            class: 0
        }
    }
}


#[derive(Debug, Clone, Component)]
pub(crate) struct Wall {
}

impl Wall {
    pub fn new() -> Self {
        Wall {
        }
    }
}
