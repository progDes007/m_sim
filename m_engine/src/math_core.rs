

pub(crate) fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Solves quadratic equation.
/// Returns the roots of the equation if any.
pub(crate) fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b + sqrt_discriminant) / (2.0 * a);
    let t2 = (-b - sqrt_discriminant) / (2.0 * a);
    Some((t1, t2))
}

/// Returns energy of the particle with given mass and velocity.
pub (crate) fn kinetic_energy(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * velocity * velocity
}

/// Converts kinetic energy to temperature
pub (crate) fn temp_from_energy(energy : f64) -> f64 {
    // assume Boltzmann constant = 1
    energy * 3.0 / 2.0
}