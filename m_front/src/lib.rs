pub mod prelude;
pub mod bevy_front;
pub mod skins;
pub mod frame;

pub use skins::{ParticleSkin, WallSkin};
pub use frame::Frame;

mod utils;
mod systems
{
    pub(crate) mod playback;
    pub(crate) mod particles_update;
    pub(crate) mod walls_update;
    pub(crate) mod statistics_update;
}

mod resources
{
    pub(crate) mod sim_info;
    pub(crate) mod graphic_resources;

    pub(crate) use sim_info::SimInfo;
    pub(crate) use graphic_resources::GlobalMeshes;
    pub(crate) use graphic_resources::GlobalMaterials;
    pub(crate) use graphic_resources::SkinGraphics;
    pub(crate) use graphic_resources::TextStyles;
}

mod components
{
    pub(crate) mod frames_timeline;
    pub(crate) mod playback_control;
    pub(crate) mod objects;
    pub(crate) mod statistics;

    pub(crate) use frames_timeline::FramesTimeline;
    pub(crate) use playback_control::{PlaybackControl, TimeIndicator};
    pub(crate) use objects::Particle;
    pub(crate) use objects::Wall;
    pub(crate) use statistics::StatisticsReport;
}

