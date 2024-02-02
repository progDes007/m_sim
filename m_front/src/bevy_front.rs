use crate::ParticleSkin;
use bevy::app::App;
use bevy::DefaultPlugins;
use m_engine::prelude::*;
use std::collections::HashMap;

pub struct BevyFront {
    particle_skins: HashMap<ClassId, ParticleSkin>,
}

impl BevyFront {
    pub fn new() -> Self {
        BevyFront {
            particle_skins: HashMap::new(),
        }
    }

    pub fn define_class_skin(&mut self, class_id: ClassId, particle_skin: &ParticleSkin) {
        self.particle_skins.insert(class_id, particle_skin.clone());
    }
    
    pub fn run(&self) {
        App::new().add_plugins(DefaultPlugins).run();
    }
}
