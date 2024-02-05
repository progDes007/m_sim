use crate::FramesTimeline;
use crate::ParticleSkin;
use crate::playback_control::PlaybackControl;
use crate::sim_info::SimInfo;
use crate::sys_playback;
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
        
        app.add_systems(Startup, sys_playback::start_playback);
        app.add_systems(PreUpdate,
             (sys_playback::poll_frames, sys_playback::advance_time));
        
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
