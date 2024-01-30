use crate::{Particle, Vec2, ClassId};

pub fn generate_grid(
    origin: Vec2,
    primary_axis_dir: Vec2,
    size_primary: f64,
    size_secondary: f64,
    num_cells_primary: usize,
    num_cells_secondary: usize,
    initial_velocity: impl Fn(Vec2) -> Vec2,
    class_id: ClassId,
) -> Vec<Particle> {
    debug_assert!(primary_axis_dir.is_unit());
    debug_assert!(size_primary > 0.0);
    debug_assert!(size_secondary > 0.0);

    if num_cells_primary == 0 || num_cells_secondary == 0 {
        return Vec::new();
    }    
    
    let cell_size_primary = size_primary / num_cells_primary as f64;
    let cell_size_secondary = size_secondary / num_cells_secondary as f64;
    let secondary_axis_dir = primary_axis_dir.rotated_90_ccw();
    let mut particles = Vec::new();
    for i in 0..num_cells_secondary+1 {
        for j in 0..num_cells_primary+1 {
            let pos = origin + primary_axis_dir * j as f64 * cell_size_primary +
             secondary_axis_dir * i as f64 * cell_size_secondary;
            particles.push(Particle::new(
                pos, initial_velocity(pos), class_id));
        }
    }

    particles
}

pub fn constant_velocity(velocity: Vec2) -> impl Fn(Vec2) -> Vec2 {
    move |_| velocity 
}


#[cfg(test)]
mod tests {
    use crate::DISTANCE_EPS;

    use super::*;

    #[test]
    fn test_generate_grid() {
        
        let origin = Vec2::new(5.0, 4.0);
        let primary_axis_dir = Vec2::new(1.0, 0.0);
        let size_primary = 1.0;
        let size_secondary = 2.0;
        let num_cells_primary = 1;
        let num_cells_secondary = 1;
        let initial_velocity = constant_velocity(Vec2::new(1.0, 2.0));
        let class_id = 2;
        let particles = generate_grid(
            origin,
            primary_axis_dir,
            size_primary,
            size_secondary,
            num_cells_primary,
            num_cells_secondary,
            initial_velocity,
            class_id,
        );
        
        assert_eq!(particles.len(), 4);
        assert!(particles[0].position.approx_eq(Vec2::new(5.0, 4.0), DISTANCE_EPS));
        assert!(particles[1].position.approx_eq(Vec2::new(6.0, 4.0), DISTANCE_EPS));
        assert!(particles[2].position.approx_eq(Vec2::new(5.0, 6.0), DISTANCE_EPS));
        assert!(particles[3].position.approx_eq(Vec2::new(6.0, 6.0), DISTANCE_EPS));
        assert!(particles[0].velocity.approx_eq(Vec2::new(1.0, 2.0), DISTANCE_EPS));
        assert!(particles[1].velocity.approx_eq(Vec2::new(1.0, 2.0), DISTANCE_EPS));
        assert!(particles[2].velocity.approx_eq(Vec2::new(1.0, 2.0), DISTANCE_EPS));
        assert!(particles[3].velocity.approx_eq(Vec2::new(1.0, 2.0), DISTANCE_EPS));
        assert_eq!(particles[0].class(), class_id);
        assert_eq!(particles[1].class(), class_id);
        assert_eq!(particles[2].class(), class_id);
        assert_eq!(particles[3].class(), class_id);
    }
   
}