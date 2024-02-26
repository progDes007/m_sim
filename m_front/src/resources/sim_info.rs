use crate::{ParticleSkin, WallSkin};

use m_engine::prelude::ClassId;

use bevy::prelude::*;

use std::collections::HashMap;
use std::time::Duration;

/// This component contains general simulation information
#[derive(Debug, Clone, Resource)]
pub(crate) struct SimInfo {
    pub total_duration: Duration,
    pub particle_skins: HashMap<ClassId, ParticleSkin>,
    pub wall_skins: HashMap<ClassId, WallSkin>,
}

impl SimInfo {
    pub fn new(
        total_duration: Duration,
        particle_skins: HashMap<ClassId, ParticleSkin>,
        wall_skins: HashMap<ClassId, WallSkin>,
    ) -> Self {
        Self {
            total_duration,
            particle_skins,
            wall_skins,
        }
    }
}
