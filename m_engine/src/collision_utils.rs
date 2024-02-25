use crate::math_core;
use crate::Vec2;
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
    if velocity1.dot(center1) > 0.0 {
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
            return Some(t1.min(t2));
        }
        None => {
            return None;
        }
    }
}

/// Function calculates collision between 2 moving circles
/// and returns the time of collision if any.
pub(crate) fn find_circle_vs_circle_collision(
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

/// Calculates collision normal.
/// Collision normal can't be calculated if centers are identical and velocities are equal.
pub(crate) fn collision_normal(
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
    let approach_velocity = relative_velocity.dot(collision_normal);
    let impulse = approach_velocity * (1.0 + coefficient_of_restitution);
    return impulse / (1.0 / mass1 + 1.0 / mass2);
}

/// Applies impulse to the object with given mass
pub(crate) fn apply_impulse(mass: f64, velocity: Vec2, impulse: Vec2) -> Vec2 {
    return velocity + impulse / mass;
}

/// Calculate separation velocity after collision
pub(crate) fn collision_separation_velocity(
    center1: Vec2,
    velocity1: Vec2,
    mass1: f64,
    center2: Vec2,
    velocity2: Vec2,
    mass2: f64,
    coefficient_of_restitution: f64,
) -> Option<(Vec2, Vec2)> {
    let collision_normal = collision_normal(center1, velocity1, center2, velocity2)?;
    let impulse = collision_impulse(
        mass1,
        velocity1,
        mass2,
        velocity2,
        collision_normal,
        coefficient_of_restitution,
    );
    let new_velocity1 = apply_impulse(mass1, velocity1, -collision_normal * impulse);
    let new_velocity2 = apply_impulse(mass2, velocity2, collision_normal * impulse);
    return Some((new_velocity1, new_velocity2));
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
    }

    #[test]
    fn test_find_circle_vs_circle_collision() {
        // circle 2 is catching up circle 1
        let res = find_circle_vs_circle_collision(
            Vec2::new(2.0, 1.0),
            1.0,
            Vec2::new(-1.0, 0.0),
            Vec2::new(7.5, 1.0),
            1.5,
            Vec2::new(-2.5, 0.0),
        )
        .expect("Collision expected");
        // gap between circles is 3.0. catch up speed is 1.5
        assert!(math_core::approx_eq(res, 2.0, DOUBLE_COMPARE_EPS_STRICT));
    }

    #[test]
    fn test_collision_normal() {
        // Identical centers and velocities
        assert!(collision_normal(
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 1.0)
        )
        .is_none());
        // Identical centers but different velocities
        let res = collision_normal(
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
        let res = collision_normal(
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
            let res1 = apply_impulse(m1, v1, -n * impulse);
            let res2 = apply_impulse(m2, v2, n * impulse);
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
}
