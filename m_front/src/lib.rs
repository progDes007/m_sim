pub mod prelude;
pub mod bevy_front;
pub mod particle_skin;
pub mod frame;

pub use bevy_front::BevyFront;
pub use particle_skin::ParticleSkin;
pub use frame::Frame;


mod systems
{
    pub mod playback;
}

pub mod components
{
    pub(crate) mod frames_timeline;
    pub(crate) mod playback_control;
    pub(crate) mod sim_info;

    pub(crate) use frames_timeline::FramesTimeline;
    pub(crate) use playback_control::PlaybackControl;
    pub(crate) use sim_info::SimInfo;
}

