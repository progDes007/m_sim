pub mod prelude;
pub mod vec2;
pub mod particle;
pub mod particle_class;
pub mod wall;
pub mod wall_class;
pub mod integrator;
pub mod velocity_verlet_integrator;
pub mod generators;
pub mod simulation;
pub mod polygon;
pub mod geometric_primitives;

mod collision_utils;
mod motion_resolver;
mod math_core;

pub use vec2::Vec2;
pub use particle::Particle;
pub use particle_class::ParticleClass;
pub use wall::Wall;
pub use wall_class::WallClass;
pub use simulation::Simulation;
pub use integrator::Integrator;
pub use velocity_verlet_integrator::VelocityVerletIntegrator;
pub use polygon::Polygon;
pub use geometric_primitives::{Plane, LineSegment};