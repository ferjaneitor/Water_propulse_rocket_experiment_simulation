use crate::simulation::Simulation;

mod constants;
mod math_utils;
mod physics;
mod simulation;

fn main() {
    
    let mut sim = Simulation::new(0.00001);

    sim.run(30.0);

    // Exportar con coma (,) o punto y coma (;)
    sim.export_logs_to_csv("logs.csv", ',').expect("Error al exportar CSV");

}


