mod constants;
mod math_utils;
mod physics;

use math_utils::vector_2d::Vector2D;
use constants::{GRAVITY, INITIAL_POSITION};
use physics::dynamics;

fn main() {
    println!("Hello, world!");
}


pub struct Simulation {
    pub position: Vector2D,
    pub steps: u32,
}

impl Simulation {
    pub fn new(position: Vector2D, steps: u32) -> Simulation {
        Simulation { position, steps }
    }
}
