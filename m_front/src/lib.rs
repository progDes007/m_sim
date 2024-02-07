pub mod prelude;
pub mod bevy_front;
pub mod particle_skin;
pub mod frame;

pub use particle_skin::ParticleSkin;
pub use frame::Frame;


mod systems
{
    pub(crate) mod playback;
    pub(crate) mod particles_update;
}

mod resources
{
    pub(crate) mod sim_info;

    pub(crate) use sim_info::SimInfo;
}

mod components
{
    pub(crate) mod frames_timeline;
    pub(crate) mod playback_control;
    pub(crate) mod particle;

    pub(crate) use frames_timeline::FramesTimeline;
    pub(crate) use playback_control::PlaybackControl;
    pub(crate) use particle::Particle;
}

