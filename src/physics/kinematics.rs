// kinematics.rs
pub mod mru {
    pub fn position(x0: f64, v: f64, t: f64) -> f64 { x0 + v * t }
    pub fn velocity(displacement: f64, time: f64) -> Option<f64> {
        (time.abs() > f64::EPSILON).then_some(displacement / time)
    }
    pub fn displacement(v: f64, t: f64) -> f64 { v * t }
    pub fn time(displacement: f64, velocity: f64) -> Option<f64> {
        (velocity.abs() > f64::EPSILON).then_some(displacement / velocity)
    }
}

pub mod mrua {
    pub fn position(x0: f64, v0: f64, a: f64, t: f64) -> f64 {
        x0 + v0 * t + 0.5 * a * t * t
    }
    pub fn final_velocity(v0: f64, a: f64, t: f64) -> f64 { v0 + a * t }
    pub fn displacement(v0: f64, t: f64, a: f64) -> f64 { v0 * t + 0.5 * a * t * t }
    pub fn acceleration_from(vf: f64, v0: f64, t: f64) -> Option<f64> {
        (t.abs() > f64::EPSILON).then_some((vf - v0) / t)
    }
}
