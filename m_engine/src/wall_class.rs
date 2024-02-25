#[derive(Debug, Clone)]
/// Wall class. Describes the properties of the wall.
pub struct WallClass {
    name: String,
    coefficient_of_restitution: f64,
}

impl WallClass {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the wall.
    /// * `coefficient_of_restitution` - The coefficient of restitution for the wall.
    /// if coefficient_of_restitution > 1.0 - the wall acts as source of energy
    /// if coefficient_of_restitution < 1.0 - the wall acts as sink of energy
    pub fn new(name: &str, coefficient_of_restitution: f64) -> Self {
        WallClass {
            name: name.to_string(),
            coefficient_of_restitution,
        }
    }

    /// Get the name of the wall.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the coefficient of restitution for the wall.
    pub fn coefficient_of_restitution(&self) -> f64 {
        self.coefficient_of_restitution
    }
}
