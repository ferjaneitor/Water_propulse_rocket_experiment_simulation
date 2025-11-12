// constants.rs
use crate::math_utils::vector_2d::Vector2D;

pub const GRAVITY: f64 = 9.81; // m/s^2 (standard approx.)
pub const WATER_DENSITY: f64 = 1000.0; // kg/m^3
pub const ATMOSFERIC_PRESSURE: f64 = 101325.0; // Pa at sea level
pub const ADIABATIC_INDEX_AIR: f64 = 1.4; // for air

pub const INITIAL_POSITION: Vector2D = Vector2D::ZERO;
pub const INITIAL_VELOCITY: Vector2D = Vector2D::ZERO;
pub const INITIAL_ACCELERATION: Vector2D = Vector2D::ZERO;
pub const LAUNCHING_ANGLE_DEG: f64 = 45.0; // degrees
pub const DRY_MASS: f64 = 0.6; // kg
pub const INITIAL_WATER_MASS: f64 = 1.0; // kg
pub const INITIAL_AIR_PRESSURE_PSI: f64 = 45.0; // psi
pub const DISCHARGE_COEFFICIENT : f64 = 0.0;

pub const BOTTLE_VOLUME : f64 = 0.002; // m^3 (2 liters)
pub const NOZZLE_DIAMETER : f64 = 0.021; // m