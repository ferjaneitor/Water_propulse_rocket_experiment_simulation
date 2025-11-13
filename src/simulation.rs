use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::{
    constants::{
        ADIABATIC_INDEX_AIR,
        ATMOSFERIC_PRESSURE,
        BOTTLE_VOLUME,
        DISCHARGE_COEFFICIENT,
        DRY_MASS,
        GRAVITY,
        INITIAL_ACCELERATION,
        INITIAL_AIR_PRESSURE_PSI, // asumida manométrica (gauge)
        INITIAL_POSITION,
        INITIAL_VELOCITY,
        INITIAL_WATER_MASS,
        LAUNCHING_ANGLE_DEG,
        NOZZLE_DIAMETER,
        WATER_DENSITY,
    },
    math_utils::vector_2d::Vector2D,
    physics::gas_dynamics::gas_dynamics,
};

pub struct Simulation {
    // --- tiempo ---
    pub(crate) step: f64, // paso de simulación [s]
    pub(crate) time: f64, // tiempo actual [s]

    // --- parámetros físicos ---
    pub(crate) gravity: f64,              // [m/s²]
    pub(crate) water_density: f64,        // [kg/m³]
    pub(crate) atmospheric_pressure: f64, // [Pa]
    pub(crate) adiabatic_index: f64,      // índice adiabático

    // --- estado de traslación ---
    pub(crate) position: Vector2D,     // [m]
    pub(crate) velocity: Vector2D,     // [m/s]
    pub(crate) acceleration: Vector2D, // [m/s²]

    // --- estado de masa/volúmenes/presión ---
    pub(crate) water_mass: f64, // masa de agua actual [kg]
    pub(crate) dry_mass: f64,   // masa en seco [kg]
    pub(crate) total_mass: f64, // masa total actual [kg]

    pub(crate) bottle_volume: f64,       // volumen interno botella [m³]
    pub(crate) nozzle_area: f64,         // área tobera [m²]
    pub(crate) launching_angle_rad: f64, // ángulo lanzamiento [rad]

    // presión inicial ABSOLUTA (gauge convertida a Pa + atm)
    #[allow(dead_code)]
    pub(crate) initial_air_pressure_pa: f64,

    // NUEVO: estado termodinámico dinámico
    pub(crate) polytropic_constant: f64, // K = P * V^gamma
    pub(crate) current_pressure_pa: f64, // presión interna absoluta actual [Pa]
    pub(crate) current_air_volume: f64,  // volumen de aire actual [m³]

    // para compatibilidad si quieres leerlo externamente
    pub(crate) water_volume: f64, // volumen de agua (se actualiza opcionalmente)
    pub(crate) air_volume: f64,   // volumen de aire (se espelha de current_air_volume)

    // coeficiente de descarga (0..1)
    pub(crate) discharge_coefficient: f64,

    // --- fuerzas ---
    pub(crate) thrust_force: f64, // empuje instantáneo [N]

    // --- logs ---
    pub(crate) x_log: Vec<f64>,
    pub(crate) y_log: Vec<f64>,
    pub(crate) x_velocity_log: Vec<f64>,
    pub(crate) y_velocity_log: Vec<f64>,
    pub(crate) velocity_magnitur_log: Vec<f64>,
    pub(crate) x_acceleration_log: Vec<f64>,
    pub(crate) y_acceleration_log: Vec<f64>,
    pub(crate) acceleration_magnitur_log: Vec<f64>,
    pub(crate) time_log: Vec<f64>,
    pub(crate) water_mass_log: Vec<f64>,
    pub(crate) mass_log: Vec<f64>,
    pub(crate) thrust_log: Vec<f64>,
    pub(crate) pressure_log: Vec<f64>, // ahora guarda presión ACTUAL absoluta
}

impl Simulation {
    // =========================
    // construcción
    // =========================
    pub fn new(time_steps: f64) -> Self {
        let launching_angle_rad: f64 = LAUNCHING_ANGLE_DEG.to_radians();
        let nozzle_area: f64 = std::f64::consts::PI * (NOZZLE_DIAMETER / 2.0).powi(2);

        // presión inicial ABSOLUTA = (psi gauge -> Pa) + atm
        let initial_air_pressure_pa_abs: f64 =
            INITIAL_AIR_PRESSURE_PSI * 6894.757293168361 + ATMOSFERIC_PRESSURE;

        // volúmenes iniciales
        let water_volume: f64 = INITIAL_WATER_MASS / WATER_DENSITY;
        let air_volume: f64 = (BOTTLE_VOLUME - water_volume).max(1e-12);

        // constante politrópica K = P0 * Va0^gamma
        let polytropic_constant: f64 =
            initial_air_pressure_pa_abs * air_volume.powf(ADIABATIC_INDEX_AIR);

        let total_mass: f64 = INITIAL_WATER_MASS + DRY_MASS;

        let mut sim = Self {
            step: time_steps,
            time: 0.0,

            gravity: GRAVITY,
            water_density: WATER_DENSITY,
            atmospheric_pressure: ATMOSFERIC_PRESSURE,
            adiabatic_index: ADIABATIC_INDEX_AIR,

            position: INITIAL_POSITION,
            velocity: INITIAL_VELOCITY,
            acceleration: INITIAL_ACCELERATION,

            water_mass: INITIAL_WATER_MASS,
            dry_mass: DRY_MASS,
            total_mass,

            bottle_volume: BOTTLE_VOLUME,
            nozzle_area,
            launching_angle_rad,

            initial_air_pressure_pa: initial_air_pressure_pa_abs,

            polytropic_constant,
            current_pressure_pa: initial_air_pressure_pa_abs,
            current_air_volume: air_volume,

            water_volume,
            air_volume, // espejo del actual para mantener compatibilidad

            discharge_coefficient: DISCHARGE_COEFFICIENT, // realista; cambia si quieres

            thrust_force: 0.0,

            x_log: Vec::new(),
            y_log: Vec::new(),
            x_velocity_log: Vec::new(),
            y_velocity_log: Vec::new(),
            velocity_magnitur_log: Vec::new(),
            x_acceleration_log: Vec::new(),
            y_acceleration_log: Vec::new(),
            acceleration_magnitur_log: Vec::new(),
            time_log: Vec::new(),
            water_mass_log: Vec::new(),
            mass_log: Vec::new(),
            thrust_log: Vec::new(),
            pressure_log: Vec::new(),
        };

        sim.push_logs(); // primer muestreo
        sim
    }

    // =========================
    // bucle principal
    // =========================
    pub fn run(&mut self, t_max: f64) {
        while self.time < t_max {
            self.step_once();

            // cortar si toca suelo tras un instante
            if self.time > 0.02 && self.position.y < 0.0 {
                break;
            }
        }
    }

    pub fn step_once(&mut self) {
        self.update_current_air_volume();
        self.update_current_internal_pressure();

        let delta_p = self.current_pressure_pa - self.atmospheric_pressure;
        if self.water_mass <= 0.0 || delta_p <= 0.0 {
            // sin empuje
            self.thrust_force = 0.0;
            self.update_total_mass(self.water_mass);
            self.update_current_acceleration();
            self.update_current_velocity();
            self.update_current_position();
            self.update_time();
            self.push_logs();
            return;
        }

        let exit_velocity = self.compute_exit_velocity();
        let mass_flow = self.compute_mass_flow(exit_velocity);

        // Si por algún motivo no hay flujo, no “inventes” empuje
        if mass_flow <= 0.0 || exit_velocity <= 0.0 {
            self.thrust_force = 0.0;
        } else {
            self.update_current_thrust(exit_velocity, mass_flow);
        }

        self.update_total_mass(self.water_mass);
        self.update_current_acceleration();
        self.update_current_velocity();
        self.update_current_position();
        self.update_current_water_flow_with(mass_flow);

        self.update_time();
        self.push_logs();
    }

    // =========================
    // pequeñas funciones puras / atómicas
    // =========================

    // masa total = seco + agua (con argumento por compatibilidad)
    pub fn update_total_mass(&mut self, new_water_mass: f64) {
        self.total_mass = self.dry_mass + new_water_mass;
    }

    // volumen de aire actual = volumen botella - volumen de agua
    pub fn update_current_air_volume(&mut self) {
        self.current_air_volume =
            (self.bottle_volume - (self.water_mass / self.water_density)).max(1e-12);
        // mantener campos espejo si los usas en otro lado
        self.water_volume = self.water_mass / self.water_density;
        self.air_volume = self.current_air_volume;
    }

    // presión actual por politropía: P = K / Va^gamma
    pub fn update_current_internal_pressure(&mut self) {
        self.current_pressure_pa =
            self.polytropic_constant / self.current_air_volume.powf(self.adiabatic_index);
    }

    // velocidad de salida: con o sin coeficiente de descarga
    pub fn compute_exit_velocity(&self) -> f64 {
        if self.water_mass <= 0.0 {
            return 0.0;
        }
        let dp = self.current_pressure_pa - self.atmospheric_pressure;
        if dp <= 0.0 {
            return 0.0;
        }

        if self.discharge_coefficient > 0.0 {
            gas_dynamics::exit_velocity_with_cd(
                self.discharge_coefficient,
                self.current_pressure_pa,
                self.atmospheric_pressure,
                self.water_density,
            )
        } else {
            gas_dynamics::exit_velocity(
                self.current_pressure_pa,
                self.atmospheric_pressure,
                self.water_density,
            )
        }
    }

    // flujo másico = densidad * área * velocidad_salida
    pub fn compute_mass_flow(&self, exit_velocity: f64) -> f64 {
        if self.water_mass <= 0.0 || exit_velocity <= 0.0 {
            return 0.0;
        }
        gas_dynamics::mass_flow_rate(exit_velocity, self.nozzle_area, self.water_density)
    }

    // empuje = ṁ * v + (P_interna - P_atm) * área
    pub fn update_current_thrust(&mut self, exit_velocity: f64, mass_flow: f64) {
        if self.water_mass <= 0.0 || exit_velocity <= 0.0 || mass_flow <= 0.0 {
            self.thrust_force = 0.0;
            return;
        }
        self.thrust_force = gas_dynamics::thrust(
            mass_flow,
            exit_velocity,
            self.current_pressure_pa,
            self.atmospheric_pressure,
            self.nozzle_area,
        );
    }

    // aceleración a partir de empuje y masa total (proyección por ángulo)
    pub fn update_current_acceleration(&mut self) {
        let ax = (self.thrust_force * self.launching_angle_rad.cos()) / self.total_mass;
        let ay =
            (self.thrust_force * self.launching_angle_rad.sin()) / self.total_mass - self.gravity;
        self.acceleration = Vector2D { x: ax, y: ay };
    }

    // integra velocidad (NO vuelve a calcular aceleración aquí)
    pub fn update_current_velocity(&mut self) {
        self.velocity = self.velocity + self.acceleration * self.step;
    }

    // integra posición con v(t+dt)
    pub fn update_current_position(&mut self) {
        self.position = self.position + self.velocity * self.step;
    }

    // restar agua con un flujo ya calculado en este paso
    pub fn update_current_water_flow_with(&mut self, mass_flow: f64) {
        let dm = mass_flow * self.step;
        self.water_mass = (self.water_mass - dm).max(0.0);
        self.update_total_mass(self.water_mass);
        if self.water_mass <= 0.0 {
            self.thrust_force = 0.0;
        }
    }

    // si quieres mantener la vieja función, hazla delegar:
    #[allow(dead_code)]
    pub fn update_current_water_flow(&mut self) {
        let v = self.compute_exit_velocity();
        let m_dot = self.compute_mass_flow(v);
        self.update_current_water_flow_with(m_dot);
    }

    pub fn update_time(&mut self) {
        self.time = self.time + self.step;
    }

    // =========================
    // logs y export
    // =========================
    pub fn push_logs(&mut self) {
        self.x_log.push(self.position.x);
        self.y_log.push(self.position.y);
        self.x_velocity_log.push(self.velocity.x);
        self.y_velocity_log.push(self.velocity.y);
        self.velocity_magnitur_log.push(self.velocity.magnitude());
        self.x_acceleration_log.push(self.acceleration.x);
        self.y_acceleration_log.push(self.acceleration.y);
        self.acceleration_magnitur_log
            .push(self.acceleration.magnitude());
        self.time_log.push(self.time);
        self.water_mass_log.push(self.water_mass);
        self.mass_log.push(self.total_mass);
        self.thrust_log.push(self.thrust_force);
        self.pressure_log.push(self.current_pressure_pa); // << presión ACTUAL absoluta
    }

    /// Exporta los logs a un archivo CSV sin usar crates externos.
    /// Usa coma como separador (puedes cambiarla por ';' si tu Excel lo requiere).
    pub fn export_logs_to_csv<P: AsRef<Path>>(
        &self,
        path: P,
        delimiter: char,
    ) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // BOM UTF-8 (Excel)
        writer.write_all("\u{FEFF}".as_bytes())?;

        // Encabezados (renombré presión para dejar claro que es absoluta)
        writeln!(
            writer,
            "time_s{d}x_m{d}y_m{d}vx_m_s{d}vy_m_s{d}v_mag_m_s{d}ax_m_s2{d}ay_m_s2{d}a_mag_m_s2{d}water_mass_kg{d}total_mass_kg{d}thrust_N{d}pressure_abs_Pa",
            d = delimiter
        )?;

        // Longitud mínima común
        let n = [
            self.time_log.len(),
            self.x_log.len(),
            self.y_log.len(),
            self.x_velocity_log.len(),
            self.y_velocity_log.len(),
            self.velocity_magnitur_log.len(),
            self.x_acceleration_log.len(),
            self.y_acceleration_log.len(),
            self.acceleration_magnitur_log.len(),
            self.water_mass_log.len(),
            self.mass_log.len(),
            self.thrust_log.len(),
            self.pressure_log.len(),
        ]
        .iter()
        .min()
        .cloned()
        .unwrap_or(0);

        // Filas
        for i in 0..n {
            writeln!(
                writer,
                "{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}{d}{:.6}",
                self.time_log[i],
                self.x_log[i],
                self.y_log[i],
                self.x_velocity_log[i],
                self.y_velocity_log[i],
                self.velocity_magnitur_log[i],
                self.x_acceleration_log[i],
                self.y_acceleration_log[i],
                self.acceleration_magnitur_log[i],
                self.water_mass_log[i],
                self.mass_log[i],
                self.thrust_log[i],
                self.pressure_log[i],
                d = delimiter
            )?;
        }

        writer.flush()?;
        Ok(())
    }
}

// =========================
// Tests finales
// =========================

/// Con las constantes actuales, el burnout (agua agotada) debe ser rápido.
#[test]
fn burnout_time_is_short() {
    let dt = 1e-3;
    let mut sim = super::Simulation::new(dt);
    let mut t_burnout = 0.0;
    for _ in 0..10_000 {
        if sim.water_mass <= 0.0 {
            t_burnout = sim.time;
            break;
        }
        sim.step_once();
    }
    assert!(
        sim.water_mass <= 1e-9,
        "no llegó a burnout en el tiempo simulado"
    );
    // tolerancia amplia: debería ser << 0.5 s con tus constantes (~0.1–0.2)
    assert!(
        t_burnout > 0.0 && t_burnout < 0.5,
        "burnout fuera de rango razonable: {:.4} s",
        t_burnout
    );
}

/// Tras burnout, el cohete es balístico: el apogeo y el alcance deben quedar en un rango razonable.
#[test]
fn apogee_and_range_reasonable() {
    let dt = 1e-3;
    let mut sim = super::Simulation::new(dt);
    // corre hasta tocar el suelo o 30 s (lo que ocurra primero)
    sim.run(30.0);

    // Apogeo
    let y_max = sim.y_log.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    // Alcance (x final)
    let x_final = *sim.x_log.last().unwrap_or(&0.0);

    // Rangos amplios para no ser frágiles frente a pequeños cambios de modelo:
    // con tus constantes deberías ver decenas de metros en apogeo y ~100 m de alcance.
    assert!(
        y_max.is_finite() && y_max > 5.0 && y_max < 300.0,
        "apogeo fuera de rango: y_max = {} m",
        y_max
    );
    assert!(
        x_final.is_finite() && x_final > 10.0 && x_final < 600.0,
        "alcance fuera de rango: x_final = {} m",
        x_final
    );
}
