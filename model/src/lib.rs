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


/// CO2 produced by complete combustion of pure methane, kg CO2/kg CH4.
/// This is a chemical upper-bound conversion for a methane-only fuel stream,
/// not a natural-gas composition model.
pub fn co2_from_pure_methane_combustion_kg_per_kg_ch4() -> f64 {
    const M_CO2: f64 = 44.0095;
    const M_CH4: f64 = 16.04246;
    M_CO2 / M_CH4
}

/// Screening upper bound for CO2 associated with the separately reported
/// make-up natural-gas fuel, treating that fuel as pure methane.
///
/// IMPORTANT: this is NOT the total furnace CO2. Conventional SMR also burns
/// PSA tail gas containing unrecovered H2, CH4, CO and CO2. Removing the fired
/// furnace therefore creates a tail-gas disposition problem rather than making
/// its carbon disappear.
pub fn makeup_fuel_co2_upper_bound_kg_per_kg_h2(case: SmrCase) -> f64 {
    case.ng_fuel_kg_per_kg_h2() * co2_from_pure_methane_combustion_kg_per_kg_ch4()
}


#[derive(Debug, Clone, Copy)]
pub struct NgComposition {
    pub co2: f64,
    pub methane: f64,
    pub ethane: f64,
    pub propane: f64,
    pub n_butane: f64,
    pub n_pentane: f64,
}

impl NgComposition {
    /// kmol of carbon atoms per kmol of natural-gas mixture.
    pub fn carbon_kmol_per_kmol(self) -> f64 {
        self.co2
            + self.methane
            + 2.0 * self.ethane
            + 3.0 * self.propane
            + 4.0 * self.n_butane
            + 5.0 * self.n_pentane
    }
}

/// IEAGHG 2017-02 base-case natural-gas composition (molar fractions).
pub const IEAGHG_NG: NgComposition = NgComposition {
    co2: 0.0200,
    methane: 0.8900,
    ethane: 0.0700,
    propane: 0.0100,
    n_butane: 0.0010,
    n_pentane: 0.0001,
};

/// kmol-C/h in the IEAGHG base-case feedstock NG stream.
pub fn ieaghg_feed_carbon_kmol_per_h() -> f64 {
    1455.8 * IEAGHG_NG.carbon_kmol_per_kmol()
}

/// kmol-C/h in the IEAGHG base-case separately supplied furnace-fuel NG.
pub fn ieaghg_makeup_fuel_carbon_kmol_per_h() -> f64 {
    240.4 * IEAGHG_NG.carbon_kmol_per_kmol()
}

/// kmol-C/h in the PSA tail gas from its reported molar flow and composition.
pub fn ieaghg_tail_gas_carbon_kmol_per_h() -> f64 {
    const TAIL_KMOL_H: f64 = 2106.3;
    const Y_CO2: f64 = 0.5095;
    const Y_CO: f64 = 0.1454;
    const Y_CH4: f64 = 0.0945;
    TAIL_KMOL_H * (Y_CO2 + Y_CO + Y_CH4)
}

/// kmol-C/h in the base-case flue gas.
/// The IEAGHG heat/material balance reports essentially all flue carbon as CO2.
pub fn ieaghg_flue_carbon_kmol_per_h() -> f64 {
    8659.4 * 0.2123
}

/// Fraction of total incoming NG carbon attributable to separately supplied
/// make-up furnace fuel (rather than feedstock).
pub fn ieaghg_makeup_fuel_fraction_of_input_carbon() -> f64 {
    let makeup = ieaghg_makeup_fuel_carbon_kmol_per_h();
    makeup / (ieaghg_feed_carbon_kmol_per_h() + makeup)
}

/// Carbon remaining after the shifted-syngas CO2 is removed before PSA,
/// using the IEAGHG base-case PSA-inlet stream. This consists primarily of
/// CO and CH4 and therefore cannot simply be vented in a nuclear-heated case.
pub fn ieaghg_psa_inlet_non_co2_carbon_kmol_per_h() -> f64 {
    const PSA_IN_KMOL_H: f64 = 6596.9;
    const Y_CO: f64 = 0.0464;
    const Y_CH4: f64 = 0.0302;
    PSA_IN_KMOL_H * (Y_CO + Y_CH4)
}
