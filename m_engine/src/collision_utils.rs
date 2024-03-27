use num_traits::Float;

use crate::prelude::{DISTANCE_EPS, DOUBLE_COMPARE_EPS_STRICT};
use crate::{math_core, LineSegment};
use crate::{Plane, Polygon, Vec2};
use std::option::Option;

/// Function that calculates the collision between circle
/// and [0,0] point.
/// Returns the time of collision if any.
pub(crate) fn find_circle_vs_origin_collision(
    center1: Vec2,
    radius1: f64,
    velocity1: Vec2,
) -> Option<f64> {
    // First check if we even approaching or moving apart.
    // If we have overlap but we are moving apart - no collision.
    if velocity1.dot(center1) >= 0.0 {
        return None;
    }
    // The equation we need to solve is:
    // (cx + vx * t)^2 + (cy + vy * t)^2 = (r1)^2
    // After algebraic manipulations we get:
    // (vx^2 + vy^2)t^2 + 2(cx * vx + cy * vy)t + (cx^2 + cy^2 - r1^2) = 0
    // Which is conviniently a quadratic equation.
    let a = velocity1.x * velocity1.x + velocity1.y * velocity1.y;
    let b = 2.0 * (center1.x * velocity1.x + center1.y * velocity1.y);
    let c = center1.x * center1.x + center1.y * center1.y - radius1 * radius1;
    let t = math_core::solve_quadratic(a, b, c);
    match t {
        Some((t1, t2)) => {
            // No exra logic is needed because we handled the case of moving apart
            let t = t1.min(t2);
            // We need to filter out the tangential collision
            let res_center = center1 + velocity1 * t;
            let v1 = res_center.normalized().unwrap();
            let v2 = velocity1.normalized().unwrap();
            if v1.dot(v2).abs() < DOUBLE_COMPARE_EPS_STRICT {
                return None;
            }

            return Some(t);
        }
        None => {
            return None;
        }
    }
}

/// Function calculates collision between 2 moving particles
/// and returns the time of collision if any.
pub(crate) fn find_particle_vs_particle_collision(
    center1: Vec2,
    radius1: f64,
    velocity1: Vec2,
    center2: Vec2,
    radius2: f64,
    velocity2: Vec2,
) -> Option<f64> {
    // This problem can be reduced to collision of circle vs origin
    let radius = radius1 + radius2;
    let center = center1 - center2;
    let velocity = velocity1 - velocity2;

    return find_circle_vs_origin_collision(center, radius, velocity);
}

/// Function calculates the collision between moving point and a plane
pub(crate) fn find_point_vs_plane_collision(
    point: Vec2,
    velocity: Vec2,
    plane: Plane,
) -> Option<f64> {
    let normal = plane.normal;
    let distance = plane.distance;
    let approach_speed = -velocity.dot(normal);
    if approach_speed <= 0.0 {
        return None;
    }
    let t = (point.dot(normal) - distance) / approach_speed;
    return Some(t);
}

/// Function calculates collision between moving point and stationary line segment
pub(crate) fn find_point_vs_segment_collision(
    point: Vec2,
    velocity: Vec2,
    segment: LineSegment,
) -> Option<f64> {
    let plane = segment.plane()?;
    let t = find_point_vs_plane_collision(point, velocity, plane)?;
    // Find collision point and see if it fell into segment
    let collision_point = point + velocity * t;
    let on = (collision_point - segment.begin).dot(collision_point - segment.end) <= 0.0;
    return if on { Some(t) } else { None };
}

/// Function calculate the collision between moving particle and polygon
/// Returns time and collision normal, if any
pub(crate) fn find_particle_vs_polygon_collision(
    center: Vec2,
    radius: f64,
    velocity: Vec2,
    polygon: &Polygon,
) -> Option<(f64, Vec2)> {
    let mut result = None;
    // helper lamda that replaces collision with new one
    // if new one is earlier
    let mut take = |collision: (f64, Vec2)| {
        if let Some((t, _)) = result {
            if collision.0 < t {
                result = Some(collision);
            }
        } else {
            result = Some(collision);
        }
    };

    // Intersect with vertices
    for (vertex_index, vertex) in polygon.points.iter().enumerate() {
        // Don't collide with vertex if center is already on the internal side of the corner.
        // This can only happen if:
        // - point is outside polygon, but behind other edges. so collision is useless
        // - point is inside polygon. in which case we don't want collision
        if !polygon.is_point_outside_corner(vertex_index, center) {
            continue;
        }

        let local_center = center - *vertex;
        let t = find_circle_vs_origin_collision(local_center, radius, velocity);
        if let Some(t) = t {
            let expected_collision = local_center + velocity * t;
            let normal = particles_collision_normal(Vec2::ZERO, Vec2::ZERO, expected_collision, velocity);
            if let Some(normal) = normal {
                take((t, normal));
            }
        }
    }

    // Intersect with edges
    for edge in polygon.edges_iter() {
        let plane = edge.plane().expect("Non 0 edge is expected");
        // If particle already penetrated the edge by > radius - skip it
        // This is needed for collision stability. If we ignore all collisions in the past
        // this will cause ghosting even after tiny penetration (think numerical accuracy).
        // At the same time we don't want to accept collisions far in the past (large penetration).
        // In combination with OTHER mistakes in simulation it can lead for particle that is completely ourside of
        // polygon to teleport back through the polygon.
        // This may be overcautious, but it is most robust.
        let center_depth = -plane.distance(center);
        if center_depth >= 0.0 {
            continue;
        }

        // Reduce problem to point vs line segment.
        // Note: because we reduce it to point vs line segment there is subtle side effect
        // There won't be a collision if particle center is outside of edge, but particle perimeter is hitting.
        // However, this case will be corvered in the vertex collision check above
        let off_edge = edge.offseted(radius).expect("Failed to offset edge");
        let t_opt = find_point_vs_segment_collision(center, velocity, off_edge);
        if let Some(t) = t_opt {
            take((t, plane.normal));
        }
    }

    return result;
}

/// Calculates collision normal of 2 colliding particles.
/// Collision normal can't be calculated if centers are identical and velocities are equal.
pub(crate) fn particles_collision_normal(
    center1: Vec2,
    velocity1: Vec2,
    center2: Vec2,
    velocity2: Vec2,
) -> Option<Vec2> {
    match (center2 - center1).normalized() {
        Some(normal) => Some(normal),
        None => {
            // If center are already overlapping - consider
            // this case as collision of 2 points
            let dv = velocity2 - velocity1;
            return dv.normalized();
        }
    }
}

/// Calculates collision impulse
pub(crate) fn collision_impulse(
    mass1: f64,
    velocity1: Vec2,
    mass2: f64,
    velocity2: Vec2,
    collision_normal: Vec2,
    coefficient_of_restitution: f64,
) -> f64 {
    let relative_velocity = velocity1 - velocity2;
    let approach_velocity = -relative_velocity.dot(collision_normal);
    let impulse = approach_velocity * (1.0 + coefficient_of_restitution);
    return impulse / (1.0 / mass1 + 1.0 / mass2);
}

/// Calculate collision impulse when one of the objects is fixed (immoveable)
fn collision_impulse_stationary(
    mass1: f64,
    velocity1: Vec2,
    collision_normal: Vec2,
    coefficient_of_restitution: f64,
) -> f64 {
    let approach_velocity = -velocity1.dot(collision_normal);
    let impulse = approach_velocity * (1.0 + coefficient_of_restitution);
    return impulse * mass1;
}

/// Applies impulse to the object with given mass
pub(crate) fn apply_impulse(mass: f64, velocity: Vec2, impulse: Vec2) -> Vec2 {
    return velocity + impulse / mass;
}

/// Calculate separation velocity after collision
pub(crate) fn particles_collision_separation_velocity(
    velocity1: Vec2,
    mass1: f64,
    velocity2: Vec2,
    mass2: f64,
    collision_normal: Vec2,
    coefficient_of_restitution: f64,
) -> (Vec2, Vec2) {
    let impulse = collision_impulse(
        mass1,
        velocity1,
        mass2,
        velocity2,
        collision_normal,
        coefficient_of_restitution,
    );
    let new_velocity1 = apply_impulse(mass1, velocity1, collision_normal * impulse);
    let new_velocity2 = apply_impulse(mass2, velocity2, -collision_normal * impulse);
    return (new_velocity1, new_velocity2);
}

/// Calculate separation velocity after collision
pub(crate) fn particles_vs_wall_collision_separation_velocity(
    velocity1: Vec2,
    mass1: f64,
    collision_normal: Vec2,
    wall_temperature: f64,
    wall_heat_conductivity: f64,
) -> Vec2 {

    // If particle not moving - return nothing
    let direction_opt = velocity1.normalized();
    if direction_opt.is_none() {
        return velocity1;
    }

    // Total energy of the particle and wall
    let sampled_temperature = math_core::random_0_to_mean(wall_temperature);
    let wall_energy = math_core::energy_from_temp(sampled_temperature);
    let particle_energy = math_core::kinetic_energy_from_velocity(mass1, velocity1.length());

    // The amount of energy gained or lost depends on the collision angle
    let angle_dot = -direction_opt.unwrap().dot(collision_normal);
    // Tricky moment. The proportion of velocity that can be traded depends on angle of collision
    // But proportion of energy that can be traded depends on square of that.
    let sq_angle_dot = angle_dot * angle_dot;

    let delta_e = (wall_energy - particle_energy) * sq_angle_dot * wall_heat_conductivity;

    // First simmulate the collision with energy loss (or gain) (fully elastic)
    let impulse = collision_impulse_stationary(mass1, velocity1, collision_normal, 1.0);
    let res_v = apply_impulse(mass1, velocity1, collision_normal * impulse);
    // Then apply additional impulse based on delta_e
    // Calculate the velocity we would need to have to match the eneragy
    let expected_velocity = math_core::velocity_from_kinetic_energy(mass1, particle_energy + delta_e);
    // Now calculate what kind of dv to apply along normal to current velocity so that
    // epxected velocity magnitude is reached
    // Let's write some system of equation.
    // rv.x = x * n.x + cv.x
    // rv.y = x * n.y + cv.y
    // sqrt(rv.x^2 + rv.y^2) = expected_velocity
    // Substitute:
    // sqrt((x * n.x + cv.x)^2 + (x * n.y + cv.y)^2) = expected_velocity
    // (x * n.x + cv.x)^2 + (x * n.y + cv.y)^2 = expected_velocity^2
    // Bring it to standard quadratic equation form. go very slow:
    // x^2 * (n.x^2 + n.y^2) + 2(x * n.x * cv.x + x * n.y * cv.y) + cv.x^2 + cv.y^2 = expected_velocity^2
    // x^2 * (n.x^2 + n.y^2) + 2(x * n.x * cv.x + x * n.y * cv.y) + cv.x^2 + cv.y^2 - expected_velocity^2 = 0
    // (n.x^2 + n.y^2)x^2 + 2(n.x * cv.x + n.y * cv.y)x + (cv.x^2 + cv.y^2 - expected_velocity^2) = 0
    let a = collision_normal.x * collision_normal.x + collision_normal.y * collision_normal.y;
    let b = 2.0 * (collision_normal.x * res_v.x + collision_normal.y * res_v.y);
    let c = res_v.x * res_v.x + res_v.y * res_v.y - expected_velocity * expected_velocity;
    let dv = math_core::solve_quadratic(a, b, c).expect("No solution for dv");
    // pick dv that has smaller magnitude
    let dv = if dv.0.abs() < dv.1.abs() { dv.0 } else { dv.1 };

    let final_v = res_v + dv * collision_normal;
    // Validate that our velocity is indeed correct
    if !math_core::approx_eq(final_v.length(), expected_velocity, DISTANCE_EPS)
    {
        println!("Final velocity is not correct:");
    }
    return final_v;
}

#[cfg(test)]
mod tests {
    use crate::prelude::DOUBLE_COMPARE_EPS_STRICT;

    use super::*;

    #[test]
    fn test_find_circles_vs_origin_collision() {
        // When departing - no collision even if overlaping
        assert!(
            find_circle_vs_origin_collision(Vec2::new(0.8, 0.0), 1.0, Vec2::new(1.0, 0.0))
                .is_none()
        );

        // 0 distance
        let res = find_circle_vs_origin_collision(Vec2::new(0.0, 2.0), 2.0, Vec2::new(0.0, -2.0))
            .expect("Collision expected");
        assert!(math_core::approx_eq(res, 0.0, DOUBLE_COMPARE_EPS_STRICT));

        // When already having overlap - returns negative time
        let res = find_circle_vs_origin_collision(Vec2::new(0.5, 0.0), 1.0, Vec2::new(-2.0, 0.0))
            .expect("Collision expected");
        assert!(math_core::approx_eq(res, -0.25, DOUBLE_COMPARE_EPS_STRICT));

        // Normal approach - returns first collision
        let res = find_circle_vs_origin_collision(Vec2::new(3.0, 0.0), 1.0, Vec2::new(-2.0, 0.0))
            .expect("Collision expected");
        assert!(math_core::approx_eq(res, 1.0, DOUBLE_COMPARE_EPS_STRICT));

        // Barely hit
        let res =
            find_circle_vs_origin_collision(Vec2::new(1.0, -5.0), 1.0001, Vec2::new(0.0, 2.0))
                .expect("Collision expected");
        assert!(math_core::approx_eq(res, 2.5, 0.01)); // higher epsilon because of large error expected

        // Just miss
        assert!(
            find_circle_vs_origin_collision(Vec2::new(1.0, -5.0), 0.999, Vec2::new(0.0, 2.0))
                .is_none()
        );

        // Tangent - no collision
        let res =
            find_circle_vs_origin_collision(Vec2::new(1.0, -5.0), 1.0, Vec2::new(0.0, 2.0));
        assert!(res.is_none());

    }

    #[test]
    fn test_find_particle_vs_particle_collision() {
        // particle 2 is catching up particle 1
        let res = find_particle_vs_particle_collision(
            Vec2::new(2.0, 1.0),
            1.0,
            Vec2::new(-1.0, 0.0),
            Vec2::new(7.5, 1.0),
            1.5,
            Vec2::new(-2.5, 0.0),
        )
        .expect("Collision expected");
        // gap between particles is 3.0. catch up speed is 1.5
        assert!(math_core::approx_eq(res, 2.0, DOUBLE_COMPARE_EPS_STRICT));
    }

    #[test]
    fn test_find_point_vs_plane_collision() {
        // Point in front but moves away
        assert!(find_point_vs_plane_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Plane::new(Vec2::new(0.0, 1.0), -1.0)
        )
        .is_none());
        // Point in behind and moves away
        assert!(find_point_vs_plane_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Plane::new(Vec2::new(0.0, 1.0), 1.0)
        )
        .is_none());
        // Point is in behind and moves towards. The collision shall
        // be in the past
        let t = find_point_vs_plane_collision(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, -1.0),
            Plane::new(Vec2::new(0.0, 1.0), 1.0),
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(t, -1.0, DOUBLE_COMPARE_EPS_STRICT));

        // Point is in in front and moves towards. The collision shall
        // be in the future
        let t = find_point_vs_plane_collision(
            Vec2::new(0.0, 2.0),
            Vec2::new(0.0, -1.0),
            Plane::new(Vec2::new(0.0, 1.0), 0.0),
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(t, 2.0, DOUBLE_COMPARE_EPS_STRICT));

        // Check velocity affects time
        let t = find_point_vs_plane_collision(
            Vec2::new(0.0, 2.0),
            Vec2::new(0.0, -2.0),
            Plane::new(Vec2::new(0.0, 1.0), 0.0),
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(t, 1.0, DOUBLE_COMPARE_EPS_STRICT));

        // Check angle of approach affects time. 30 deg is convinient, since it's sin is 1/2
        let t = find_point_vs_plane_collision(
            Vec2::new(0.0, 2.0),
            Vec2::from_angle_rad(-std::f64::consts::PI / 6.0),
            Plane::new(Vec2::new(0.0, 1.0).normalized().unwrap(), 0.0),
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(t, 4.0, DOUBLE_COMPARE_EPS_STRICT));
    }

    #[test]
    fn test_find_point_vs_segment_collision() {
        let segment = LineSegment::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, 3.0));
        // Miss from begin side
        assert!(find_point_vs_segment_collision(
            Vec2::new(3.0, 0.0),
            Vec2::new(-1.0, 0.0),
            segment
        )
        .is_none());
        // RIght onto begin
        assert!(find_point_vs_segment_collision(
            Vec2::new(3.0, 1.0),
            Vec2::new(-1.0, 0.0),
            segment
        )
        .is_some());

        // Hit mid-edge
        assert!(find_point_vs_segment_collision(
            Vec2::new(3.0, 2.0),
            Vec2::new(-1.0, 0.0),
            segment
        )
        .is_some());
        // Right onto end
        assert!(find_point_vs_segment_collision(
            Vec2::new(3.0, 3.0),
            Vec2::new(-1.0, 0.0),
            segment
        )
        .is_some());
        // Miss from end side
        assert!(find_point_vs_segment_collision(
            Vec2::new(3.0, 4.0),
            Vec2::new(-1.0, 0.0),
            segment
        )
        .is_none());
    }

    #[test]
    fn test_find_particle_vs_polygon_edge_collision() {
        // Make a square
        let polygon = Polygon::new_rectangle(1.0, 1.0, 4.0, 3.0);

        // Collision from outside (moving from top down)
        let res = find_particle_vs_polygon_collision(
            Vec2::new(3.0, 6.0),
            1.0,
            Vec2::new(0.0, -2.0),
            &polygon,
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(res.0, 1.0, DOUBLE_COMPARE_EPS_STRICT));
        assert!(res
            .1
            .approx_eq(Vec2::new(0.0, 1.0), DOUBLE_COMPARE_EPS_STRICT));

        // When there is small penetration (< radius) - collision is in the past
        let res = find_particle_vs_polygon_collision(
            Vec2::new(3.0, 3.1),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon,
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(res.0, -0.2, DOUBLE_COMPARE_EPS_STRICT));
        assert!(res
            .1
            .approx_eq(Vec2::new(0.0, 1.0), DOUBLE_COMPARE_EPS_STRICT));

        // When penetration is larger than radius - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(3.0, 2.99),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon
        )
        .is_none());

        // When fully inside - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(3.0, 2.0),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon
        )
        .is_none());

        // When starting to exit, but center is still inside - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(3.0, 0.1),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon
        )
        .is_none());

        // When exiting. Center outside, but there is overlap - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(3.0, -0.2),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon
        )
        .is_none());

        // When exited completely - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(3.0, -2.2),
            0.5,
            Vec2::new(0.0, -2.0),
            &polygon
        )
        .is_none());
    }

    #[test]
    fn test_find_particle_vs_polygon_vertex_collision() {
        // Make CCW rombus for convinient math
        let polygon = Polygon::from(vec![
            Vec2::new(1.0, 0.0),
            Vec2::new(2.0, 1.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(2.0, 1.0),
        ]);

        // Hitting from outside int the right vertex
        let res = find_particle_vs_polygon_collision(
            Vec2::new(4.0, 1.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            &polygon,
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(res.0, 0.5, DOUBLE_COMPARE_EPS_STRICT));
        assert!(res
            .1
            .approx_eq(Vec2::new(1.0, 0.0), DOUBLE_COMPARE_EPS_STRICT));

        // When there is a bit of penetration - collision in the past
        let res = find_particle_vs_polygon_collision(
            Vec2::new(2.2, 1.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            &polygon,
        )
        .expect("Collision expected");
        assert!(math_core::approx_eq(res.0, -0.4, DOUBLE_COMPARE_EPS_STRICT));
        assert!(res
            .1
            .approx_eq(Vec2::new(1.0, 0.0), DOUBLE_COMPARE_EPS_STRICT));

        // When penetration is deeper (center inside) - no collision
        assert!(find_particle_vs_polygon_collision(
            Vec2::new(1.9, 1.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            &polygon
        )
        .is_none());

        // Now test the collision with vertex at an angle.
        let res = find_particle_vs_polygon_collision(
            Vec2::new(2.99, -1.0),
            1.0,
            Vec2::new(0.0, 1.0),
            &polygon,
        )
        .expect("Collision expected");
        // Barely scraping the vertex. The exact result is hard to calculate
        // But roughly:
        assert!(math_core::approx_eq(res.0, 1.8, 0.1));
        assert!(res.1.approx_eq(Vec2::new(1.0, 0.0), 0.2));
    }

    #[test]
    fn test_particles_collision_normal() {
        // Identical centers and velocities
        assert!(particles_collision_normal(
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0)
        )
        .is_none());
        // Identical centers but different velocities
        let res = particles_collision_normal(
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 2.0),
        )
        .expect("Collision expected");
        assert!(res.approx_eq(
            Vec2::new(-1.0, 2.0).normalized().unwrap(),
            DOUBLE_COMPARE_EPS_STRICT
        ));
        // Different centers and velocities don't matter
        let res = particles_collision_normal(
            Vec2::new(1.0, 1.0),
            Vec2::new(112.0, 323.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(4444.0, 332.0),
        )
        .expect("Collision expected");
        assert!(res.approx_eq(
            Vec2::new(1.0, 1.0).normalized().unwrap(),
            DOUBLE_COMPARE_EPS_STRICT
        ));
    }

    #[test]
    fn test_collision_impulse() {
        let test = |m1, v1, m2, v2, n, c, expect_v1, expect_v2| {
            let impulse = collision_impulse(m1, v1, m2, v2, n, c);
            println!("impulse: {}", impulse);
            let res1 = apply_impulse(m1, v1, n * impulse);
            let res2 = apply_impulse(m2, v2, -n * impulse);
            println!("res1: {}", res1);
            println!("res2: {}", res2);
            // Weighted summ of velocities is unchanged
            assert!((v1 * m1 + v2 * m2).approx_eq(res1 * m1 + res2 * m2, DOUBLE_COMPARE_EPS_STRICT));
            assert!(res1.approx_eq(expect_v1, DOUBLE_COMPARE_EPS_STRICT));
            assert!(res2.approx_eq(expect_v2, DOUBLE_COMPARE_EPS_STRICT));
        };
        // Head on collision case. Non elastic. Same masses
        test(
            1.0,
            Vec2::new(2.0, 0.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            Vec2::new(1.0, 0.0),
            0.0,
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
        );
        // Head on collision case. Non elastic. Differnt masses
        test(
            1.0,
            Vec2::new(2.0, 0.0),
            2.0,
            Vec2::new(-2.0, 0.0),
            Vec2::new(1.0, 0.0),
            0.0,
            Vec2::new(-2.0 / 3.0, 0.0),
            Vec2::new(-2.0 / 3.0, 0.0),
        );

        // Head on collision case.Elastic. Same masses
        test(
            1.0,
            Vec2::new(2.0, 0.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            Vec2::new(1.0, 0.0),
            1.0,
            Vec2::new(-2.0, 0.0),
            Vec2::new(2.0, 0.0),
        );

        // Head on collision case.Elastic. Different masses
        test(
            1.0,
            Vec2::new(2.0, 0.0),
            2.0,
            Vec2::new(-2.0, 0.0),
            Vec2::new(1.0, 0.0),
            1.0,
            Vec2::new(-10.0 / 3.0, 0.0), // twice the delta of non-elastic
            Vec2::new(2.0 / 3.0, 0.0),   // twice the delta of non-elastic
        );

        // Collision at an angle
        test(
            1.0,
            Vec2::new(2.0, 3.0),
            1.0,
            Vec2::new(-2.0, -2.0),
            Vec2::new(1.0, 0.0),
            1.0,
            Vec2::new(-2.0, 3.0),
            Vec2::new(2.0, -2.0),
        );
    }

    #[test]
    fn test_collision_impulse_stationary() {
        // Hitting at 45 deg
        let v = Vec2::new(0.0, 2.0);
        let n = Vec2::new(1.0, -1.0).normalized().unwrap();
        let impulse = collision_impulse_stationary(2.0, v, n, 1.0);
        // Should bounce to the right.
        let res_v = apply_impulse(2.0, v, impulse * n);
        assert!(res_v.approx_eq(Vec2::new(2.0, 0.0), DOUBLE_COMPARE_EPS_STRICT));

        // Same 45 deg hit. But with 0 coefficient of restitution
        let impulse = collision_impulse_stationary(2.0, v, n, 0.0);
        // Resulting velocity must be along wall
        let res_v = apply_impulse(2.0, v, impulse * n);
        assert!(res_v.approx_eq(Vec2::new(1.0, 1.0), DOUBLE_COMPARE_EPS_STRICT));
    }
}
