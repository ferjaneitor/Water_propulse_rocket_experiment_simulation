pub mod dynamics{
    // dynamics.rs
    /// Simple helpers with safe division; return None on invalid inputs.
    pub fn force(mass: f64, acceleration: f64) -> f64 { mass * acceleration }

    pub fn acceleration_from_force(force: f64, mass: f64) -> Option<f64> {
        (mass.abs() > f64::EPSILON).then_some(force / mass)
    }

    pub fn mass_from(force: f64, acceleration: f64) -> Option<f64> {
        (acceleration.abs() > f64::EPSILON).then_some(force / acceleration)
    }
}