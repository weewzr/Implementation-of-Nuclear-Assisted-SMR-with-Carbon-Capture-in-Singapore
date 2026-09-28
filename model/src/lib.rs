//! Screening-level SMR baseline model.
//!
//! The first validation target is IEAGHG 2017-02.  This module intentionally
//! keeps the published reference case separate from later Singapore and nuclear
//! assumptions.

/// kg H2 per normal cubic metre implied by IEAGHG's own base-case pair:
/// 8.994 t/h = 100,000 Nm3/h.
pub const IEAGHG_KG_H2_PER_NM3: f64 = 8_994.0 / 100_000.0;

#[derive(Debug, Clone, Copy)]
pub struct SmrCase {
    pub h2_kg_per_h: f64,
    pub ng_feed_kg_per_h: f64,
    pub ng_fuel_kg_per_h: f64,
    pub co2_emitted_kg_per_nm3_h2: f64,
    pub co2_captured_kg_per_nm3_h2: f64,
}

impl SmrCase {
    pub fn ng_total_kg_per_h(self) -> f64 {
        self.ng_feed_kg_per_h + self.ng_fuel_kg_per_h
    }

    pub fn ng_feed_kg_per_kg_h2(self) -> f64 {
        self.ng_feed_kg_per_h / self.h2_kg_per_h
    }

    pub fn ng_fuel_kg_per_kg_h2(self) -> f64 {
        self.ng_fuel_kg_per_h / self.h2_kg_per_h
    }

    pub fn ng_total_kg_per_kg_h2(self) -> f64 {
        self.ng_total_kg_per_h() / self.h2_kg_per_h
    }

    pub fn co2_emitted_kg_per_kg_h2(self) -> f64 {
        self.co2_emitted_kg_per_nm3_h2 / IEAGHG_KG_H2_PER_NM3
    }

    pub fn co2_captured_kg_per_kg_h2(self) -> f64 {
        self.co2_captured_kg_per_nm3_h2 / IEAGHG_KG_H2_PER_NM3
    }
}

/// IEAGHG 2017-02 base case, 100,000 Nm3/h H2.
pub const IEAGHG_BASE: SmrCase = SmrCase {
    h2_kg_per_h: 8_994.0,
    ng_feed_kg_per_h: 26_231.0,
    ng_fuel_kg_per_h: 4_332.0,
    co2_emitted_kg_per_nm3_h2: 0.8091,
    co2_captured_kg_per_nm3_h2: 0.0,
};

/// IEAGHG 2017-02 Case 1A: shifted-syngas capture using MDEA.
pub const IEAGHG_CASE_1A: SmrCase = SmrCase {
    h2_kg_per_h: 8_994.0,
    ng_feed_kg_per_h: 26_262.0,
    ng_fuel_kg_per_h: 5_300.0,
    co2_emitted_kg_per_nm3_h2: 0.3704,
    co2_captured_kg_per_nm3_h2: 0.4660,
};

/// Plant-gate marginal abatement in kg CO2 per kg H2.
///
/// This deliberately excludes upstream methane, electricity, CO2 transport,
/// storage and infrastructure. Those terms belong in the later lifecycle model.
pub fn plant_gate_abatement_kg_per_kg_h2(baseline: SmrCase, candidate: SmrCase) -> f64 {
    baseline.co2_emitted_kg_per_kg_h2() - candidate.co2_emitted_kg_per_kg_h2()
}

/// Annual hydrogen production required to exceed an annual abatement target.
///
/// Because kg CO2/kg H2 is numerically equal to t CO2/t H2, the result follows
/// directly from target tonnes divided by specific abatement.
pub fn required_h2_t_per_year(target_co2_t_per_year: f64, delta_kg_per_kg: f64) -> f64 {
    assert!(delta_kg_per_kg > 0.0);
    target_co2_t_per_year / delta_kg_per_kg
}

/// Ideal stoichiometric methane demand for CH4 + 2 H2O -> CO2 + 4 H2.
pub fn ideal_ch4_kg_per_kg_h2() -> f64 {
    const M_CH4: f64 = 16.04246;
    const M_H2: f64 = 2.01588;
    M_CH4 / (4.0 * M_H2)
}

/// Ideal process CO2 generation for CH4 + 2 H2O -> CO2 + 4 H2.
pub fn ideal_process_co2_kg_per_kg_h2() -> f64 {
    const M_CO2: f64 = 44.0095;
    const M_H2: f64 = 2.01588;
    M_CO2 / (4.0 * M_H2)
}
