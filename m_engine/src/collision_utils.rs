use crate::math_core;
use crate::Vec2;
use std::option::Option;

/// Function that calculates the collision between circle
/// and [0,0] point.
/// Returns the time of collision if any.
pub fn find_circle_vs_origin_collision(
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
}
