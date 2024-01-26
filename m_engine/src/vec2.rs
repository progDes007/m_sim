use crate::config::*;

pub struct Vec2 {
    x: Real,
    y: Real,
}

impl Vec2 {
    pub fn new(x: Real, y: Real) -> Self {
        Self { x, y }
    }
    pub fn length(&self) -> Real {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(Vec2::new(3.0, 4.0).length(), 5.0);
        assert_eq!(Vec2::new(0.0, 0.0).length(), 0.0);
        assert_eq!(Vec2::new(-3.0, 4.0).length(), 5.0);
    }
}
