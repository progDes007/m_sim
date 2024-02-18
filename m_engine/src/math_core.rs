

pub fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Solves quadratic equation.
/// Returns the roots of the equation if any.
pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b + sqrt_discriminant) / (2.0 * a);
    let t2 = (-b - sqrt_discriminant) / (2.0 * a);
    Some((t1, t2))
}