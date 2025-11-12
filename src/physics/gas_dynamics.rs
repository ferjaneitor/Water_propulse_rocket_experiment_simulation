pub mod gas_dynamics {
    // gas_dynamics.rs

    /// Ideal (Bernoulli-style) exit velocity for an orifice into atmosphere.
    /// Returns 0 if Δp <= 0.
    pub fn exit_velocity(p_air: f64, p_atm: f64, rho: f64) -> f64 {
        let dp = p_air - p_atm;
        if dp <= 0.0 || rho <= 0.0 { 0.0 } else { (2.0 * dp / rho).sqrt() }
    }

    /// Mass flow rate ρ A v
    pub fn mass_flow_rate(v_exit: f64, area: f64, rho: f64) -> f64 { rho * area * v_exit }

    /// Thrust ≈ ṁ v (no pressure term)
    pub fn thrust(mdot: f64, v_exit: f64, p_air: f64, p_atm: f64, area: f64) -> f64 { 
    
        mdot * v_exit + (p_air-p_atm) - area 
    
    }

    /// Exit velocity with discharge coefficient.
    /// Returns 0 if any parameter is invalid (c_d ≤ 0, Δp ≤ 0, or ρ ≤ 0).
    #[inline]
    pub fn exit_velocity_with_cd(c_d: f64, p_air: f64, p_atm: f64, rho: f64) -> f64 {
        let dp = p_air - p_atm;
        if c_d <= 0.0 || dp <= 0.0 || rho <= 0.0 {
            0.0
        } else {
            c_d * (2.0 * dp / rho).sqrt()
        }
    }

}