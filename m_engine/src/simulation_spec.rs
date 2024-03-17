use crate::{prelude::*, Vec2};
use crate::{ParticleClass, Simulation, Wall, WallClass};
use crate::generators;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub struct RGBA(pub f32, pub f32, pub f32, pub f32);

/// Describes the specification for a particle class
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ParticleClassSpec {
    pub id: ClassId,
    pub name: String,
    pub mass: f64,
    pub radius: f64,
    pub color: RGBA,
}

/// Describes specification for wall class
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct WallClassSpec {
    pub id: ClassId,
    pub name: String,
    pub coefficient_of_restitution: f64,
    pub color: RGBA,
}

/// Describes spawning of grid of particles
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SpawnParticlesGrid {
    pub class_id: ClassId,
    pub origin_x: f64,
    pub origin_y: f64,
    pub x_axis_angle: f64,
    pub dim_x: f64,
    pub dim_y: f64,
    pub num_cells_x: usize,
    pub num_cells_y: usize,
    pub mean_speed: f64,
}

/// Describes spawning of single wall
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SpawnStraightWall {
    class_id: ClassId,
    from_x: f64,
    from_y: f64,
    to_x: f64,
    to_y: f64,
    width: f64,
}

/// Describes the specification for the simulation scene that ought to be created
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SimulationSpec {
    pub name: String,
    pub duration: Duration,
    pub time_step: Duration,
    pub particle_classes: Vec<ParticleClassSpec>,
    pub wall_classes: Vec<WallClassSpec>,
    pub particle_grids: Vec<SpawnParticlesGrid>,
    pub straight_walls: Vec<SpawnStraightWall>,
}

impl Default for SimulationSpec {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
            duration: Duration::from_secs(10),
            time_step: Duration::from_millis(10),
            particle_classes: Vec::new(),
            wall_classes: Vec::new(),
            particle_grids: Vec::new(),
            straight_walls: Vec::new(),
        }
    }
}

impl SimulationSpec {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }

    pub fn build(&self) -> Simulation {
        // Make particle classes map
        let mut p_classes = HashMap::new();
        for class in &self.particle_classes {
            let p_class = ParticleClass::new(&class.name, class.mass, class.radius);
            p_classes.insert(class.id, p_class);
        }
        // Make wall classes map
        let mut w_classes = HashMap::new();
        for class in &self.wall_classes {
            let w_class = WallClass::new(&class.name, class.coefficient_of_restitution);
            w_classes.insert(class.id, w_class);
        }

        let mut sim = Simulation::new(p_classes, w_classes);
        // Spawn grids
        for grid in &self.particle_grids {
            sim.spawn_particles(&generators::generate_grid(
                Vec2::new(grid.origin_x, grid.origin_y),
                Vec2::from_angle_rad(grid.x_axis_angle.to_radians()),
                grid.dim_x,
                grid.dim_y,
                grid.num_cells_x,
                grid.num_cells_y,
                generators::random_velocity(grid.mean_speed),
                grid.class_id,
            ));
        }
        // Spawn walls
        for wall in &self.straight_walls {
            let new_w = Wall::make_straight_wall(
                Vec2::new(wall.from_x, wall.from_y),
                Vec2::new(wall.to_x, wall.to_y),
                wall.width,
                wall.class_id,
            );
            if let Some(new_w) = new_w {
                sim.spawn_wall(new_w);
            }
        }
        return sim;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize() {
        let spec = SimulationSpec {
            name: "Test".to_string(),
            duration: Duration::from_millis(10100),
            time_step: Duration::from_millis(10),
            particle_classes: vec![
                ParticleClassSpec {
                    id: 0,
                    name: "test".to_string(),
                    mass: 2.0,
                    radius: 1.0,
                    color: RGBA(1.0, 0.9, 0.8, 0.7),
                },
                ParticleClassSpec {
                    id: 1,
                    name: "test2".to_string(),
                    mass: 1.0,
                    radius: 2.0,
                    color: RGBA(0.7, 0.8, 0.9, 1.0),
                },
            ],
            wall_classes: vec![
                WallClassSpec {
                    id: 0,
                    name: "wall".to_string(),
                    coefficient_of_restitution: 0.5,
                    color: RGBA(0.5, 0.5, 0.5, 0.5),
                },
                WallClassSpec {
                    id: 1,
                    name: "wall2".to_string(),
                    coefficient_of_restitution: 0.6,
                    color: RGBA(0.6, 0.6, 0.6, 0.6),
                },
            ],
            particle_grids: vec![SpawnParticlesGrid {
                class_id: 0,
                origin_x: 2.0,
                origin_y: 3.0,
                x_axis_angle: 45.0,
                dim_x: 10.0,
                dim_y: 10.0,
                num_cells_x: 10,
                num_cells_y: 10,
                mean_speed: 30.0,
            }],
            straight_walls: vec![SpawnStraightWall {
                class_id: 0,
                from_x: 4.0,
                from_y: 5.0,
                to_x: 10.0,
                to_y: 0.0,
                width: 0.1,
            }],
        };
        let yaml = serde_yaml::to_string(&spec).unwrap();
        let spec2 = SimulationSpec::from_yaml(&yaml).unwrap();
        assert_eq!(spec, spec2);
    }
}
