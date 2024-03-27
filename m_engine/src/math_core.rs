

pub(crate) fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// Creates positive random number with normal distribution with given mean
pub(crate) fn random_0_to_mean(mean: f64) -> f64 {
    let mut sum = 0.0;
    for _ in 0..6 {
        sum += rand::random::<f64>() * 2.0 * mean;
    }
    return sum / 6.0;
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
pub (crate) fn kinetic_energy_from_velocity(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * velocity * velocity
}

/// Returns velocity of the particle with given mass and energy.
pub (crate) fn velocity_from_kinetic_energy(mass: f64, energy: f64) -> f64 {
    (2.0 * energy / mass).sqrt()
}

/// Converts kinetic energy to temperature
pub (crate) fn temp_from_energy(energy : f64) -> f64 {
    // assume Boltzmann constant = 1
    energy * 3.0 / 2.0
}

/// Converts temperature to kinetic energy
pub (crate) fn energy_from_temp(temp : f64) -> f64 {
    // assume Boltzmann constant = 1
    temp * 2.0 / 3.0
}