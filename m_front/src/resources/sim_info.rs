
use crate::ParticleSkin;

use m_engine::prelude::ClassId;

use bevy::prelude::*;

use std::time::Duration;
use std::collections::HashMap;


/// This component contains general simulation information
#[derive(Debug, Clone, Resource)]
pub(crate) struct SimInfo {
    pub total_duration: Duration,
    pub skins : HashMap<ClassId, ParticleSkin>,
}

impl SimInfo{
    pub fn new(total_duration: Duration, skins : HashMap<ClassId, ParticleSkin>) -> Self {
        Self {
            total_duration,
            skins
        }
    }
}