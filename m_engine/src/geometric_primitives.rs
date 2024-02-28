use crate::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Plane
{
    /// Plane normal
    pub normal: Vec2,
    /// Plan distance from origin. Positive distance move away from origin
    pub distance: f64,
}

impl Plane
{
    pub fn new(normal: Vec2, distance: f64) -> Self
    {
        Plane { normal, distance }
    }
}