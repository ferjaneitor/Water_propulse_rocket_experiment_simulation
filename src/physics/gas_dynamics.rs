pub mod gas_dynamics{
    // gas_dynamics.rs
    /// Polytropic / adiabatic invariant: p * V^gamma = const
    pub fn polytropic_constant(pressure: f64, volume: f64, gamma: f64) -> f64 {
        pressure * volume.powf(gamma)
    }

    /// Ideal (Bernoulli-style) exit velocity for an orifice into atmosphere.
    /// Returns 0 if Δp <= 0.
    pub fn exit_velocity(p_air: f64, p_atm: f64, rho: f64) -> f64 {
        let dp = p_air - p_atm;
        if dp <= 0.0 || rho <= 0.0 { 0.0 } else { (2.0 * dp / rho).sqrt() }
    }

    /// Mass flow rate ρ A v
    pub fn mass_flow_rate(v_exit: f64, area: f64, rho: f64) -> f64 { rho * area * v_exit }

    /// Volumetric flow rate A v
    pub fn volumetric_flow_rate(v_exit: f64, area: f64) -> f64 { area * v_exit }

    /// Thrust ≈ ṁ v (no pressure term)
    pub fn thrust(mdot: f64, v_exit: f64) -> f64 { mdot * v_exit }

}