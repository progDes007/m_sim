use crate::ParticleSkin;
use crate::components::FramesTimeline;
use crate::components::PlaybackControl;
use crate::components::SimInfo;
use crate::systems;
use bevy::app::App;
use bevy::prelude::*;
use bevy::DefaultPlugins;
use m_engine::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

pub struct BevyFront {
    particle_skins: HashMap<ClassId, ParticleSkin>,
    app: App,
}

impl BevyFront {
    pub fn new(timeline: FramesTimeline, total_duration : Duration) -> Self {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);

        app.add_systems(Startup, systems::playback::start_playback);
        app.add_systems(PreUpdate,
             (systems::playback::poll_frames, systems::playback::advance_time));
        
        // Spawn entity for timeline
        app.world.spawn(timeline);
        // Spawn entity for sim info
        app.world.spawn(SimInfo::new(total_duration));
        // Spawn entity for playback control
        app.world.spawn(PlaybackControl::new());

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
