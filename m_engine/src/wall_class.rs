#[derive(Debug, Clone)]
/// Wall class. Describes the properties of the wall.
pub struct WallClass {
    name: String,
    temperature: f64,
    heat_conductivity: f64,
}

impl WallClass {
    /// Constructor.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the wall.
    /// * `temperature` - The temperature of the wall.
    /// * `heat_conductivity` - The heat conductivity of the wall.
    pub fn new(name: &str, temperature: f64, heat_conductivity: f64) -> Self {
        WallClass {
            name: name.to_string(),
            temperature,
            heat_conductivity,
        }
    }

    /// Get the name of the wall.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Temperature of the wall
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    /// Heat conductivity of the wall
    pub fn heat_conductivity(&self) -> f64 {
        self.heat_conductivity
    }
}
