use crate::FramesTimeline;
use crate::ParticleSkin;
use crate::sys_frames;
use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use m_engine::prelude::*;
use std::collections::HashMap;

pub struct BevyFront {
    particle_skins: HashMap<ClassId, ParticleSkin>,
    app: App,
}

impl BevyFront {
    pub fn new(timeline: FramesTimeline) -> Self {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_systems(Update, (sys_frames::poll_frames, ));
        // Spawn entity for timeline
        app.world.spawn(timeline);

        BevyFront {
            particle_skins: HashMap::new(),
            app,
        }
    }

    pub fn define_class_skin(&mut self, class_id: ClassId, particle_skin: ParticleSkin) {
        self.particle_skins.insert(class_id, particle_skin);
    }

    pub fn run(&mut self) {
        self.app.run();
    }


}
