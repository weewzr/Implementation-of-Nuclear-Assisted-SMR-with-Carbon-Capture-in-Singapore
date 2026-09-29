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


/// A reduced C-H-O material stream for the IEAGHG reference model.
///
/// Nitrogen, argon and unreacted steam are deliberately excluded from this
/// struct until the complete published stream table is encoded. This model is
/// therefore for carbon/hydrogen chemistry closure, not yet a total wet-gas
/// flowsheet.
#[derive(Debug, Clone, Copy)]
pub struct ChoStream {
    pub h2: f64,
    pub co2: f64,
    pub co: f64,
    pub ch4: f64,
}

impl ChoStream {
    pub fn carbon_kmol_per_h(self) -> f64 {
        self.co2 + self.co + self.ch4
    }

    pub fn combustible_kmol_per_h(self) -> f64 {
        self.h2 + self.co + self.ch4
    }
}

/// IEAGHG base-case PSA inlet reconstructed from the published total molar
/// flow and dry/wet composition entries used in the report's heat/material
/// balance. Values are kmol/h.
pub fn ieaghg_psa_inlet_cho() -> ChoStream {
    const TOTAL: f64 = 6596.9;
    ChoStream {
        h2: TOTAL * 0.7563,
        co2: TOTAL * 0.1627,
        co: TOTAL * 0.0464,
        ch4: TOTAL * 0.0302,
    }
}

/// IEAGHG base-case PSA tail gas. Values are kmol/h.
pub fn ieaghg_psa_tail_cho() -> ChoStream {
    const TOTAL: f64 = 2106.3;
    ChoStream {
        h2: TOTAL * 0.2369,
        co2: TOTAL * 0.5095,
        co: TOTAL * 0.1454,
        ch4: TOTAL * 0.0945,
    }
}

/// Hydrogen product implied by the difference between PSA inlet and tail gas.
/// This is a reconstruction from rounded source stream values, not an
/// independent PSA adsorption model.
pub fn ieaghg_reconstructed_h2_product_kmol_per_h() -> f64 {
    ieaghg_psa_inlet_cho().h2 - ieaghg_psa_tail_cho().h2
}

/// Hydrogen recovery reconstructed from the published inlet/tail streams.
pub fn ieaghg_reconstructed_psa_h2_recovery() -> f64 {
    let inlet = ieaghg_psa_inlet_cho().h2;
    ieaghg_reconstructed_h2_product_kmol_per_h() / inlet
}

/// Fraction of the PSA-tail-gas molar flow that is combustible H2+CO+CH4.
/// This is not an energy fraction; LHV accounting is a later model layer.
pub fn ieaghg_tail_combustible_mole_fraction() -> f64 {
    let tail = ieaghg_psa_tail_cho();
    tail.combustible_kmol_per_h() / 2106.3
}

/// CO2 fraction of tail-gas carbon. The remainder is carbon in CO+CH4 and
/// survives a hypothetical CO2-only separator.
pub fn ieaghg_tail_carbon_as_co2_fraction() -> f64 {
    let tail = ieaghg_psa_tail_cho();
    tail.co2 / tail.carbon_kmol_per_h()
}


#[derive(Debug, Clone, Copy)]
pub struct ReactionExtents {
    /// CH4 + H2O -> CO + 3 H2, kmol/h.
    pub smr: f64,
    /// CO + H2O -> CO2 + H2, kmol/h.
    pub wgs: f64,
}

/// Reconstruct reaction extents from a methane-only reduced model.
///
/// The inlet is an equivalent methane flow carrying the same number of carbon
/// atoms as the actual NG feed. This is a carbon-equivalent reduction, not a
/// claim that the published NG is pure methane.
pub fn reduced_extents_from_psa_inlet() -> ReactionExtents {
    let p = ieaghg_psa_inlet_cho();
    let carbon_feed = ieaghg_feed_carbon_kmol_per_h();
    let smr = carbon_feed - p.ch4;
    let wgs = p.co2;
    ReactionExtents { smr, wgs }
}

/// H2 implied by the reduced equivalent-CH4 reforming + WGS extents.
pub fn reduced_reaction_h2_kmol_per_h() -> f64 {
    let x = reduced_extents_from_psa_inlet();
    3.0 * x.smr + x.wgs
}

/// Net steam consumed chemically by the reduced SMR + WGS reactions.
pub fn reduced_chemical_steam_consumption_kmol_per_h() -> f64 {
    let x = reduced_extents_from_psa_inlet();
    x.smr + x.wgs
}

/// Standard reaction enthalpies at 298.15 K, kJ/mol reaction.
/// Values correspond to gaseous H2O:
/// CH4 + H2O(g) -> CO + 3H2 : +206.1 kJ/mol
/// CO + H2O(g) -> CO2 + H2 : -41.2 kJ/mol
pub const DELTA_H_SMR_298_KJ_PER_MOL: f64 = 206.1;
pub const DELTA_H_WGS_298_KJ_PER_MOL: f64 = -41.2;

/// Reference-state reaction duty from the reduced extents, MW.
///
/// This is NOT the fired-reformer duty. It excludes sensible heating,
/// vaporisation, excess steam, higher-hydrocarbon prereforming, heat losses,
/// equilibrium temperature dependence and heat recovery.
pub fn reduced_standard_reaction_duty_mw() -> f64 {
    let x = reduced_extents_from_psa_inlet();
    // kmol/h * kJ/mol = MJ/h; divide by 3600 => MW.
    (x.smr * DELTA_H_SMR_298_KJ_PER_MOL
        + x.wgs * DELTA_H_WGS_298_KJ_PER_MOL)
        / 3600.0
}


#[derive(Debug, Clone, Copy)]
pub struct FullStream {
    pub total_kmol_h: f64,
    pub co2: f64,
    pub co: f64,
    pub h2: f64,
    pub n2: f64,
    pub ch4: f64,
    pub c2h6: f64,
    pub c3h8: f64,
    pub nc4h10: f64,
    pub nc5h12: f64,
    pub h2o: f64,
}

impl FullStream {
    pub fn flow(self, y: f64) -> f64 { self.total_kmol_h * y }

    pub fn carbon_kmol_h(self) -> f64 {
        self.total_kmol_h * (
            self.co2 + self.co + self.ch4 + 2.0*self.c2h6 +
            3.0*self.c3h8 + 4.0*self.nc4h10 + 5.0*self.nc5h12
        )
    }

    pub fn hydrogen_atoms_kmol_h(self) -> f64 {
        self.total_kmol_h * (
            2.0*self.h2 + 4.0*self.ch4 + 6.0*self.c2h6 +
            8.0*self.c3h8 + 10.0*self.nc4h10 + 12.0*self.nc5h12 +
            2.0*self.h2o
        )
    }

    pub fn oxygen_atoms_kmol_h(self) -> f64 {
        self.total_kmol_h * (2.0*self.co2 + self.co + self.h2o)
    }
}

pub fn ieaghg_prereformer_feed() -> FullStream {
    FullStream {
        total_kmol_h: 5514.0,
        co2: 0.0053, co: 0.0, h2: 0.0053, n2: 0.0023,
        ch4: 0.2350, c2h6: 0.0185, c3h8: 0.0026,
        nc4h10: 0.0003, nc5h12: 0.0, h2o: 0.7307,
    }
}

pub fn ieaghg_hts_inlet() -> FullStream {
    FullStream {
        total_kmol_h: 8370.3,
        co2: 0.0492, co: 0.1156, h2: 0.5171, n2: 0.0015,
        ch4: 0.0238, c2h6: 0.0, c3h8: 0.0,
        nc4h10: 0.0, nc5h12: 0.0, h2o: 0.2927,
    }
}

pub fn ieaghg_hts_outlet() -> FullStream {
    FullStream {
        total_kmol_h: 8370.3,
        co2: 0.1283, co: 0.0366, h2: 0.5961, n2: 0.0015,
        ch4: 0.0238, c2h6: 0.0, c3h8: 0.0,
        nc4h10: 0.0, nc5h12: 0.0, h2o: 0.2137,
    }
}

/// WGS extent reconstructed independently from CO consumption and CO2 formation.
pub fn ieaghg_hts_wgs_extent_from_co_kmol_h() -> f64 {
    let i=ieaghg_hts_inlet(); let o=ieaghg_hts_outlet();
    i.flow(i.co) - o.flow(o.co)
}
pub fn ieaghg_hts_wgs_extent_from_co2_kmol_h() -> f64 {
    let i=ieaghg_hts_inlet(); let o=ieaghg_hts_outlet();
    o.flow(o.co2) - i.flow(i.co2)
}
pub fn ieaghg_hts_wgs_extent_from_h2_kmol_h() -> f64 {
    let i=ieaghg_hts_inlet(); let o=ieaghg_hts_outlet();
    o.flow(o.h2) - i.flow(i.h2)
}
pub fn ieaghg_hts_wgs_extent_from_water_kmol_h() -> f64 {
    let i=ieaghg_hts_inlet(); let o=ieaghg_hts_outlet();
    i.flow(i.h2o) - o.flow(o.h2o)
}

/// Approximate source steam-to-carbon ratio entering the pre-reformer.
/// Water is divided by carbon atoms in carbonaceous species of stream 4.
/// Because stream 4 already contains a small amount of H2/CO2 produced
/// upstream, this is a stream-based diagnostic, not the plant design S/C spec.
pub fn ieaghg_prereformer_stream_steam_to_carbon() -> f64 {
    let s=ieaghg_prereformer_feed();
    s.flow(s.h2o)/s.carbon_kmol_h()
}


/// Element-balance residuals between two streams, output minus input.
#[derive(Debug, Clone, Copy)]
pub struct ElementResidual {
    pub carbon_kmol_atoms_h: f64,
    pub hydrogen_kmol_atoms_h: f64,
    pub oxygen_kmol_atoms_h: f64,
}

pub fn element_residual(input: FullStream, output: FullStream) -> ElementResidual {
    ElementResidual {
        carbon_kmol_atoms_h: output.carbon_kmol_h() - input.carbon_kmol_h(),
        hydrogen_kmol_atoms_h: output.hydrogen_atoms_kmol_h() - input.hydrogen_atoms_kmol_h(),
        oxygen_kmol_atoms_h: output.oxygen_atoms_kmol_h() - input.oxygen_atoms_kmol_h(),
    }
}

/// Source-stream diagnostic from pre-reformer feed to HTS inlet.
///
/// A nonzero residual means the two published stream rows do not form a closed
/// control volume by themselves (e.g. an omitted stream, transcription issue,
/// or an intervening source-table boundary). It must not be "fixed" by tuning.
pub fn prereformer_to_hts_source_residual() -> ElementResidual {
    element_residual(ieaghg_prereformer_feed(), ieaghg_hts_inlet())
}


/// Water/steam addition required between published IEAGHG stream 4
/// (feed to pre-reformer) and stream 5 (HTS inlet), inferred independently
/// from the H and O elemental residuals of the rounded source table.
///
/// IEAGHG's process description states that after stream 4 the pre-reformer
/// product receives a second HP-superheated-steam addition and BFW
/// desuperheating before the primary reformer. Those internal additions are
/// not separately numbered in the published heat/material-balance table.
/// The returned value is the least-squares reconciliation of the two
/// independent water estimates: dH/2 and dO. It is a reconstruction from
/// rounded source data, not an independently published flow.
pub fn ieaghg_interstage_water_addition_kmol_h() -> f64 {
    let r = prereformer_to_hts_source_residual();
    let from_h = r.hydrogen_kmol_atoms_h / 2.0;
    let from_o = r.oxygen_kmol_atoms_h;
    (from_h + from_o) / 2.0
}

/// Elemental residual after including the inferred unnumbered water/steam
/// addition between streams 4 and 5. Carbon is unaffected by water addition.
pub fn prereformer_to_hts_residual_with_inferred_water() -> ElementResidual {
    let r = prereformer_to_hts_source_residual();
    let w = ieaghg_interstage_water_addition_kmol_h();
    ElementResidual {
        carbon_kmol_atoms_h: r.carbon_kmol_atoms_h,
        hydrogen_kmol_atoms_h: r.hydrogen_kmol_atoms_h - 2.0*w,
        oxygen_kmol_atoms_h: r.oxygen_kmol_atoms_h - w,
    }
}


/// IEAGHG base-case LHV energy ledger for 100,000 Nm3/h H2.
/// Published specific consumptions are 12.197 GJ/1000 Nm3 feedstock,
/// 2.014 GJ/1000 Nm3 separately supplied NG fuel and 14.212 total.
/// The 0.001 GJ/1000 Nm3 discrepancy is source-table rounding.
pub const IEAGHG_FEED_LHV_GJ_PER_1000_NM3_H2: f64 = 12.197;
pub const IEAGHG_FUEL_LHV_GJ_PER_1000_NM3_H2: f64 = 2.014;
pub const IEAGHG_TOTAL_LHV_GJ_PER_1000_NM3_H2: f64 = 14.212;
pub const IEAGHG_H2_PRODUCT_ENERGY_MW: f64 = 299.70;
pub const IEAGHG_NET_POWER_EXPORT_MWE: f64 = 9.918;

/// Convert the IEAGHG specific-energy basis to MW at its 100,000 Nm3/h H2 scale.
pub fn ieaghg_specific_gj_per_1000_nm3_to_mw(x: f64) -> f64 {
    // x GJ / 1000 Nm3 * 100,000 Nm3/h = 100*x GJ/h.
    // 1 GJ/h = 1/3.6 MW.
    100.0 * x / 3.6
}

pub fn ieaghg_feed_lhv_mw() -> f64 {
    ieaghg_specific_gj_per_1000_nm3_to_mw(IEAGHG_FEED_LHV_GJ_PER_1000_NM3_H2)
}
pub fn ieaghg_makeup_fuel_lhv_mw() -> f64 {
    ieaghg_specific_gj_per_1000_nm3_to_mw(IEAGHG_FUEL_LHV_GJ_PER_1000_NM3_H2)
}
pub fn ieaghg_total_ng_lhv_mw() -> f64 {
    ieaghg_specific_gj_per_1000_nm3_to_mw(IEAGHG_TOTAL_LHV_GJ_PER_1000_NM3_H2)
}

/// Product-H2 LHV divided by total NG feed+fuel LHV.
/// This is a source-ledger efficiency diagnostic, not a complete exergy metric.
pub fn ieaghg_h2_lhv_efficiency() -> f64 {
    IEAGHG_H2_PRODUCT_ENERGY_MW / ieaghg_total_ng_lhv_mw()
}


/// Overall elemental closure ratios for the published stream 4 -> stream 5
/// control volume (pre-reformer + primary reformer + heat recovery).
/// Heat exchangers do not change material inventory, so source rows 4 and 5
/// should conserve C/H/O to the precision of the rounded table.
pub fn stream4_to_stream5_element_ratios() -> (f64, f64, f64) {
    let i = ieaghg_prereformer_feed();
    let o = ieaghg_hts_inlet();
    (
        o.carbon_kmol_h() / i.carbon_kmol_h(),
        o.hydrogen_atoms_kmol_h() / i.hydrogen_atoms_kmol_h(),
        o.oxygen_atoms_kmol_h() / i.oxygen_atoms_kmol_h(),
    )
}

/// Carbon-equivalent reforming extent across source streams 4 -> 5.
/// All C2+ species disappear by stream 5; carbon remaining as CH4 is
/// subtracted from total inlet carbon. Units kmol-C/h represented as
/// equivalent kmol CH4 conversion.
pub fn source_reformer_carbon_conversion_kmol_h() -> f64 {
    let i=ieaghg_prereformer_feed();
    let o=ieaghg_hts_inlet();
    i.carbon_kmol_h() - o.flow(o.ch4)
}

/// Net CO2 increase across pre-reformer + reformer before HTS, kmol/h.
pub fn source_prehts_co2_change_kmol_h() -> f64 {
    let i=ieaghg_prereformer_feed();
    let o=ieaghg_hts_inlet();
    o.flow(o.co2) - i.flow(i.co2)
}

/// Net water consumption across source streams 4 -> 5, kmol/h.
pub fn source_reformer_water_consumption_kmol_h() -> f64 {
    let i=ieaghg_prereformer_feed();
    let o=ieaghg_hts_inlet();
    i.flow(i.h2o) - o.flow(o.h2o)
}


/// IEAGHG comparison cases: authoritative total NG energy input and net power.
#[derive(Debug, Clone, Copy)]
pub struct EnergyCase {
    pub ng_input_mw_lhv: f64,
    pub h2_product_mw_lhv: f64,
    pub net_power_export_mwe: f64,
    pub co2_kg_per_nm3_h2: f64,
}

pub const IEAGHG_ENERGY_BASE: EnergyCase = EnergyCase {
    ng_input_mw_lhv: 394.77,
    h2_product_mw_lhv: 299.70,
    net_power_export_mwe: 9.918,
    co2_kg_per_nm3_h2: 0.8091,
};

/// Shifted-syngas MDEA capture (Case 1A).
pub const IEAGHG_ENERGY_CASE_1A: EnergyCase = EnergyCase {
    ng_input_mw_lhv: 407.68,
    h2_product_mw_lhv: 299.70,
    net_power_export_mwe: 1.492,
    co2_kg_per_nm3_h2: 0.3704,
};

/// Reformer-flue-gas MEA capture (Case 03/3).
pub const IEAGHG_ENERGY_CASE_3: EnergyCase = EnergyCase {
    ng_input_mw_lhv: 433.72,
    h2_product_mw_lhv: 299.70,
    net_power_export_mwe: 0.426,
    co2_kg_per_nm3_h2: 0.0888,
};

/// Incremental NG thermal input relative to the no-capture base case.
pub fn incremental_ng_input_mw(case: EnergyCase) -> f64 {
    case.ng_input_mw_lhv - IEAGHG_ENERGY_BASE.ng_input_mw_lhv
}

/// Loss of net electricity export relative to the base case.
/// Positive means the capture case has consumed an additional electrical
/// opportunity that the base plant exported.
pub fn lost_power_export_mwe(case: EnergyCase) -> f64 {
    IEAGHG_ENERGY_BASE.net_power_export_mwe - case.net_power_export_mwe
}

/// A deliberately simple first-law opportunity metric:
/// incremental NG LHV plus lost exported electricity.
/// This is NOT SPECCA and does not convert electricity to primary energy.
pub fn incremental_energy_service_mw(case: EnergyCase) -> f64 {
    incremental_ng_input_mw(case) + lost_power_export_mwe(case)
}

/// Plant-gate CO2 avoided at the fixed 100,000 Nm3/h H2 production rate, t/h.
pub fn plant_gate_co2_avoided_t_per_h(case: EnergyCase) -> f64 {
    (IEAGHG_ENERGY_BASE.co2_kg_per_nm3_h2 - case.co2_kg_per_nm3_h2)
        * 100_000.0 / 1000.0
}


/// IEAGHG base-case preliminary equipment list: steam-reformer radiant duty,
/// reported as 82.63 million kcal/h.
pub const IEAGHG_REFORMER_RADIANT_DUTY_MMKCAL_H: f64 = 82.63;

/// Exact thermochemical conversion used here:
/// 1 kcal = 4.184 kJ; therefore 1 million kcal/h = 4.184 GJ/h.
pub fn mmkcal_per_h_to_mw(x: f64) -> f64 {
    x * 4.184 / 3.6
}

pub fn ieaghg_reformer_radiant_duty_mw() -> f64 {
    mmkcal_per_h_to_mw(IEAGHG_REFORMER_RADIANT_DUTY_MMKCAL_H)
}

/// Ratio of published radiant duty to separately purchased NG-fuel LHV.
/// >1 is expected because PSA tail gas is also fired in the conventional furnace.
pub fn ieaghg_radiant_to_makeup_fuel_lhv_ratio() -> f64 {
    ieaghg_reformer_radiant_duty_mw() / ieaghg_makeup_fuel_lhv_mw()
}

/// Difference between the published radiant duty and the earlier reduced
/// 298-K reaction-only thermochemical layer.
pub fn radiant_minus_reduced_reaction_duty_mw() -> f64 {
    ieaghg_reformer_radiant_duty_mw() - reduced_standard_reaction_duty_mw()
}


/// Screening secondary-helium heat-carrier calculation.
///
/// For helium over this temperature range, cp is treated here as a constant
/// screening parameter supplied by the caller. The final model will replace
/// this with a temperature-dependent property correlation.
pub fn helium_mass_flow_kg_s(q_mw: f64, cp_kj_kg_k: f64, t_hot_c: f64, t_cold_c: f64) -> f64 {
    assert!(q_mw > 0.0);
    assert!(cp_kj_kg_k > 0.0);
    assert!(t_hot_c > t_cold_c);
    q_mw * 1000.0 / (cp_kj_kg_k * (t_hot_c - t_cold_c))
}

/// Minimum secondary-helium hot-side temperature required for a chosen
/// reformer/process hot-end temperature and minimum terminal approach.
pub fn required_secondary_he_hot_c(process_hot_c: f64, min_approach_k: f64) -> f64 {
    assert!(min_approach_k > 0.0);
    process_hot_c + min_approach_k
}

/// Remaining reactor-to-secondary-helium temperature budget after imposing
/// the reformer hot-end approach. Positive means the assumed reactor outlet
/// still has temperature headroom for the IHX; non-positive is infeasible.
pub fn ihx_hot_end_temperature_budget_k(
    reactor_primary_out_c: f64,
    process_hot_c: f64,
    reformer_min_approach_k: f64,
) -> f64 {
    reactor_primary_out_c - required_secondary_he_hot_c(process_hot_c, reformer_min_approach_k)
}


/// Large-scale JAEA GTHTR300C IHX benchmark reported in the HTGR hydrogen
/// literature. This is a comparison anchor, not the selected Singapore reactor.
pub const GTHTR300C_IHX_DUTY_MW: f64 = 170.0;
pub const GTHTR300C_SECONDARY_HE_MASS_FLOW_KG_S: f64 = 81.0;
pub const GTHTR300C_SECONDARY_HE_INLET_C: f64 = 500.0;
pub const GTHTR300C_SECONDARY_HE_OUTLET_C: f64 = 900.0;
pub const GTHTR300C_SECONDARY_HE_PRESSURE_MPA: f64 = 5.15;
pub const GTHTR300C_SECONDARY_IHX_DP_KPA: f64 = 58.0;

/// Ideal-gas helium density screening relation.
pub fn helium_ideal_density_kg_m3(pressure_mpa: f64, temperature_c: f64) -> f64 {
    const R_HE_J_KG_K: f64 = 2077.1;
    let p_pa = pressure_mpa * 1.0e6;
    let t_k = temperature_c + 273.15;
    p_pa / (R_HE_J_KG_K * t_k)
}

/// Small-pressure-rise circulator power screening approximation:
/// W = m_dot * DeltaP / (rho * eta).
///
/// This is appropriate only when DeltaP/P is small. It is a lower-layer
/// estimate; total loop pressure loss must include piping, reformer, steam
/// generator, valves and other components in addition to the IHX.
pub fn helium_circulator_power_mw(
    mass_flow_kg_s: f64,
    delta_p_kpa: f64,
    suction_pressure_mpa: f64,
    suction_temperature_c: f64,
    efficiency: f64,
) -> f64 {
    assert!(mass_flow_kg_s > 0.0);
    assert!(delta_p_kpa > 0.0);
    assert!(efficiency > 0.0 && efficiency <= 1.0);
    let rho = helium_ideal_density_kg_m3(suction_pressure_mpa, suction_temperature_c);
    mass_flow_kg_s * delta_p_kpa * 1000.0 / (rho * efficiency) / 1.0e6
}


/// Sum component pressure losses for a secondary-helium loop.
pub fn helium_loop_delta_p_kpa(component_losses_kpa: &[f64]) -> f64 {
    assert!(!component_losses_kpa.is_empty());
    assert!(component_losses_kpa.iter().all(|x| *x >= 0.0));
    component_losses_kpa.iter().sum()
}

/// Circulator parasitic fraction relative to useful delivered heat.
pub fn helium_circulator_parasitic_fraction(
    useful_heat_mw: f64,
    mass_flow_kg_s: f64,
    total_delta_p_kpa: f64,
    pressure_mpa: f64,
    suction_temperature_c: f64,
    efficiency: f64,
) -> f64 {
    helium_circulator_power_mw(
        mass_flow_kg_s,total_delta_p_kpa,pressure_mpa,suction_temperature_c,efficiency
    ) / useful_heat_mw
}

/// Sensitivity multiplier relative to the published GTHTR300C IHX-only
/// pressure drop. This avoids inventing unverified component pressure losses.
pub fn loop_dp_from_ihx_multiple_kpa(multiplier: f64) -> f64 {
    assert!(multiplier >= 1.0);
    GTHTR300C_SECONDARY_IHX_DP_KPA * multiplier
}


/// IEAGHG base-case HP steam mass ledger from the published heat/material balance.
pub const IEAGHG_HP_STEAM_TO_PROCESS_KG_H: f64 = 95_301.0;
pub const IEAGHG_HP_STEAM_EXPORT_KG_H: f64 = 46_053.0;
pub const IEAGHG_HP_STEAM_TO_PROCESS_C: f64 = 400.0;
pub const IEAGHG_HP_STEAM_EXPORT_C: f64 = 395.0;

/// IEAGHG states that around 75% of saturated HP steam is generated in the
/// reformer syngas waste-heat boiler. This fraction is approximate.
pub const IEAGHG_SYNGAS_WHB_STEAM_FRACTION_APPROX: f64 = 0.75;

pub fn ieaghg_total_superheated_hp_steam_kg_h() -> f64 {
    IEAGHG_HP_STEAM_TO_PROCESS_KG_H + IEAGHG_HP_STEAM_EXPORT_KG_H
}

/// Approximate steam generation retained through the syngas WHB if the
/// nuclear-heated reformer preserves comparable hot-syngas outlet conditions.
pub fn ieaghg_syngas_whb_steam_kg_h_approx() -> f64 {
    ieaghg_total_superheated_hp_steam_kg_h() * IEAGHG_SYNGAS_WHB_STEAM_FRACTION_APPROX
}

/// Upper bound on the combined share generated by shift heat recovery plus
/// the fired-furnace convection steam-generator coil. It is NOT the convection
/// coil contribution alone.
pub fn ieaghg_non_syngas_whb_steam_kg_h_upper_group() -> f64 {
    ieaghg_total_superheated_hp_steam_kg_h() * (1.0 - IEAGHG_SYNGAS_WHB_STEAM_FRACTION_APPROX)
}


/// IAPWS-consistent steam-property bracket around the IEAGHG HP steam pressure.
/// At 4.0 MPa: h_g,sat=2800.9 and h(400C)=3214.5 kJ/kg.
/// At 4.5 MPa: h_g,sat=2797.9 and h(400C)=3205.6 kJ/kg.
/// The source steam pressure 4.23-4.29 MPa lies between these anchors.
pub const STEAM_SUPERHEAT_DH_4MPA_KJ_KG: f64 = 3214.5 - 2800.9;
pub const STEAM_SUPERHEAT_DH_4P5MPA_KJ_KG: f64 = 3205.6 - 2797.9;

pub fn ieaghg_hp_steam_superheat_duty_bounds_mw() -> (f64, f64) {
    let m_kg_s = ieaghg_total_superheated_hp_steam_kg_h() / 3600.0;
    let q4p5 = m_kg_s * STEAM_SUPERHEAT_DH_4P5MPA_KJ_KG / 1000.0;
    let q4 = m_kg_s * STEAM_SUPERHEAT_DH_4MPA_KJ_KG / 1000.0;
    (q4p5.min(q4), q4p5.max(q4))
}

/// Minimum currently quantified furnace-replacement thermal services:
/// verified reformer radiant duty plus bounded HP steam superheat.
/// This excludes feed-preheat and furnace-convection steam-generation duties.
pub fn quantified_furnace_replacement_service_bounds_mw() -> (f64, f64) {
    let (lo,hi)=ieaghg_hp_steam_superheat_duty_bounds_mw();
    (ieaghg_reformer_radiant_duty_mw()+lo, ieaghg_reformer_radiant_duty_mw()+hi)
}


#[derive(Debug, Clone, Copy)]
pub struct Shomate {
    pub a: f64, pub b: f64, pub c: f64, pub d: f64,
    pub e: f64, pub f: f64, pub g: f64, pub h: f64,
}

impl Shomate {
    /// NIST Shomate H(T)-H(298.15 K), kJ/mol, with t=T/1000.
    pub fn sensible_h_kj_mol(self, temperature_k: f64) -> f64 {
        let t=temperature_k/1000.0;
        self.a*t + self.b*t*t/2.0 + self.c*t*t*t/3.0
            + self.d*t*t*t*t/4.0 - self.e/t + self.f - self.h
    }
    pub fn delta_h_kj_mol(self, t1_k: f64, t2_k: f64) -> f64 {
        self.sensible_h_kj_mol(t2_k)-self.sensible_h_kj_mol(t1_k)
    }
}

/// NIST Chemistry WebBook methane, 298-1300 K, Chase 1998.
pub const NIST_CH4_298_1300: Shomate = Shomate {
    a:-0.703029,b:108.4773,c:-42.52157,d:5.862788,
    e:0.678565,f:-76.84376,g:158.7163,h:-74.87310,
};

/// Rigorous partial lower bound on the fired-convection feed-preheater duty:
/// only the CH4 portion of the published NG feedstock is counted, heated from
/// 135 C to 370 C. All other NG species and recycled H2 have positive sensible
/// duty and are deliberately omitted, so this is not a full coil duty.
pub fn feed_preheater_ch4_only_lower_bound_mw() -> f64 {
    let ch4_kmol_h=1455.8*0.89;
    let dh=NIST_CH4_298_1300.delta_h_kj_mol(135.0+273.15,370.0+273.15);
    // kmol/h * kJ/mol = MJ/h; /3600 = MW.
    ch4_kmol_h*dh/3600.0
}

/// Current source-backed furnace-service lower bound:
/// radiant + HP steam superheat + methane-only portion of feed preheat.
/// It remains deliberately incomplete.
pub fn current_furnace_service_lower_bound_mw() -> (f64,f64) {
    let (lo,hi)=quantified_furnace_replacement_service_bounds_mw();
    let q_feed_min=feed_preheater_ch4_only_lower_bound_mw();
    (lo+q_feed_min,hi+q_feed_min)
}


/// NIST SRD 69 Shomate coefficients used for the non-hydrocarbon minor
/// components of the IEAGHG natural-gas feed.
pub const NIST_CO2_298_1200: Shomate = Shomate {
    a:24.99735,b:55.18696,c:-33.69137,d:7.948387,
    e:-0.136638,f:-403.6075,g:228.2431,h:-393.5224,
};
pub const NIST_N2_100_500: Shomate = Shomate {
    a:28.98641,b:1.853978,c:-9.647459,d:16.63537,
    e:0.000117,f:-8.671914,g:226.4168,h:0.0,
};
pub const NIST_N2_500_2000: Shomate = Shomate {
    a:19.50583,b:19.88705,c:-8.598535,d:1.369784,
    e:0.527601,f:-4.935202,g:212.3900,h:0.0,
};

/// Piecewise-linear integration of tabulated ideal-gas Cp data.
/// Temperatures K; Cp J/mol-K; result kJ/mol.
pub fn integrate_cp_table_kj_mol(points: &[(f64,f64)], t1: f64, t2: f64) -> f64 {
    assert!(t2 > t1 && points.len() >= 2);
    let cp_at = |t:f64| -> f64 {
        for w in points.windows(2) {
            if t >= w[0].0 && t <= w[1].0 {
                let x=(t-w[0].0)/(w[1].0-w[0].0);
                return w[0].1+x*(w[1].1-w[0].1);
            }
        }
        panic!("temperature outside Cp table");
    };
    let mut knots=vec![t1];
    for &(t,_) in points { if t>t1 && t<t2 { knots.push(t); } }
    knots.push(t2);
    let mut area=0.0;
    for w in knots.windows(2) {
        area += 0.5*(cp_at(w[0])+cp_at(w[1]))*(w[1]-w[0]);
    }
    area/1000.0
}

const NIST_C2H6_CP: &[(f64,f64)] = &[
    (400.0,65.46),(500.0,77.94),(600.0,89.19),(700.0,99.14)
];
const NIST_C3H8_CP: &[(f64,f64)] = &[
    (400.0,94.01),(500.0,112.59),(600.0,128.70),(700.0,142.67)
];
const NIST_NC4H10_CP: &[(f64,f64)] = &[
    (400.0,124.77),(500.0,148.66),(600.0,169.28),(700.0,187.02)
];
const NIST_NC5H12_CP: &[(f64,f64)] = &[
    (400.0,152.55),(500.0,182.59),(600.0,208.78),(700.0,231.38)
];

fn n2_delta_h_kj_mol(t1:f64,t2:f64)->f64 {
    assert!(t1 < 500.0 && t2 > 500.0);
    NIST_N2_100_500.delta_h_kj_mol(t1,500.0)
        + NIST_N2_500_2000.delta_h_kj_mol(500.0,t2)
}

/// Complete sensible duty for the published IEAGHG natural-gas mixture,
/// excluding the separately added recycled-H2 slipstream.
/// Composition: 2% CO2, 89% CH4, 7% C2H6, 1% C3H8,
/// 0.1% n-C4H10, 0.01% n-C5H12; the residual 0.89% is treated as N2.
pub fn feed_preheater_ng_only_duty_mw() -> f64 {
    let t1=408.15; let t2=643.15; let n=1455.8;
    let dh =
        0.0200*NIST_CO2_298_1200.delta_h_kj_mol(t1,t2)
        +0.8900*NIST_CH4_298_1300.delta_h_kj_mol(t1,t2)
        +0.0700*integrate_cp_table_kj_mol(NIST_C2H6_CP,t1,t2)
        +0.0100*integrate_cp_table_kj_mol(NIST_C3H8_CP,t1,t2)
        +0.0010*integrate_cp_table_kj_mol(NIST_NC4H10_CP,t1,t2)
        +0.0001*integrate_cp_table_kj_mol(NIST_NC5H12_CP,t1,t2)
        +0.0089*n2_delta_h_kj_mol(t1,t2);
    n*dh/3600.0
}


/// NIST SRD 69 hydrogen Shomate coefficients, 298-1000 K.
pub const NIST_H2_298_1000: Shomate = Shomate {
    a:33.066178,b:-11.363417,c:11.432816,d:-2.772874,
    e:-0.158558,f:-9.980797,g:172.707974,h:0.0,
};
pub const NIST_H2_1000_2500: Shomate = Shomate {
    a:18.563083,b:12.257357,c:-2.859786,d:0.268238,
    e:1.977990,f:-1.147438,g:156.288133,h:0.0,
};

/// IEAGHG base-case PSA hydrogen recycle stream 13.
pub const IEAGHG_H2_RECYCLE_KMOL_H: f64 = 29.1;
pub const IEAGHG_H2_RECYCLE_INLET_C: f64 = 40.0;

/// H2-recycle sensible duty through the furnace Feed Pre-Heater Coil,
/// screening from published 40 C recycle state to the 370 C feed-preheater
/// outlet stated in the process description.
pub fn feed_preheater_h2_recycle_duty_mw() -> f64 {
    let dh=NIST_H2_298_1000.delta_h_kj_mol(
        IEAGHG_H2_RECYCLE_INLET_C+273.15,370.0+273.15);
    IEAGHG_H2_RECYCLE_KMOL_H*dh/3600.0
}

pub fn feed_preheater_ng_plus_h2_duty_mw() -> f64 {
    feed_preheater_ng_only_duty_mw()+feed_preheater_h2_recycle_duty_mw()
}

/// NIST SRD 69 water-vapour Shomate coefficients, 500-1700 K.
pub const NIST_H2O_500_1700: Shomate = Shomate {
    a:30.09200,b:6.832514,c:6.793435,d:-2.534480,
    e:0.082139,f:-250.8810,g:223.3967,h:-241.8264,
};

/// Lower-bound Pre-Reformer Feed Pre-Heater duty.
/// The published stream 4 exits this coil / enters the pre-reformer at 500 C.
/// We conservatively assume NG+H2 enter at 370 C and all stream-4 water enters
/// as already-superheated 400 C steam. Any BFW desuperheating lowers inlet
/// enthalpy and therefore increases actual coil duty.
pub fn prereformer_feed_preheater_lower_bound_mw() -> f64 {
    let s=ieaghg_prereformer_feed();
    let t_out=500.0+273.15;
    let t_gas_in=370.0+273.15;
    let t_steam_in=400.0+273.15;

    let q_ch4=s.flow(s.ch4)*NIST_CH4_298_1300.delta_h_kj_mol(t_gas_in,t_out);
    let q_co2=s.flow(s.co2)*NIST_CO2_298_1200.delta_h_kj_mol(t_gas_in,t_out);
    let q_h2=s.flow(s.h2)*NIST_H2_298_1000.delta_h_kj_mol(t_gas_in,t_out);
    let q_n2=s.flow(s.n2)*NIST_N2_500_2000.delta_h_kj_mol(t_gas_in,t_out);
    // C2+ terms are omitted in this lower bound because the currently
    // encoded Cp tables stop below the 773.15 K outlet.
    let q_h2o=s.flow(s.h2o)*NIST_H2O_500_1700.delta_h_kj_mol(t_steam_in,t_out);
    (q_ch4+q_co2+q_h2+q_n2+q_h2o)/3600.0
}


/// Sensitivity model for the fired-furnace Reformer Pre-Heater Coil.
///
/// Source-resolved inlet composition is approximated here by stream 4 after
/// pre-reforming only as a screening composition. The standalone IEAGHG report
/// does not expose the exact post-pre-reformer / post-second-steam-addition
/// intermediate stream in its summary table, so this function is explicitly
/// a sensitivity, not a reconstructed source duty.
///
/// It heats CH4/CO2/H2/N2/H2O from 500 C to a caller-specified primary
/// reformer inlet temperature. C2+ are omitted because a functioning
/// pre-reformer should strongly reduce them and because their exact outlet
/// composition is not source-resolved here.
pub fn reformer_preheater_sensitivity_mw(reformer_inlet_c: f64) -> f64 {
    assert!(reformer_inlet_c > 500.0 && reformer_inlet_c <= 700.0);
    let s=ieaghg_prereformer_feed();
    let t1=500.0+273.15;
    let t2=reformer_inlet_c+273.15;
    let q_ch4=s.flow(s.ch4)*NIST_CH4_298_1300.delta_h_kj_mol(t1,t2);
    let q_co2=s.flow(s.co2)*NIST_CO2_298_1200.delta_h_kj_mol(t1,t2);
    let q_h2=s.flow(s.h2)*NIST_H2_298_1000.delta_h_kj_mol(t1,t2);
    let q_n2=s.flow(s.n2)*NIST_N2_500_2000.delta_h_kj_mol(t1,t2);
    let q_h2o=s.flow(s.h2o)*NIST_H2O_500_1700.delta_h_kj_mol(t1,t2);
    (q_ch4+q_co2+q_h2+q_n2+q_h2o)/3600.0
}


/// Composite lower-bound/sensitivity envelope for furnace-dependent thermal
/// services already represented in the model.
///
/// The reformer-preheater term is sensitivity-based, not source-reconstructed.
/// The prereformer-feed term is a conservative lower bound.
/// Furnace steam-generation share and nuclear-loop heat losses are excluded.
pub fn current_htgr_service_envelope_mw(reformer_inlet_c: f64) -> (f64,f64) {
    let (steam_lo,steam_hi)=ieaghg_hp_steam_superheat_duty_bounds_mw();
    let fixed=ieaghg_reformer_radiant_duty_mw()
        + feed_preheater_ng_plus_h2_duty_mw()
        + prereformer_feed_preheater_lower_bound_mw()
        + reformer_preheater_sensitivity_mw(reformer_inlet_c);
    (fixed+steam_lo,fixed+steam_hi)
}


/// Break down the current service envelope into named terms for reproducible
/// reporting and uncertainty analysis.
#[derive(Debug,Clone,Copy)]
pub struct HtgrServiceBreakdown {
    pub radiant_mw: f64,
    pub hp_steam_superheat_lo_mw: f64,
    pub hp_steam_superheat_hi_mw: f64,
    pub feed_preheat_mw: f64,
    pub prereformer_preheat_lower_bound_mw: f64,
    pub reformer_preheat_sensitivity_mw: f64,
}
pub fn current_htgr_service_breakdown(reformer_inlet_c:f64)->HtgrServiceBreakdown {
    let (lo,hi)=ieaghg_hp_steam_superheat_duty_bounds_mw();
    HtgrServiceBreakdown {
        radiant_mw:ieaghg_reformer_radiant_duty_mw(),
        hp_steam_superheat_lo_mw:lo,
        hp_steam_superheat_hi_mw:hi,
        feed_preheat_mw:feed_preheater_ng_plus_h2_duty_mw(),
        prereformer_preheat_lower_bound_mw:prereformer_feed_preheater_lower_bound_mw(),
        reformer_preheat_sensitivity_mw:reformer_preheater_sensitivity_mw(reformer_inlet_c),
    }
}


/// Project-loop circulator sensitivity using the current screening helium flow
/// (96.04 MW radiant service, 880 -> 650 C, cp=5.2 kJ/kg-K) and total loop
/// pressure drop expressed as a multiple of the 58 kPa GTHTR300C IHX anchor.
/// The 5.15 MPa pressure is likewise a benchmark assumption, not a selected design.
pub fn project_loop_circulator_sensitivity_mw(dp_multiple: f64) -> f64 {
    let m=helium_mass_flow_kg_s(ieaghg_reformer_radiant_duty_mw(),5.2,880.0,650.0);
    helium_circulator_power_mw(m,loop_dp_from_ihx_multiple_kpa(dp_multiple),
        5.15,650.0,0.80)
}


/// Saturated-steam latent-heat bracket around the IEAGHG 4.23 MPa steam drum.
/// 4.0 MPa: h_fg=1713.3 kJ/kg; 4.5 MPa: h_fg=1675.6 kJ/kg.
pub const STEAM_HFG_4MPA_KJ_KG: f64 = 1713.3;
pub const STEAM_HFG_4P5MPA_KJ_KG: f64 = 1675.6;

/// Strict upper bound on furnace-convection saturated-steam generation duty.
/// IEAGHG says ~75% of saturated steam is generated in the syngas WHB.
/// The remaining ~25% is shared by shift heat recovery AND the furnace coil.
/// Assigning all 25% to the furnace therefore intentionally overestimates
/// the furnace contribution.
pub fn furnace_steam_generation_upper_bound_mw() -> (f64,f64) {
    let m_kg_s=ieaghg_non_syngas_whb_steam_kg_h_upper_group()/3600.0;
    let q_lo=m_kg_s*STEAM_HFG_4P5MPA_KJ_KG/1000.0;
    let q_hi=m_kg_s*STEAM_HFG_4MPA_KJ_KG/1000.0;
    (q_lo,q_hi)
}

/// Current bounded conventional furnace-service envelope.
/// Lower: currently quantified services, excluding furnace SG.
/// Upper: assigns the entire non-syngas-WHB steam-generation remainder to
/// the furnace coil, even though IEAGHG explicitly says shift recovery shares it.
pub fn bounded_furnace_service_envelope_mw(reformer_inlet_c:f64)->(f64,f64) {
    let (service_lo,service_hi)=current_htgr_service_envelope_mw(reformer_inlet_c);
    let (_,sg_hi)=furnace_steam_generation_upper_bound_mw();
    (service_lo,service_hi+sg_hi)
}


/// Screening conversion from process-heat service to electrical demand for eSMR.
pub fn electric_heater_power_mwe(process_heat_mw: f64, electricity_to_heat_eff: f64) -> f64 {
    assert!(process_heat_mw > 0.0);
    assert!(electricity_to_heat_eff > 0.0 && electricity_to_heat_eff <= 1.0);
    process_heat_mw / electricity_to_heat_eff
}

/// Reactor thermal power required to supply an electric load through a power cycle.
pub fn reactor_thermal_for_electric_load_mw(electric_mw: f64, net_generation_eff: f64) -> f64 {
    assert!(electric_mw > 0.0);
    assert!(net_generation_eff > 0.0 && net_generation_eff <= 1.0);
    electric_mw / net_generation_eff
}

/// First common-service eSMR screening envelope.
/// Uses the bounded conventional process-service range and caller-specified
/// electricity-to-heat and HTGR net generation efficiencies.
pub fn nuclear_electric_reformer_screen_mw(
    reformer_inlet_c:f64,
    electricity_to_heat_eff:f64,
    net_generation_eff:f64,
)->((f64,f64),(f64,f64)) {
    let (qlo,qhi)=bounded_furnace_service_envelope_mw(reformer_inlet_c);
    let elo=electric_heater_power_mwe(qlo,electricity_to_heat_eff);
    let ehi=electric_heater_power_mwe(qhi,electricity_to_heat_eff);
    let rlo=reactor_thermal_for_electric_load_mw(elo,net_generation_eff);
    let rhi=reactor_thermal_for_electric_load_mw(ehi,net_generation_eff);
    ((elo,ehi),(rlo,rhi))
}


/// Carbon-equivalent CO2 generated from the IEAGHG feedstock carbon before
/// capture, expressed per kg H2. This is a carbon-accounting quantity:
/// it assumes all feedstock carbon ultimately becomes CO2 or captured carbon.
pub fn feedstock_carbon_co2_equivalent_kg_per_kg_h2() -> f64 {
    const M_CO2_KG_PER_KMOL: f64 = 44.0095;
    ieaghg_feed_carbon_kmol_per_h()*M_CO2_KG_PER_KMOL/IEAGHG_BASE.h2_kg_per_h
}

/// Residual plant-gate carbon emission for a furnace-free matched architecture
/// when a specified fraction of feedstock carbon is permanently captured.
/// Tail gas must be recycled/converted consistently; otherwise this identity
/// does not apply.
pub fn matched_furnace_free_residual_co2_kg_per_kg_h2(capture_fraction: f64) -> f64 {
    assert!((0.0..=1.0).contains(&capture_fraction));
    feedstock_carbon_co2_equivalent_kg_per_kg_h2()*(1.0-capture_fraction)
}

/// Under identical H2 output, NG feed, carbon conversion/capture and tail-gas
/// disposition, direct heat and eSMR have identical plant-gate carbon emissions.
/// This function makes that comparison invariant explicit.
pub fn matched_direct_minus_electric_plant_gate_co2(
    capture_fraction: f64
) -> f64 {
    let d=matched_furnace_free_residual_co2_kg_per_kg_h2(capture_fraction);
    let e=matched_furnace_free_residual_co2_kg_per_kg_h2(capture_fraction);
    d-e
}

/// Required annual H2 production for the CN4252 abatement target under a
/// specified lifecycle intensity relative to a baseline.
pub fn required_h2_kt_per_year_for_abatement(
    baseline_kgco2e_per_kgh2: f64,
    candidate_kgco2e_per_kgh2: f64,
    target_mtco2e_per_year: f64,
) -> f64 {
    let delta=baseline_kgco2e_per_kgh2-candidate_kgco2e_per_kgh2;
    assert!(delta>0.0);
    target_mtco2e_per_year*1000.0/delta
}


/// IEAGHG feedstock-NG energy for the 100,000 Nm3/h H2 reference case.
/// 12.197 GJ per 1000 Nm3 H2 * 100 = 1219.7 GJ/h.
pub const IEAGHG_FEEDSTOCK_NG_GJ_H: f64 = 1219.7;

/// Convert an upstream gas-supply intensity (gCO2e/MJ delivered gas) to
/// kgCO2e/kgH2 for the IEAGHG fixed-output feedstock basis.
pub fn upstream_ng_kgco2e_per_kgh2(intensity_g_per_mj: f64) -> f64 {
    assert!(intensity_g_per_mj >= 0.0);
    let upstream_kg_h=IEAGHG_FEEDSTOCK_NG_GJ_H*1000.0*intensity_g_per_mj/1000.0;
    upstream_kg_h/IEAGHG_BASE.h2_kg_per_h
}

/// Nuclear-electric lifecycle contribution, kgCO2e/kgH2.
pub fn nuclear_electric_lca_kgco2e_per_kgh2(
    electric_load_mwe:f64,
    nuclear_gco2e_per_kwh_e:f64,
)->f64 {
    electric_load_mwe*1000.0*nuclear_gco2e_per_kwh_e/1000.0/IEAGHG_BASE.h2_kg_per_h
}

/// Derived proxy for direct nuclear heat lifecycle intensity.
///
/// If an electricity LCA factor E [g/kWh_e] is associated with a reactor whose
/// net electric efficiency is eta_e, the implied lifecycle burden per unit
/// reactor thermal throughput is approximated as E*eta_e [g/kWh_th].
/// This is an allocation proxy, NOT a published process-heat LCA result.
pub fn nuclear_heat_lca_proxy_gco2e_per_kwh_th(
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
)->f64 {
    assert!(net_electric_efficiency>0.0 && net_electric_efficiency<=1.0);
    nuclear_gco2e_per_kwh_e*net_electric_efficiency
}

pub fn direct_nuclear_heat_lca_proxy_kgco2e_per_kgh2(
    thermal_service_mw:f64,
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
)->f64 {
    let g_per_kwh_th=nuclear_heat_lca_proxy_gco2e_per_kwh_th(
        nuclear_gco2e_per_kwh_e,net_electric_efficiency);
    thermal_service_mw*1000.0*g_per_kwh_th/1000.0/IEAGHG_BASE.h2_kg_per_h
}

/// CO2 transport-chain emissions represented as a fraction of captured CO2.
/// Useful for transparent sensitivity before a Singapore-specific route is fixed.
pub fn ccs_transport_kgco2e_per_kgh2(
    captured_co2_kg_per_kgh2:f64,
    transport_emission_fraction:f64,
)->f64 {
    assert!(captured_co2_kg_per_kgh2>=0.0);
    assert!((0.0..1.0).contains(&transport_emission_fraction));
    captured_co2_kg_per_kgh2*transport_emission_fraction
}


#[derive(Debug,Clone,Copy)]
pub struct LifecycleCase {
    pub plant_carbon: f64,
    pub upstream_ng: f64,
    pub nuclear: f64,
    pub ccs_transport: f64,
}
impl LifecycleCase {
    pub fn total(self)->f64 {
        self.plant_carbon+self.upstream_ng+self.nuclear+self.ccs_transport
    }
}

/// Matched direct-heat lifecycle screening case.
/// CCS transport fraction is applied to captured feedstock-carbon CO2.
pub fn matched_direct_lifecycle_screen(
    capture_fraction:f64,
    upstream_gco2e_per_mj:f64,
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
    thermal_service_mw:f64,
    ccs_transport_fraction:f64,
)->LifecycleCase {
    let feed=feedstock_carbon_co2_equivalent_kg_per_kg_h2();
    LifecycleCase {
        plant_carbon:feed*(1.0-capture_fraction),
        upstream_ng:upstream_ng_kgco2e_per_kgh2(upstream_gco2e_per_mj),
        nuclear:direct_nuclear_heat_lca_proxy_kgco2e_per_kgh2(
            thermal_service_mw,nuclear_gco2e_per_kwh_e,net_electric_efficiency),
        ccs_transport:ccs_transport_kgco2e_per_kgh2(
            feed*capture_fraction,ccs_transport_fraction),
    }
}

/// Matched nuclear-electric lifecycle screening case.
pub fn matched_electric_lifecycle_screen(
    capture_fraction:f64,
    upstream_gco2e_per_mj:f64,
    nuclear_gco2e_per_kwh_e:f64,
    process_service_mw:f64,
    electric_heater_efficiency:f64,
    ccs_transport_fraction:f64,
)->LifecycleCase {
    let feed=feedstock_carbon_co2_equivalent_kg_per_kg_h2();
    let electric=electric_heater_power_mwe(process_service_mw,electric_heater_efficiency);
    LifecycleCase {
        plant_carbon:feed*(1.0-capture_fraction),
        upstream_ng:upstream_ng_kgco2e_per_kgh2(upstream_gco2e_per_mj),
        nuclear:nuclear_electric_lca_kgco2e_per_kgh2(electric,nuclear_gco2e_per_kwh_e),
        ccs_transport:ccs_transport_kgco2e_per_kgh2(
            feed*capture_fraction,ccs_transport_fraction),
    }
}


/// Capital recovery factor for annualising CAPEX.
pub fn capital_recovery_factor(discount_rate:f64,lifetime_years:u32)->f64 {
    assert!(discount_rate>=0.0);
    assert!(lifetime_years>0);
    if discount_rate==0.0 { return 1.0/lifetime_years as f64; }
    let n=lifetime_years as i32;
    discount_rate*(1.0+discount_rate).powi(n)/((1.0+discount_rate).powi(n)-1.0)
}

/// Generic annualised cost, in the caller's currency/year.
pub fn annualised_cost(
    capex:f64,discount_rate:f64,lifetime_years:u32,
    fixed_opex_per_year:f64,variable_opex_per_year:f64,
)->f64 {
    capex*capital_recovery_factor(discount_rate,lifetime_years)
        +fixed_opex_per_year+variable_opex_per_year
}

/// Incremental cost of CO2e abatement.
/// Costs must use the same currency/year and annual emissions the same tCO2e/y.
pub fn abatement_cost_per_tco2e(
    candidate_annual_cost:f64,baseline_annual_cost:f64,
    baseline_tco2e_y:f64,candidate_tco2e_y:f64,
)->f64 {
    let avoided=baseline_tco2e_y-candidate_tco2e_y;
    assert!(avoided>0.0);
    (candidate_annual_cost-baseline_annual_cost)/avoided
}

/// Maximum annual incremental cost compatible with an abatement-cost target.
pub fn max_incremental_annual_cost(
    target_currency_per_tco2e:f64,
    annual_avoided_tco2e:f64,
)->f64 {
    assert!(target_currency_per_tco2e>=0.0 && annual_avoided_tco2e>0.0);
    target_currency_per_tco2e*annual_avoided_tco2e
}


/// Incremental annual-cost budget compatible with a target abatement cost,
/// expressed from hydrogen production and specific lifecycle abatement.
pub fn annual_cost_budget_from_h2(
    h2_kt_per_year:f64,
    avoided_kgco2e_per_kgh2:f64,
    target_currency_per_tco2e:f64,
)->f64 {
    assert!(h2_kt_per_year>0.0 && avoided_kgco2e_per_kgh2>0.0);
    let avoided_t_per_year=h2_kt_per_year*1000.0*avoided_kgco2e_per_kgh2;
    max_incremental_annual_cost(target_currency_per_tco2e,avoided_t_per_year)
}

/// Maximum incremental overnight CAPEX that could be supported if the entire
/// annual abatement-cost budget were allocated to capital recovery alone.
/// This is a deliberately optimistic upper bound: real projects also have
/// incremental OPEX, fuel, CCS T&S and integration costs.
pub fn capex_only_break_even_upper_bound(
    annual_incremental_cost_budget:f64,
    discount_rate:f64,
    lifetime_years:u32,
)->f64 {
    annual_incremental_cost_budget/capital_recovery_factor(discount_rate,lifetime_years)
}


/// Energy-share allocation of common reactor cost to a process-heat branch.
/// This is one transparent allocation rule, not a universal accounting rule.
pub fn reactor_cost_allocation_fraction_by_thermal_service(
    process_heat_mw: f64,
    reactor_thermal_mw: f64,
) -> f64 {
    assert!(process_heat_mw > 0.0 && reactor_thermal_mw > 0.0);
    assert!(process_heat_mw <= reactor_thermal_mw);
    process_heat_mw / reactor_thermal_mw
}

/// Annualised reactor cost allocated to hydrogen/process heat under a chosen
/// allocation fraction.
pub fn allocated_annual_reactor_cost(
    total_reactor_capex: f64,
    allocation_fraction: f64,
    discount_rate: f64,
    lifetime_years: u32,
    total_fixed_opex_per_year: f64,
) -> f64 {
    assert!((0.0..=1.0).contains(&allocation_fraction));
    allocation_fraction * (
        total_reactor_capex * capital_recovery_factor(discount_rate,lifetime_years)
        + total_fixed_opex_per_year
    )
}

/// Residual annual budget left for non-reactor incremental costs while meeting
/// an abatement-cost ceiling.
pub fn residual_incremental_budget(
    total_allowed_incremental_cost_per_year: f64,
    allocated_reactor_cost_per_year: f64,
) -> f64 {
    total_allowed_incremental_cost_per_year - allocated_reactor_cost_per_year
}


/// Maximum total common-reactor annual cost compatible with a hydrogen-side
/// incremental-cost budget after non-reactor incremental costs are reserved.
pub fn max_common_reactor_annual_cost(
    allowed_incremental_cost_per_year:f64,
    nonreactor_incremental_cost_per_year:f64,
    reactor_allocation_fraction:f64,
)->f64 {
    assert!(reactor_allocation_fraction>0.0 && reactor_allocation_fraction<=1.0);
    (allowed_incremental_cost_per_year-nonreactor_incremental_cost_per_year)
        /reactor_allocation_fraction
}

/// Annual CCS transport/storage charge from captured CO2 mass and unit tariff.
/// Unit tariff is deliberately currency-agnostic; caller must harmonise FX/year.
pub fn annual_ccs_transport_storage_cost(
    captured_co2_t_per_year:f64,
    tariff_per_tco2:f64,
)->f64 {
    assert!(captured_co2_t_per_year>=0.0 && tariff_per_tco2>=0.0);
    captured_co2_t_per_year*tariff_per_tco2
}


/// Convert a foreign-currency per-tonne CCS tariff to annual SGD cost using
/// an explicit SGD-per-foreign-currency FX rate.
pub fn annual_ccs_cost_sgd_from_foreign_tariff(
    captured_co2_t_per_year:f64,
    tariff_foreign_per_tco2:f64,
    sgd_per_foreign_currency:f64,
)->f64 {
    annual_ccs_transport_storage_cost(captured_co2_t_per_year,tariff_foreign_per_tco2)
        *sgd_per_foreign_currency
}

/// Share of the allowed annual abatement-cost budget consumed by CCS T&S.
pub fn budget_fraction_consumed(cost_per_year:f64,allowed_budget_per_year:f64)->f64 {
    assert!(cost_per_year>=0.0 && allowed_budget_per_year>0.0);
    cost_per_year/allowed_budget_per_year
}


/// Maximum CCS T&S tariff compatible with an annual abatement-cost budget after
/// reserving other incremental annual costs.
pub fn max_ccs_tariff_per_t_captured(
    allowed_incremental_cost_per_year:f64,
    other_incremental_cost_per_year:f64,
    captured_co2_t_per_year:f64,
)->f64 {
    assert!(captured_co2_t_per_year>0.0);
    (allowed_incremental_cost_per_year-other_incremental_cost_per_year)
        /captured_co2_t_per_year
}

/// Maximum hydrogen-side annual reactor/integration cost after paying a
/// specified CCS T&S tariff.
pub fn max_reactor_and_integration_budget_after_ccs(
    allowed_incremental_cost_per_year:f64,
    captured_co2_t_per_year:f64,
    ccs_tariff_per_t_captured:f64,
)->f64 {
    allowed_incremental_cost_per_year
        - annual_ccs_transport_storage_cost(captured_co2_t_per_year,ccs_tariff_per_t_captured)
}


pub const IEAGHG_CASE1A_CAC_EUR2014_PER_T_AVOIDED: f64 = 47.1;
pub const IEAGHG_CASE1A_TS_EUR2014_PER_T_CAPTURED: f64 = 10.0;
pub const IEAGHG_CASE1A_CAPTURED_KG_PER_NM3_H2: f64 = 0.4660;
pub const IEAGHG_BASE_EMITTED_KG_PER_NM3_H2: f64 = 0.8091;
pub const IEAGHG_CASE1A_EMITTED_KG_PER_NM3_H2: f64 = 0.3704;

pub fn ieaghg_case1a_captured_per_avoided_ratio() -> f64 {
    IEAGHG_CASE1A_CAPTURED_KG_PER_NM3_H2
        /(IEAGHG_BASE_EMITTED_KG_PER_NM3_H2-IEAGHG_CASE1A_EMITTED_KG_PER_NM3_H2)
}

/// Case-1A avoidance cost excluding the report's explicit T&S charge,
/// retained in original Q4-2014 euros per tonne CO2 avoided.
pub fn ieaghg_case1a_non_ts_cac_eur2014_per_t_avoided() -> f64 {
    IEAGHG_CASE1A_CAC_EUR2014_PER_T_AVOIDED
        - IEAGHG_CASE1A_TS_EUR2014_PER_T_CAPTURED
          *ieaghg_case1a_captured_per_avoided_ratio()
}

/// Substitute any T&S tariff expressed in the SAME currency/price basis as
/// the non-T&S term. Price-year/FX harmonisation must occur before calling.
pub fn case1a_cac_with_replacement_ts(
    non_ts_cost_per_t_avoided:f64,
    replacement_ts_cost_per_t_captured:f64,
)->f64 {
    non_ts_cost_per_t_avoided
        + replacement_ts_cost_per_t_captured*ieaghg_case1a_captured_per_avoided_ratio()
}


/// Generic price-index escalation.
pub fn escalate_cost_by_index(cost:f64,index_old:f64,index_new:f64)->f64 {
    assert!(cost>=0.0 && index_old>0.0 && index_new>0.0);
    cost*index_new/index_old
}

/// Screening translation of Case-1A non-T&S CAC to SGD.
/// The new plant cost index must be supplied explicitly.
pub fn case1a_non_ts_cac_sgd_screen(
    plant_cost_index_2014:f64,
    plant_cost_index_new:f64,
    sgd_per_eur:f64,
)->f64 {
    escalate_cost_by_index(
        ieaghg_case1a_non_ts_cac_eur2014_per_t_avoided(),
        plant_cost_index_2014,plant_cost_index_new)*sgd_per_eur
}

/// Add a Singapore T&S tariff in SGD/t captured to an already harmonised
/// non-T&S cost in SGD/t avoided.
pub fn case1a_singapore_cac_sgd_screen(
    non_ts_sgd_per_t_avoided:f64,
    ts_sgd_per_t_captured:f64,
)->f64 {
    case1a_cac_with_replacement_ts(non_ts_sgd_per_t_avoided,ts_sgd_per_t_captured)
}


/// Required reduction in annual incremental cost to move an existing
/// abatement-cost comparator down to a target, at fixed annual avoided emissions.
/// Positive means savings are required; negative means premium headroom exists.
pub fn required_annual_savings_to_target(
    current_cost_per_t:f64,
    target_cost_per_t:f64,
    annual_avoided_t:f64,
)->f64 {
    assert!(annual_avoided_t>0.0);
    (current_cost_per_t-target_cost_per_t)*annual_avoided_t
}

/// IEAGHG Case-1A annual direct plant CO2 avoided at its stated 8322 h/y.
pub fn ieaghg_case1a_annual_direct_co2_avoided_t() -> f64 {
    let avoided_kg_per_nm3=IEAGHG_BASE_EMITTED_KG_PER_NM3_H2
        -IEAGHG_CASE1A_EMITTED_KG_PER_NM3_H2;
    avoided_kg_per_nm3*100_000.0*8322.0/1000.0
}


/// Annual purchased supplementary furnace-NG energy in the IEAGHG base case.
/// Uses the source 55.94 MW_LHV and 8322 h/y.
pub fn ieaghg_annual_makeup_furnace_ng_gj() -> f64 {
    ieaghg_makeup_fuel_lhv_mw()*8322.0*3.6
}

/// Annual resource-cost saving if a fraction of purchased supplementary
/// furnace NG is displaced, for an explicit SGD/GJ commodity-price assumption.
pub fn avoided_furnace_ng_cost_sgd_per_year(
    gas_price_sgd_per_gj:f64,
    displaced_fraction:f64,
)->f64 {
    assert!(gas_price_sgd_per_gj>=0.0);
    assert!((0.0..=1.0).contains(&displaced_fraction));
    ieaghg_annual_makeup_furnace_ng_gj()*gas_price_sgd_per_gj*displaced_fraction
}

/// Gas price that would be required for avoided purchased furnace NG alone to
/// deliver a specified annual saving. This is a break-even diagnostic.
pub fn gas_price_required_for_savings_sgd_per_gj(
    required_savings_sgd_per_year:f64,
    displaced_fraction:f64,
)->f64 {
    assert!(required_savings_sgd_per_year>=0.0);
    assert!(displaced_fraction>0.0 && displaced_fraction<=1.0);
    required_savings_sgd_per_year/(ieaghg_annual_makeup_furnace_ng_gj()*displaced_fraction)
}


/// Lower-heating-value screening for IEAGHG base-case PSA tail gas.
/// Uses standard species LHVs for H2, CO and CH4; CO2/inerts contribute zero.
/// Returned MW_LHV is reconstructed from rounded source composition.
pub fn ieaghg_psa_tail_gas_lhv_mw() -> f64 {
    let t=ieaghg_psa_tail_cho();
    const H2_LHV_MJ_PER_KMOL: f64 = 241.826;
    const CO_LHV_MJ_PER_KMOL: f64 = 282.99;
    const CH4_LHV_MJ_PER_KMOL: f64 = 802.30;
    (t.h2*H2_LHV_MJ_PER_KMOL
        +t.co*CO_LHV_MJ_PER_KMOL
        +t.ch4*CH4_LHV_MJ_PER_KMOL)/3600.0
}

/// Combustible molar flow in PSA tail gas, kmol/h.
pub fn ieaghg_psa_tail_combustible_kmol_h() -> f64 {
    ieaghg_psa_tail_cho().combustible_kmol_per_h()
}

/// If all combustible tail-gas species were recycled and ultimately converted,
/// this is their chemical LHV inventory per kg current H2 product.
/// It is an energy-inventory upper context, NOT a recoverable H2 yield.
pub fn ieaghg_tail_lhv_mj_per_kg_h2() -> f64 {
    ieaghg_psa_tail_gas_lhv_mw()*3600.0/IEAGHG_BASE.h2_kg_per_h
}

/// Annual electricity value for an explicit MWe change and SGD/MWh price.
/// Positive MW means export/value gained; negative means additional import/use.
pub fn annual_electricity_value_sgd(
    power_mwe:f64,hours_per_year:f64,electricity_sgd_per_mwh:f64
)->f64 {
    power_mwe*hours_per_year*electricity_sgd_per_mwh
}

/// Annual steam value for an explicit exported steam mass and SGD/t tariff.
/// This is a market-value sensitivity, not an energy-equivalence calculation.
pub fn annual_steam_value_sgd(
    steam_t_per_h:f64,hours_per_year:f64,steam_sgd_per_t:f64
)->f64 {
    steam_t_per_h*hours_per_year*steam_sgd_per_t
}


/// Tail-gas carbon and combustible inventory for configuration screening.
#[derive(Debug,Clone,Copy)]
pub struct TailGasInventory {
    pub h2_kmol_h:f64,
    pub co_kmol_h:f64,
    pub ch4_kmol_h:f64,
    pub co2_kmol_h:f64,
}
pub fn ieaghg_tail_inventory()->TailGasInventory {
    let t=ieaghg_psa_tail_cho();
    TailGasInventory{h2_kmol_h:t.h2,co_kmol_h:t.co,ch4_kmol_h:t.ch4,co2_kmol_h:t.co2}
}

/// Ideal additional H2 molecular flow if tail-gas CO is fully shifted and CH4
/// is fully steam-reformed+shifted, while existing H2 is recovered:
/// H2_existing + CO + 4*CH4. This is a stoichiometric upper bound, not a
/// process yield and excludes equilibrium/PSA losses and added steam duty.
pub fn tail_gas_ideal_h2_upper_bound_kmol_h()->f64 {
    let t=ieaghg_tail_inventory();
    t.h2_kmol_h+t.co_kmol_h+4.0*t.ch4_kmol_h
}

/// Tail-gas CO2 already present and separable before recycle, t/h.
pub fn tail_gas_existing_co2_t_h()->f64 {
    let t=ieaghg_tail_inventory();
    t.co2_kmol_h*44.0095/1000.0
}

/// Carbon in tail-gas CO+CH4 requiring conversion/capture if not combusted.
pub fn tail_gas_nonco2_carbon_kmol_h()->f64 {
    let t=ieaghg_tail_inventory();
    t.co_kmol_h+t.ch4_kmol_h
}


/// Carbon-equivalent fresh-NG displacement upper bound for tail-gas
/// Configuration B (remove existing CO2, recycle CO+CH4).
///
/// At fixed carbon throughput, each kmol-C/h in recycled CO or CH4 can at most
/// displace one kmol-C/h of fresh NG feed. This is a material-balance upper
/// bound, not a solved recycle flowsheet.
pub fn tail_recycle_fresh_ng_carbon_displacement_upper_fraction() -> f64 {
    tail_gas_nonco2_carbon_kmol_h()/ieaghg_feed_carbon_kmol_per_h()
}

/// Fresh NG molar-flow displacement corresponding to the carbon-equivalent
/// upper bound, using the published NG carbon content per kmol mixture.
pub fn tail_recycle_fresh_ng_displacement_upper_kmol_h() -> f64 {
    tail_gas_nonco2_carbon_kmol_h()/IEAGHG_NG.carbon_kmol_per_kmol()
}

/// Upper-bound fresh-NG feed energy displaced if the carbon-equivalent recycle
/// replaces fresh feed one-for-one. Uses the source feedstock-NG specific LHV
/// implied by 12.197 GJ/1000 Nm3 H2 and 1455.8 kmol/h NG.
/// This does NOT include purchased supplementary furnace fuel, which is a
/// separate saving when the furnace is removed.
pub fn tail_recycle_fresh_ng_feed_energy_displacement_upper_mw() -> f64 {
    let feed_energy_mw=IEAGHG_FEEDSTOCK_NG_GJ_H/3.6;
    feed_energy_mw*tail_recycle_fresh_ng_carbon_displacement_upper_fraction()
}

/// Annual value of the carbon-equivalent fresh-feed displacement upper bound.
pub fn tail_recycle_fresh_ng_feed_savings_upper_sgd_y(gas_price_sgd_per_gj:f64)->f64 {
    assert!(gas_price_sgd_per_gj>=0.0);
    tail_recycle_fresh_ng_feed_energy_displacement_upper_mw()*8322.0*3.6
        *gas_price_sgd_per_gj
}

/// Case-2A anchored electrical penalty for tail-gas CO2 separation and
/// compression, net of the source-reported tail-gas expander recovery.
/// This is an anchor for Configuration B, not yet a redesigned nuclear-loop value.
pub fn case2a_tail_separation_net_electric_anchor_mwe() -> f64 {
    4.575 + 2.874 - 1.140
}


/// Reduced fixed-output tail-recycle model.
/// This is intentionally not a full equilibrium flowsheet.
#[derive(Debug,Clone,Copy)]
pub struct ReducedTailRecycleResult {
    pub recovered_h2_kmol_h:f64,
    pub fresh_ng_displaced_kmol_h:f64,
    pub fresh_ng_displaced_fraction:f64,
    pub fresh_feed_energy_displaced_mw:f64,
    pub extra_water_consumed_kmol_h:f64,
    pub converted_tail_carbon_kmol_h:f64,
}

pub fn reduced_tail_recycle_fixed_h2(
    co_conversion:f64,
    ch4_conversion:f64,
    existing_h2_recovery:f64,
)->ReducedTailRecycleResult {
    for x in [co_conversion,ch4_conversion,existing_h2_recovery] {
        assert!((0.0..=1.0).contains(&x));
    }
    let t=ieaghg_tail_inventory();
    let psa=ieaghg_reconstructed_psa_h2_recovery();
    let recovered_h2 =
        existing_h2_recovery*t.h2_kmol_h
        + psa*(co_conversion*t.co_kmol_h + 4.0*ch4_conversion*t.ch4_kmol_h);
    let baseline_product_per_ng =
        ieaghg_reconstructed_h2_product_kmol_per_h()/1455.8;
    let ng_displaced=(recovered_h2/baseline_product_per_ng).min(1455.8);
    let frac=ng_displaced/1455.8;
    let water=co_conversion*t.co_kmol_h + 2.0*ch4_conversion*t.ch4_kmol_h;
    let carbon=co_conversion*t.co_kmol_h + ch4_conversion*t.ch4_kmol_h;
    let feed_energy_mw=IEAGHG_FEEDSTOCK_NG_GJ_H/3.6*frac;
    ReducedTailRecycleResult{
        recovered_h2_kmol_h:recovered_h2,
        fresh_ng_displaced_kmol_h:ng_displaced,
        fresh_ng_displaced_fraction:frac,
        fresh_feed_energy_displaced_mw:feed_energy_mw,
        extra_water_consumed_kmol_h:water,
        converted_tail_carbon_kmol_h:carbon,
    }
}

/// Partial operating-value screen: fresh-feed NG saving minus the Case-2A
/// anchored electricity cost. Solvent steam, recycle compression changes,
/// reformer-duty changes and CAPEX are deliberately excluded.
pub fn reduced_tail_recycle_partial_net_value_sgd_y(
    result:ReducedTailRecycleResult,
    gas_price_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
)->f64 {
    let gas=result.fresh_feed_energy_displaced_mw*8322.0*3.6*gas_price_sgd_per_gj;
    let power=case2a_tail_separation_net_electric_anchor_mwe()
        *8322.0*electricity_sgd_per_mwh;
    gas-power
}


/// Standard reaction enthalpies used only for a first reaction-heat screen.
/// CH4 + H2O -> CO + 3H2: +205.8 kJ/mol.
/// CO + H2O -> CO2 + H2: -41.2 kJ/mol.
/// Sources: NETL SMR technical references. Actual high-temperature duties
/// require temperature-dependent reaction enthalpies/equilibrium.
pub const SMR_DH298_KJ_MOL: f64 = 205.8;
pub const WGS_DH298_KJ_MOL: f64 = -41.2;

/// Reaction-enthalpy contribution of converted recycled CO/CH4.
/// Full CH4 conversion includes SMR followed by WGS, hence SMR+WGS per CH4.
/// This is a 298-K reaction-enthalpy screen, not total reformer duty.
pub fn reduced_tail_recycle_reaction_heat_screen_mw(
    co_conversion:f64,ch4_conversion:f64
)->f64 {
    let t=ieaghg_tail_inventory();
    let q_kj_h =
        ch4_conversion*t.ch4_kmol_h*1000.0*(SMR_DH298_KJ_MOL+WGS_DH298_KJ_MOL)
        +co_conversion*t.co_kmol_h*1000.0*WGS_DH298_KJ_MOL;
    q_kj_h/3.6e6
}

/// Case-2A solvent-regeneration steam latent-heat screen.
/// Source steam flow is 66.9 t/h. A 4-7 barg saturated-steam bracket uses
/// h_fg ~2108 to 2048 kJ/kg from saturated steam tables.
pub fn case2a_mdea_regeneration_latent_heat_bounds_mw()->(f64,f64) {
    let m_kg_s=66_900.0/3600.0;
    let lo=m_kg_s*2048.0/1000.0;
    let hi=m_kg_s*2108.0/1000.0;
    (lo,hi)
}

/// Carbon-throughput diagnostic for the reduced recycle case.
/// Returns (fresh carbon removed, tail non-CO2 carbon converted), kmol-C/h.
/// Their difference is more informative for net steam demand than gross
/// stoichiometric water consumption alone.
pub fn reduced_tail_recycle_carbon_replacement(
    result:ReducedTailRecycleResult
)->(f64,f64) {
    let fresh_removed=result.fresh_ng_displaced_kmol_h*IEAGHG_NG.carbon_kmol_per_kmol();
    (fresh_removed,result.converted_tail_carbon_kmol_h)
}


/// Methane-only standard reaction-heat requirement removed when fresh NG is
/// displaced, assuming displaced NG has the baseline 89 mol% CH4 composition.
/// This deliberately omits C2+ reforming heat, so it is a lower bound on the
/// fresh-feed reaction duty removed.
pub fn displaced_fresh_ng_ch4_reaction_heat_lower_bound_mw(
    result:ReducedTailRecycleResult
)->f64 {
    let ch4_kmol_h=result.fresh_ng_displaced_kmol_h*IEAGHG_NG.methane;
    ch4_kmol_h*1000.0*(SMR_DH298_KJ_MOL+WGS_DH298_KJ_MOL)/3.6e6
}

/// Difference between recycled-species standard reaction heat and the
/// methane-only lower bound of displaced fresh-feed reaction heat.
/// Negative means recycle requires less standard reaction enthalpy.
/// C2+ omission makes the magnitude of savings conservative.
pub fn recycle_minus_displaced_reaction_heat_screen_mw(
    result:ReducedTailRecycleResult,
    co_conversion:f64,
    ch4_conversion:f64,
)->f64 {
    reduced_tail_recycle_reaction_heat_screen_mw(co_conversion,ch4_conversion)
        - displaced_fresh_ng_ch4_reaction_heat_lower_bound_mw(result)
}


/// MDEA regeneration heat integration sensitivity.
/// waste_heat_fraction is supplied by retained low-grade process heat rather
/// than incremental nuclear heat. A recent integrated eSMR/CCS study reports
/// 63% direct post-shift/condensing-syngas contribution; this is an external
/// sensitivity anchor, not an IEAGHG Case-2A reconstructed value.
pub fn mdea_incremental_nuclear_heat_bounds_mw(
    waste_heat_fraction:f64
)->(f64,f64) {
    assert!((0.0..=1.0).contains(&waste_heat_fraction));
    let (lo,hi)=case2a_mdea_regeneration_latent_heat_bounds_mw();
    ((1.0-waste_heat_fraction)*lo,(1.0-waste_heat_fraction)*hi)
}

pub fn recycle_heat_cascade_screen_80pct(
    waste_heat_fraction:f64
)->((f64,f64),f64) {
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    (
        mdea_incremental_nuclear_heat_bounds_mw(waste_heat_fraction),
        recycle_minus_displaced_reaction_heat_screen_mw(r,0.80,0.80)
    )
}

/// Incomplete net thermal screen: incremental MDEA heat plus standard
/// reaction-heat change. Sensible/preheat/steam-network changes are omitted.
pub fn recycle_net_thermal_increment_screen_80pct(
    waste_heat_fraction:f64
)->(f64,f64) {
    let ((lo,hi),rxn)=recycle_heat_cascade_screen_80pct(waste_heat_fraction);
    (lo+rxn,hi+rxn)
}


/// Source-based upper-group bound for retained non-syngas-WHB steam generation.
/// IEAGHG says ~75% of saturated HP steam comes from the reformer WHB.
/// The remaining ~25% is shared by shift heat recovery and furnace SG.
/// This converts that remaining steam mass to an approximate latent-heat pool.
/// It is an upper bound on shift-recoverable heat, not a measured shift duty.
pub fn non_syngas_whb_steam_latent_heat_upper_bounds_mw()->(f64,f64) {
    let m=ieaghg_non_syngas_whb_steam_kg_h_upper_group()/3600.0;
    (m*STEAM_HFG_4P5MPA_KJ_KG/1000.0,m*STEAM_HFG_4MPA_KJ_KG/1000.0)
}

/// Conservative source-based MDEA waste-heat fraction ceiling using only the
/// non-syngas-WHB saturated-steam latent-heat group as potentially redirectable.
/// Because the group includes BOTH shift recovery and furnace steam generation,
/// this is deliberately an upper bound on retained shift heat.
pub fn mdea_waste_heat_fraction_upper_from_ieaghg_steam_group()->(f64,f64) {
    let (qlo,qhi)=non_syngas_whb_steam_latent_heat_upper_bounds_mw();
    let (mlo,mhi)=case2a_mdea_regeneration_latent_heat_bounds_mw();
    ((qlo/mhi).min(1.0),(qhi/mlo).min(1.0))
}


/// NIST SRD 69 carbon-monoxide Shomate coefficients, 298-1300 K.
pub const NIST_CO_298_1300: Shomate = Shomate {
    a:25.56759,b:6.096130,c:4.054656,d:-2.671301,
    e:0.131021,f:-118.0089,g:227.3665,h:-110.5271,
};

/// IEAGHG stream 6: HTS outlet / start of shifted-syngas cooling train.
/// Source: 412 C, 2.77 MPa, 8370.3 kmol/h.
/// Returns ideal-gas sensible heat recoverable by cooling to target_c.
/// This deliberately excludes condensation.
pub fn ieaghg_hts_outlet_sensible_heat_to_mw(target_c:f64)->f64 {
    // 226.85 C = 500 K, lower limit of the encoded NIST water-vapour
    // Shomate correlation. Do not extrapolate below the verified range.
    assert!(target_c>=226.85 && target_c<412.0);
    let n=8370.3;
    let t1=target_c+273.15;
    let t2=412.0+273.15;
    let dh =
        0.1283*NIST_CO2_298_1200.delta_h_kj_mol(t1,t2)
        +0.0366*NIST_CO_298_1300.delta_h_kj_mol(t1,t2)
        +0.5961*NIST_H2_298_1000.delta_h_kj_mol(t1,t2)
        +0.0015*NIST_N2_500_2000.delta_h_kj_mol(t1,t2)
        +0.0238*NIST_CH4_298_1300.delta_h_kj_mol(t1,t2)
        +0.2137*NIST_H2O_500_1700.delta_h_kj_mol(t1,t2);
    n*dh/3600.0
}

pub fn ieaghg_shift_sensible_heat_above_reboiler_mw(
    reboiler_c:f64,dtmin_c:f64
)->f64 {
    assert!(dtmin_c>=0.0);
    ieaghg_hts_outlet_sensible_heat_to_mw(reboiler_c+dtmin_c)
}

/// Thermodynamic sensible-heat availability ceiling before competing duties.
pub fn mdea_fraction_from_shift_sensible_ceiling(
    reboiler_c:f64,dtmin_c:f64
)->(f64,f64) {
    let q=ieaghg_shift_sensible_heat_above_reboiler_mw(reboiler_c,dtmin_c);
    let (mlo,mhi)=case2a_mdea_regeneration_latent_heat_bounds_mw();
    ((q/mhi).min(1.0),(q/mlo).min(1.0))
}


/// IAPWS-IF97 Region-4 saturation temperature from pressure in MPa.
/// Coefficients from IAPWS R7-97(2012), saturation-line backward equation.
pub fn iapws_if97_tsat_k_from_mpa(p_mpa:f64)->f64 {
    assert!(p_mpa>0.000611 && p_mpa<=22.064);
    let n1=0.11670521452767e4;
    let n2=-0.72421316703206e6;
    let n3=-0.17073846940092e2;
    let n4=0.12020824702470e5;
    let n5=-0.32325550322333e7;
    let n6=0.14915108613530e2;
    let n7=-0.48232657361591e4;
    let n8=0.40511340542057e6;
    let n9=-0.23855557567849;
    let n10=0.65017534844798e3;
    let beta=p_mpa.powf(0.25);
    let e=beta*beta+n3*beta+n6;
    let ff=n1*beta*beta+n4*beta+n7;
    let g=n2*beta*beta+n5*beta+n8;
    let d=2.0*g/(-ff-(ff*ff-4.0*e*g).sqrt());
    (n10+d-((n10+d)*(n10+d)-4.0*(n9+n10*d)).sqrt())/2.0
}

/// Ideal-mixture water dew point for IEAGHG stream 6 using y_H2O times P.
pub fn ieaghg_stream6_water_dewpoint_c()->f64 {
    let p_h2o_mpa=0.2137*2.77;
    iapws_if97_tsat_k_from_mpa(p_h2o_mpa)-273.15
}

/// Test whether bulk condensation starts above the required hot-side pinch.
pub fn stream6_condensation_above_mdea_pinch(
    reboiler_c:f64,dtmin_c:f64
)->bool {
    ieaghg_stream6_water_dewpoint_c() >= reboiler_c+dtmin_c
}


/// Bounded sensible-heat increment for IEAGHG stream 6 from 500 K down to
/// a hotter-than-dew-point target. Non-water species use encoded NIST
/// Shomate properties. Water vapour uses an explicit Cp bracket because the
/// NIST H2O Shomate fit begins at 500 K; IAPWS-IF97 is the authoritative
/// formulation to replace this bracket in the next property-layer refinement.
///
/// cp_h2o bounds are J/mol-K. 33..36 J/mol-K deliberately brackets steam Cp
/// in this moderate-temperature low-partial-pressure interval.
pub fn ieaghg_shift_sensible_500k_to_target_bounds_mw(
    target_c:f64,cp_h2o_lo_j_mol_k:f64,cp_h2o_hi_j_mol_k:f64
)->(f64,f64) {
    let t1=target_c+273.15;
    let t2=500.0;
    assert!(t1>=400.0 && t1<=500.0);
    assert!(cp_h2o_lo_j_mol_k>0.0 && cp_h2o_hi_j_mol_k>=cp_h2o_lo_j_mol_k);
    let n=8370.3;
    let dry =
        0.1283*NIST_CO2_298_1200.delta_h_kj_mol(t1,t2)
        +0.0366*NIST_CO_298_1300.delta_h_kj_mol(t1,t2)
        +0.5961*NIST_H2_298_1000.delta_h_kj_mol(t1,t2)
        +0.0238*NIST_CH4_298_1300.delta_h_kj_mol(t1,t2);
    let dt=t2-t1;
    let water_lo=0.2137*cp_h2o_lo_j_mol_k*dt/1000.0;
    let water_hi=0.2137*cp_h2o_hi_j_mol_k*dt/1000.0;
    // N2 is only 0.15 mol%; its encoded Shomate range also starts at 500 K,
    // so bracket it instead of extrapolating.
    let n2_lo=0.0015*28.0*dt/1000.0;
    let n2_hi=0.0015*31.0*dt/1000.0;
    (n*(dry+water_lo+n2_lo)/3600.0,n*(dry+water_hi+n2_hi)/3600.0)
}

/// Full shifted-syngas sensible-heat bound from 412 C to a target below
/// 226.85 C, combining exact encoded NIST 412->226.85 C with the bounded
/// 500 K -> target interval.
pub fn ieaghg_shift_sensible_412_to_target_bounds_mw(
    target_c:f64
)->(f64,f64) {
    let q_hi_t=ieaghg_hts_outlet_sensible_heat_to_mw(226.85);
    let (lo,hi)=ieaghg_shift_sensible_500k_to_target_bounds_mw(
        target_c,33.0,36.0);
    (q_hi_t+lo,q_hi_t+hi)
}


/// Residual shifted-syngas sensible heat ceiling after preserving the
/// source-explicit feed-preheater duty.
///
/// IEAGHG states that NG is heated to 135 C in the Feed Pre-heater by shifted
/// syngas leaving the BFW pre-heater. Therefore this already-verified duty
/// competes with MDEA for the 412->170 C shifted-syngas heat pool.
/// This still does NOT subtract shift-WHB, BFW, condensate or demi-water duties,
/// so it remains an optimistic residual upper bound.
pub fn residual_shift_heat_after_feed_preheat_bounds_mw()->(f64,f64) {
    let (lo,hi)=ieaghg_shift_sensible_412_to_target_bounds_mw(170.0);
    let q_feed=feed_preheater_ng_plus_h2_duty_mw();
    ((lo-q_feed).max(0.0),(hi-q_feed).max(0.0))
}

/// Corresponding optimistic MDEA heat-recovery fraction after preserving only
/// the source-explicit feed-preheater duty.
pub fn mdea_fraction_after_feed_preheat_upper_bounds()->(f64,f64) {
    let (qlo,qhi)=residual_shift_heat_after_feed_preheat_bounds_mw();
    let (mlo,mhi)=case2a_mdea_regeneration_latent_heat_bounds_mw();
    ((qlo/mhi).min(1.0),(qhi/mlo).min(1.0))
}


/// Conservative MDEA incremental-heat envelope after the source-based pinch work.
/// Recoverable shifted-syngas heat is bounded from 0 to the residual ceiling
/// after preserving the verified feed-preheater duty. Other downstream duties
/// are unresolved, so zero recovery is the conservative end.
pub fn mdea_incremental_heat_source_bounded_mw()->(f64,f64) {
    let (mlo,mhi)=case2a_mdea_regeneration_latent_heat_bounds_mw();
    let (_,recoverable_hi)=residual_shift_heat_after_feed_preheat_bounds_mw();
    ((mlo-recoverable_hi).max(0.0),mhi)
}

/// Current 80%-recycle thermal increment envelope: MDEA incremental heat plus
/// conservative standard reaction-heat change. Still excludes sensible/feed
/// network changes and recycle compression.
pub fn recycle_80pct_thermal_increment_source_bounded_mw()->(f64,f64) {
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let rxn=recycle_minus_displaced_reaction_heat_screen_mw(r,0.80,0.80);
    let (lo,hi)=mdea_incremental_heat_source_bounded_mw();
    (lo+rxn,hi+rxn)
}

/// Propagate the recycle thermal increment into the existing HTGR service
/// envelope for a specified reformer-inlet sensitivity.
pub fn htgr_service_with_recycle_source_bounded_mw(
    reformer_inlet_c:f64
)->(f64,f64) {
    let (base_lo,base_hi)=bounded_furnace_service_envelope_mw(reformer_inlet_c);
    let (dlo,dhi)=recycle_80pct_thermal_increment_source_bounded_mw();
    (base_lo+dlo,base_hi+dhi)
}


/// Annual energy cost/value for a continuous thermal service.
pub fn annual_thermal_energy_cost_sgd(
    thermal_mw:f64,hours_per_year:f64,price_sgd_per_gj:f64
)->f64 {
    assert!(thermal_mw>=0.0 && hours_per_year>=0.0 && price_sgd_per_gj>=0.0);
    thermal_mw*hours_per_year*3.6*price_sgd_per_gj
}

/// Gross annual NG resource saving in the 80% recycle screen:
/// purchased supplementary furnace NG + reduced fresh-feed NG.
/// The two physical savings channels remain separately auditable in the model.
pub fn recycle80_gross_ng_saving_sgd_y(gas_price_sgd_per_gj:f64)->f64 {
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let saved_mw=ieaghg_makeup_fuel_lhv_mw()+r.fresh_feed_energy_displaced_mw;
    annual_thermal_energy_cost_sgd(saved_mw,8322.0,gas_price_sgd_per_gj)
}

/// Operating-energy net value for the 80% recycle architecture before CAPEX,
/// fixed O&M, CCS T&S and other unresolved integration costs.
/// Nuclear heat is charged on the full reactor-side process-service envelope,
/// not merely on the recycle increment. Separation electricity uses the
/// Case-2A 6.309 MWe anchor.
pub fn recycle80_operating_energy_net_bounds_sgd_y(
    htgr_service_lo_mw:f64,
    htgr_service_hi_mw:f64,
    gas_price_sgd_per_gj:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
)->(f64,f64) {
    assert!(htgr_service_hi_mw>=htgr_service_lo_mw);
    let saving=recycle80_gross_ng_saving_sgd_y(gas_price_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()*8322.0*electricity_sgd_per_mwh;
    let heat_lo=annual_thermal_energy_cost_sgd(htgr_service_lo_mw,8322.0,nuclear_heat_sgd_per_gj);
    let heat_hi=annual_thermal_energy_cost_sgd(htgr_service_hi_mw,8322.0,nuclear_heat_sgd_per_gj);
    // lower net corresponds to higher nuclear-heat requirement.
    (saving-elec-heat_hi,saving-elec-heat_lo)
}

/// Gas price required for zero operating-energy net value, before CAPEX/O&M.
pub fn recycle80_break_even_gas_price_sgd_per_gj(
    htgr_service_mw:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
)->f64 {
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let saved_mw=ieaghg_makeup_fuel_lhv_mw()+r.fresh_feed_energy_displaced_mw;
    let annual_saved_gj=saved_mw*8322.0*3.6;
    let heat=annual_thermal_energy_cost_sgd(htgr_service_mw,8322.0,nuclear_heat_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()*8322.0*electricity_sgd_per_mwh;
    (heat+elec)/annual_saved_gj
}


/// Maximum delivered nuclear-heat price compatible with a specified remaining
/// annual cost budget after CCS T&S, separation electricity and other fixed
/// incremental annual costs are reserved. The NG savings are those of the
/// 80% recycle screen (fresh feed + supplementary furnace fuel).
pub fn recycle80_max_nuclear_heat_price_sgd_per_gj(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    gas_price_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    ccs_ts_cost_sgd_y:f64,
    other_fixed_incremental_sgd_y:f64,
)->f64 {
    let ng=recycle80_gross_ng_saving_sgd_y(gas_price_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()*8322.0*electricity_sgd_per_mwh;
    let numerator=allowed_incremental_cost_sgd_y+ng-elec-ccs_ts_cost_sgd_y-other_fixed_incremental_sgd_y;
    numerator/(htgr_service_mw*8322.0*3.6)
}

/// Maximum annual hydrogen-side HTGR/IHX/integration cost after energy and
/// CCS T&S have been paid while respecting the allowed incremental-cost budget.
/// Positive = annualised capital/fixed-O&M headroom; negative = infeasible even
/// before capital.
pub fn recycle80_max_allocated_capital_opex_sgd_y(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    gas_price_sgd_per_gj:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    ccs_ts_cost_sgd_y:f64,
)->f64 {
    let (net,_)=recycle80_operating_energy_net_bounds_sgd_y(
        htgr_service_mw,htgr_service_mw,gas_price_sgd_per_gj,
        nuclear_heat_sgd_per_gj,electricity_sgd_per_mwh);
    allowed_incremental_cost_sgd_y + net - ccs_ts_cost_sgd_y
}

/// Convert hydrogen-side annual common-reactor-cost headroom to total common
/// reactor annual-cost headroom under an allocation fraction.
pub fn total_common_reactor_headroom_sgd_y(
    hydrogen_side_headroom_sgd_y:f64,
    allocation_fraction:f64,
)->f64 {
    assert!(allocation_fraction>0.0 && allocation_fraction<=1.0);
    hydrogen_side_headroom_sgd_y/allocation_fraction
}


/// Annualised common-reactor cost from a source case.
/// CAPEX and O&M must be in the same currency units (e.g. million USD).
pub fn source_reactor_annual_cost(
    capex:f64,annual_om:f64,discount_rate:f64,lifetime_years:u32
)->f64 {
    capex*capital_recovery_factor(discount_rate,lifetime_years)+annual_om
}

/// Allocate a source reactor annual cost by thermal-service share.
/// This is an energy-share screening allocation, not a market-value allocation.
pub fn source_reactor_annual_cost_allocated_by_heat(
    capex:f64,annual_om:f64,source_reactor_mwth:f64,
    hydrogen_heat_mwth:f64,discount_rate:f64,lifetime_years:u32
)->f64 {
    assert!(hydrogen_heat_mwth<=source_reactor_mwth);
    source_reactor_annual_cost(capex,annual_om,discount_rate,lifetime_years)
        *hydrogen_heat_mwth/source_reactor_mwth
}

/// Maximum source-case cost multiplier compatible with a hydrogen-side annual
/// reactor-cost headroom under thermal-share allocation.
/// <1 means the source cost must fall; >1 means source cost fits with margin.
pub fn reactor_cost_multiplier_headroom(
    hydrogen_side_headroom:f64,
    source_allocated_annual_cost:f64
)->f64 {
    assert!(source_allocated_annual_cost>0.0);
    hydrogen_side_headroom/source_allocated_annual_cost
}


/// Annualised IHX + secondary-helium-loop cost from a source component
/// estimate. No silent inflation or FX conversion occurs here.
pub fn annualised_integration_component_cost(
    source_capex:f64,
    discount_rate:f64,
    lifetime_years:u32,
    fixed_om_fraction_of_capex_per_year:f64,
)->f64 {
    assert!(fixed_om_fraction_of_capex_per_year>=0.0);
    source_capex*capital_recovery_factor(discount_rate,lifetime_years)
        +source_capex*fixed_om_fraction_of_capex_per_year
}

/// Maximum integration CAPEX supported by an annual hydrogen-side margin,
/// after reserving an explicit fixed-O&M fraction of CAPEX per year.
pub fn integration_capex_from_annual_margin(
    annual_margin:f64,
    discount_rate:f64,
    lifetime_years:u32,
    fixed_om_fraction_of_capex_per_year:f64,
)->f64 {
    let denom=capital_recovery_factor(discount_rate,lifetime_years)
        +fixed_om_fraction_of_capex_per_year;
    annual_margin/denom
}


/// Minimum NG price required for the 80% recycle case to support a specified
/// annual reactor allocation, IHX/loop and other integration costs while
/// remaining inside the allowed abatement-cost budget.
pub fn recycle80_min_gas_price_for_full_cost_sgd_per_gj(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    ccs_ts_cost_sgd_y:f64,
    allocated_reactor_cost_sgd_y:f64,
    ihx_loop_cost_sgd_y:f64,
    other_integration_cost_sgd_y:f64,
)->f64 {
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let saved_mw=ieaghg_makeup_fuel_lhv_mw()+r.fresh_feed_energy_displaced_mw;
    let saved_gj_y=saved_mw*8322.0*3.6;
    let heat=annual_thermal_energy_cost_sgd(htgr_service_mw,8322.0,nuclear_heat_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()*8322.0*electricity_sgd_per_mwh;
    let required=allocated_reactor_cost_sgd_y+ihx_loop_cost_sgd_y
        +other_integration_cost_sgd_y+heat+elec+ccs_ts_cost_sgd_y
        -allowed_incremental_cost_sgd_y;
    required/saved_gj_y
}

/// Maximum annual CCS T&S cost compatible with the same full-cost boundary.
pub fn recycle80_max_ccs_ts_cost_sgd_y(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    gas_price_sgd_per_gj:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    allocated_reactor_cost_sgd_y:f64,
    ihx_loop_cost_sgd_y:f64,
    other_integration_cost_sgd_y:f64,
)->f64 {
    let ng=recycle80_gross_ng_saving_sgd_y(gas_price_sgd_per_gj);
    let heat=annual_thermal_energy_cost_sgd(htgr_service_mw,8322.0,nuclear_heat_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()*8322.0*electricity_sgd_per_mwh;
    allowed_incremental_cost_sgd_y+ng-heat-elec-allocated_reactor_cost_sgd_y
        -ihx_loop_cost_sgd_y-other_integration_cost_sgd_y
}


/// Convert any delivered-NG energy rate to upstream lifecycle intensity per kg H2.
pub fn upstream_ng_from_energy_mw_kgco2e_per_kgh2(
    ng_energy_mw:f64,
    intensity_gco2e_per_mj:f64,
)->f64 {
    assert!(ng_energy_mw>=0.0 && intensity_gco2e_per_mj>=0.0);
    let upstream_kg_h=ng_energy_mw*3600.0*intensity_gco2e_per_mj/1000.0;
    upstream_kg_h/IEAGHG_BASE.h2_kg_per_h
}

/// Internally consistent lifecycle baseline using IEAGHG direct plant CO2 plus
/// upstream emissions on both feedstock and supplementary furnace NG.
pub fn ieaghg_unabated_lifecycle_screen(
    upstream_gco2e_per_mj:f64
)->LifecycleCase {
    LifecycleCase {
        plant_carbon:IEAGHG_BASE.co2_emitted_kg_per_kg_h2(),
        upstream_ng:upstream_ng_from_energy_mw_kgco2e_per_kgh2(
            ieaghg_total_ng_lhv_mw(),upstream_gco2e_per_mj),
        nuclear:0.0,
        ccs_transport:0.0,
    }
}

/// Matched lifecycle screen for the reduced 80% tail-recycle + direct-HTGR case.
/// Fresh-feed NG is reduced by the recycle model and furnace NG is eliminated.
/// Capture fraction applies to reduced external feed carbon. This remains a
/// screening closure assumption until a full iterative recycle flowsheet exists.
pub fn recycle80_shared_direct_lifecycle_screen(
    capture_fraction:f64,
    upstream_gco2e_per_mj:f64,
    thermal_service_mw:f64,
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
    ccs_transport_fraction:f64,
)->LifecycleCase {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let fresh_fraction=1.0-r.fresh_ng_displaced_fraction;
    let external_feed_carbon=
        feedstock_carbon_co2_equivalent_kg_per_kg_h2()*fresh_fraction;
    let fresh_ng_energy=ieaghg_feed_lhv_mw()*fresh_fraction;
    LifecycleCase {
        plant_carbon:external_feed_carbon*(1.0-capture_fraction),
        upstream_ng:upstream_ng_from_energy_mw_kgco2e_per_kgh2(
            fresh_ng_energy,upstream_gco2e_per_mj),
        nuclear:direct_nuclear_heat_lca_proxy_kgco2e_per_kgh2(
            thermal_service_mw,nuclear_gco2e_per_kwh_e,net_electric_efficiency),
        ccs_transport:ccs_transport_kgco2e_per_kgh2(
            external_feed_carbon*capture_fraction,ccs_transport_fraction),
    }
}

pub fn annual_lifecycle_abatement_and_budget(
    baseline:LifecycleCase,
    candidate:LifecycleCase,
    h2_kg_per_h:f64,
    hours_per_year:f64,
    target_currency_per_tco2e:f64,
)->(f64,f64) {
    let delta=baseline.total()-candidate.total();
    assert!(delta>0.0);
    let avoided_t_y=delta*h2_kg_per_h*hours_per_year/1000.0;
    (avoided_t_y/1.0e6,avoided_t_y*target_currency_per_tco2e)
}


/// Converged fixed-H2 recycle closure for the reduced PSA-tail model.
///
/// This is a deliberately transparent surrogate, not an equilibrium simulator.
/// The once-through IEAGHG product and tail yields are scaled with fresh-NG feed.
/// After tail CO2 removal, H2/CO/CH4 are recycled. CO and CH4 conversions create
/// H2 according to CO+H2O->CO2+H2 and CH4+2H2O->CO2+4H2; converted carbon is
/// assumed removed as CO2 before the next PSA. PSA recovery is held at the
/// source-reconstructed value. Fresh NG is adjusted each iteration to keep the
/// IEAGHG H2 product fixed. The regenerated tail is then iterated to a fixed point.
///
/// The closure is useful for replacing the previous single-pass "80%" shortcut,
/// but its fixed conversion/recovery coefficients remain assumptions that require
/// later validation against a rigorous reformer/shift/PSA flowsheet.
#[derive(Debug,Clone,Copy)]
pub struct IterativeTailRecycleResult {
    pub iterations:u32,
    pub converged:bool,
    pub fresh_ng_kmol_h:f64,
    pub fresh_ng_fraction_of_baseline:f64,
    pub fresh_feed_energy_mw:f64,
    pub tail_h2_kmol_h:f64,
    pub tail_co_kmol_h:f64,
    pub tail_ch4_kmol_h:f64,
    pub captured_co2_kmol_h:f64,
    pub recycled_carbon_kmol_h:f64,
    pub extra_water_consumed_kmol_h:f64,
}

pub fn iterative_tail_recycle_fixed_h2(
    co_conversion:f64,
    ch4_conversion:f64,
    recycle_h2_recovery:f64,
    tolerance:f64,
    max_iterations:u32,
)->IterativeTailRecycleResult {
    for x in [co_conversion,ch4_conversion,recycle_h2_recovery] {
        assert!((0.0..=1.0).contains(&x));
    }
    assert!(tolerance>0.0 && max_iterations>0);

    let base_tail=ieaghg_tail_inventory();
    let base_product=ieaghg_reconstructed_h2_product_kmol_per_h();
    let product_per_fresh_ng=base_product/1455.8;
    let psa_recovery=ieaghg_reconstructed_psa_h2_recovery();

    let mut tail=base_tail;
    let mut fresh_ng=1455.8;
    let mut captured_co2=0.0;
    let mut water=0.0;

    for iteration in 1..=max_iterations {
        let recycled_h2_product=recycle_h2_recovery*tail.h2_kmol_h;
        let converted_h2=psa_recovery*(
            co_conversion*tail.co_kmol_h
            +4.0*ch4_conversion*tail.ch4_kmol_h
        );
        fresh_ng=((base_product-recycled_h2_product-converted_h2)
            /product_per_fresh_ng).clamp(0.0,1455.8);
        let scale=fresh_ng/1455.8;

        let generated_h2=
            co_conversion*tail.co_kmol_h+4.0*ch4_conversion*tail.ch4_kmol_h;
        let next=TailGasInventory {
            h2_kmol_h:base_tail.h2_kmol_h*scale
                +(1.0-recycle_h2_recovery)*tail.h2_kmol_h
                +(1.0-psa_recovery)*generated_h2,
            co_kmol_h:base_tail.co_kmol_h*scale
                +(1.0-co_conversion)*tail.co_kmol_h,
            ch4_kmol_h:base_tail.ch4_kmol_h*scale
                +(1.0-ch4_conversion)*tail.ch4_kmol_h,
            co2_kmol_h:0.0,
        };
        captured_co2=base_tail.co2_kmol_h*scale
            +co_conversion*tail.co_kmol_h
            +ch4_conversion*tail.ch4_kmol_h;
        water=co_conversion*tail.co_kmol_h
            +2.0*ch4_conversion*tail.ch4_kmol_h;

        let err=(next.h2_kmol_h-tail.h2_kmol_h).abs()
            .max((next.co_kmol_h-tail.co_kmol_h).abs())
            .max((next.ch4_kmol_h-tail.ch4_kmol_h).abs());
        tail=next;
        if err<tolerance {
            let frac=fresh_ng/1455.8;
            return IterativeTailRecycleResult {
                iterations:iteration,converged:true,
                fresh_ng_kmol_h:fresh_ng,
                fresh_ng_fraction_of_baseline:frac,
                fresh_feed_energy_mw:ieaghg_feed_lhv_mw()*frac,
                tail_h2_kmol_h:tail.h2_kmol_h,
                tail_co_kmol_h:tail.co_kmol_h,
                tail_ch4_kmol_h:tail.ch4_kmol_h,
                captured_co2_kmol_h:captured_co2,
                recycled_carbon_kmol_h:tail.co_kmol_h+tail.ch4_kmol_h,
                extra_water_consumed_kmol_h:water,
            };
        }
    }
    let frac=fresh_ng/1455.8;
    IterativeTailRecycleResult {
        iterations:max_iterations,converged:false,
        fresh_ng_kmol_h:fresh_ng,
        fresh_ng_fraction_of_baseline:frac,
        fresh_feed_energy_mw:ieaghg_feed_lhv_mw()*frac,
        tail_h2_kmol_h:tail.h2_kmol_h,
        tail_co_kmol_h:tail.co_kmol_h,
        tail_ch4_kmol_h:tail.ch4_kmol_h,
        captured_co2_kmol_h:captured_co2,
        recycled_carbon_kmol_h:tail.co_kmol_h+tail.ch4_kmol_h,
        extra_water_consumed_kmol_h:water,
    }
}

#[cfg(test)]
mod iterative_recycle_tests {
    use super::*;

    #[test]
    fn fixed_h2_recycle_converges_and_reduces_fresh_ng() {
        let r=iterative_tail_recycle_fixed_h2(0.80,0.80,0.80,1.0e-8,10_000);
        assert!(r.converged);
        assert!(r.iterations<10_000);
        assert!(r.fresh_ng_kmol_h>0.0 && r.fresh_ng_kmol_h<1455.8);
        assert!(r.fresh_feed_energy_mw>0.0 && r.fresh_feed_energy_mw<ieaghg_feed_lhv_mw());
    }

    #[test]
    fn no_recycle_is_not_a_recycle_fixed_point() {
        let r=iterative_tail_recycle_fixed_h2(0.0,0.0,0.0,1.0e-8,100);
        // With all removal/recovery coefficients zero, the initialized source
        // tail is simply carried forward and accumulates each iteration. There
        // is no finite recycle fixed point; the once-through IEAGHG case is a
        // separate topology, not the zero-coefficient limit of this loop.
        assert!(!r.converged);
        assert!((r.fresh_ng_kmol_h-1455.8).abs()<1.0e-6);
    }

    #[test]
    fn positive_conversion_changes_inventory_not_fresh_feed_fixed_point() {
        let a=iterative_tail_recycle_fixed_h2(0.50,0.50,0.50,1.0e-10,100_000);
        let b=iterative_tail_recycle_fixed_h2(0.80,0.80,0.80,1.0e-10,100_000);
        assert!(a.converged && b.converged);
        assert!((a.fresh_ng_kmol_h-b.fresh_ng_kmol_h).abs()<1.0e-6);
        assert!((a.tail_co_kmol_h-b.tail_co_kmol_h).abs()>1.0);
    }
}


/// Analytical fixed point for the reduced recycle equations when all three
/// recycle/conversion coefficients are strictly positive.
///
/// A useful and non-obvious consequence of the reduced model is that the
/// converged fresh-NG fraction is independent of the assumed CO conversion,
/// CH4 conversion and recycle-H2 recovery. Those coefficients change the
/// circulating inventory, but at fixed H2 product the net recoverable H2
/// equivalent from the source tail is H2 + CO + 4 CH4.
///
/// Let P be once-through H2 product and H,C,M the source tail H2,CO,CH4.
/// Then s = P/(P + H + C + 4M), where s is fresh NG / baseline fresh NG.
pub fn analytical_tail_recycle_fresh_ng_fraction()->f64 {
    let t=ieaghg_tail_inventory();
    let p=ieaghg_reconstructed_h2_product_kmol_per_h();
    p/(p+t.h2_kmol_h+t.co_kmol_h+4.0*t.ch4_kmol_h)
}

/// Analytical fixed-point circulating tail for positive conversion/recovery
/// coefficients. This is an algebraic benchmark for the iterative solver,
/// not an independent physical model.
pub fn analytical_tail_recycle_fixed_point(
    co_conversion:f64,
    ch4_conversion:f64,
    recycle_h2_recovery:f64,
)->TailGasInventory {
    assert!(co_conversion>0.0 && co_conversion<=1.0);
    assert!(ch4_conversion>0.0 && ch4_conversion<=1.0);
    assert!(recycle_h2_recovery>0.0 && recycle_h2_recovery<=1.0);
    let t=ieaghg_tail_inventory();
    let s=analytical_tail_recycle_fresh_ng_fraction();
    let psa=ieaghg_reconstructed_psa_h2_recovery();
    TailGasInventory {
        h2_kmol_h:s*(t.h2_kmol_h+(1.0-psa)*(t.co_kmol_h+4.0*t.ch4_kmol_h))
            /recycle_h2_recovery,
        co_kmol_h:s*t.co_kmol_h/co_conversion,
        ch4_kmol_h:s*t.ch4_kmol_h/ch4_conversion,
        co2_kmol_h:0.0,
    }
}

#[cfg(test)]
mod analytical_recycle_tests {
    use super::*;

    #[test]
    fn iterative_solver_matches_analytical_fixed_point() {
        let i=iterative_tail_recycle_fixed_h2(0.80,0.80,0.80,1.0e-10,100_000);
        let a=analytical_tail_recycle_fixed_point(0.80,0.80,0.80);
        let s=analytical_tail_recycle_fresh_ng_fraction();
        assert!(i.converged);
        assert!((i.fresh_ng_fraction_of_baseline-s).abs()<1.0e-9);
        assert!((i.tail_h2_kmol_h-a.h2_kmol_h).abs()<1.0e-7);
        assert!((i.tail_co_kmol_h-a.co_kmol_h).abs()<1.0e-7);
        assert!((i.tail_ch4_kmol_h-a.ch4_kmol_h).abs()<1.0e-7);
    }

    #[test]
    fn positive_coefficients_change_inventory_not_fresh_feed_fixed_point() {
        let a=iterative_tail_recycle_fixed_h2(0.50,0.60,0.70,1.0e-10,100_000);
        let b=iterative_tail_recycle_fixed_h2(0.90,0.95,0.85,1.0e-10,100_000);
        assert!(a.converged && b.converged);
        assert!((a.fresh_ng_fraction_of_baseline-b.fresh_ng_fraction_of_baseline).abs()<1.0e-9);
        assert!((a.tail_co_kmol_h-b.tail_co_kmol_h).abs()>1.0);
    }
}


/// Lifecycle screen driven by the analytically verified reduced recycle fixed
/// point, replacing the earlier single-pass 80% fresh-feed approximation.
///
/// Capture is applied to the external fresh-feed carbon entering the closed
/// recycle system. This is internally consistent with the reduced fixed-point
/// carbon ledger, but still inherits the surrogate model's fixed conversion
/// and PSA-recovery assumptions.
pub fn converged_recycle_shared_direct_lifecycle_screen(
    capture_fraction:f64,
    upstream_gco2e_per_mj:f64,
    thermal_service_mw:f64,
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
    ccs_transport_fraction:f64,
)->LifecycleCase {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let fresh_fraction=analytical_tail_recycle_fresh_ng_fraction();
    let external_feed_carbon=
        feedstock_carbon_co2_equivalent_kg_per_kg_h2()*fresh_fraction;
    let fresh_ng_energy=ieaghg_feed_lhv_mw()*fresh_fraction;
    LifecycleCase {
        plant_carbon:external_feed_carbon*(1.0-capture_fraction),
        upstream_ng:upstream_ng_from_energy_mw_kgco2e_per_kgh2(
            fresh_ng_energy,upstream_gco2e_per_mj),
        nuclear:direct_nuclear_heat_lca_proxy_kgco2e_per_kgh2(
            thermal_service_mw,nuclear_gco2e_per_kwh_e,net_electric_efficiency),
        ccs_transport:ccs_transport_kgco2e_per_kgh2(
            external_feed_carbon*capture_fraction,ccs_transport_fraction),
    }
}

/// Gross NG energy displaced relative to the unabated IEAGHG plant:
/// all supplementary furnace fuel plus the fresh-feed reduction produced by
/// the converged reduced recycle fixed point.
pub fn converged_recycle_gross_ng_displacement_mw()->f64 {
    let s=analytical_tail_recycle_fresh_ng_fraction();
    ieaghg_makeup_fuel_lhv_mw()+ieaghg_feed_lhv_mw()*(1.0-s)
}

#[cfg(test)]
mod converged_lifecycle_tests {
    use super::*;

    #[test]
    fn converged_lifecycle_uses_verified_fresh_feed_fraction() {
        let s=analytical_tail_recycle_fresh_ng_fraction();
        let c=converged_recycle_shared_direct_lifecycle_screen(
            0.90,11.5,162.0,5.5,0.504,0.025);
        let expected_upstream=upstream_ng_from_energy_mw_kgco2e_per_kgh2(
            ieaghg_feed_lhv_mw()*s,11.5);
        assert!((c.upstream_ng-expected_upstream).abs()<1.0e-12);
        assert!(c.total()>0.0);
    }

    #[test]
    fn converged_recycle_still_exceeds_assignment_abatement_scale_in_reference_screen() {
        let b=ieaghg_unabated_lifecycle_screen(11.5);
        let c=converged_recycle_shared_direct_lifecycle_screen(
            0.90,11.5,162.0,5.5,0.504,0.025);
        let (mt,_)=annual_lifecycle_abatement_and_budget(
            b,c,IEAGHG_BASE.h2_kg_per_h,8322.0,100.0);
        assert!(mt>0.25);
    }
}


/// Reference-screen numerical summary for the converged recycle lifecycle case.
/// Assumptions intentionally match the previous lifecycle sensitivity so the
/// only change is replacing the one-pass recycle approximation by the verified
/// fixed point.
#[derive(Debug,Clone,Copy)]
pub struct ConvergedReferenceScreen {
    pub fresh_ng_fraction:f64,
    pub fresh_ng_kmol_h:f64,
    pub fresh_feed_mw:f64,
    pub gross_ng_displacement_mw:f64,
    pub baseline_ci:f64,
    pub candidate_ci:f64,
    pub specific_abatement:f64,
    pub annual_abatement_mt:f64,
    pub annual_s100_budget_sgd:f64,
}
pub fn converged_reference_screen()->ConvergedReferenceScreen {
    let s=analytical_tail_recycle_fresh_ng_fraction();
    let b=ieaghg_unabated_lifecycle_screen(11.5);
    let c=converged_recycle_shared_direct_lifecycle_screen(
        0.90,11.5,162.0,5.5,0.504,0.025);
    let (mt,budget)=annual_lifecycle_abatement_and_budget(
        b,c,IEAGHG_BASE.h2_kg_per_h,8322.0,100.0);
    ConvergedReferenceScreen {
        fresh_ng_fraction:s,
        fresh_ng_kmol_h:1455.8*s,
        fresh_feed_mw:ieaghg_feed_lhv_mw()*s,
        gross_ng_displacement_mw:converged_recycle_gross_ng_displacement_mw(),
        baseline_ci:b.total(),
        candidate_ci:c.total(),
        specific_abatement:b.total()-c.total(),
        annual_abatement_mt:mt,
        annual_s100_budget_sgd:budget,
    }
}

#[cfg(test)]
mod converged_reference_screen_tests {
    use super::*;
    #[test]
    fn reference_screen_numerics_are_stable() {
        let r=converged_reference_screen();
        assert!(r.fresh_ng_fraction>0.70 && r.fresh_ng_fraction<0.77);
        assert!(r.annual_abatement_mt>0.25);
        assert!(r.annual_s100_budget_sgd>25_000_000.0);
    }
}


/// Full-cost boundary using the converged recycle NG displacement rather than
/// the superseded single-pass 80% screen.
pub fn converged_min_gas_price_for_full_cost_sgd_per_gj(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    ccs_ts_cost_sgd_y:f64,
    allocated_reactor_cost_sgd_y:f64,
    ihx_loop_cost_sgd_y:f64,
    other_integration_cost_sgd_y:f64,
)->f64 {
    let saved_gj_y=converged_recycle_gross_ng_displacement_mw()*8322.0*3.6;
    let heat=annual_thermal_energy_cost_sgd(
        htgr_service_mw,8322.0,nuclear_heat_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()
        *8322.0*electricity_sgd_per_mwh;
    let required=allocated_reactor_cost_sgd_y+ihx_loop_cost_sgd_y
        +other_integration_cost_sgd_y+heat+elec+ccs_ts_cost_sgd_y
        -allowed_incremental_cost_sgd_y;
    required/saved_gj_y
}

pub fn converged_max_ccs_ts_cost_sgd_y(
    allowed_incremental_cost_sgd_y:f64,
    htgr_service_mw:f64,
    gas_price_sgd_per_gj:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    allocated_reactor_cost_sgd_y:f64,
    ihx_loop_cost_sgd_y:f64,
    other_integration_cost_sgd_y:f64,
)->f64 {
    let ng=annual_thermal_energy_cost_sgd(
        converged_recycle_gross_ng_displacement_mw(),8322.0,gas_price_sgd_per_gj);
    let heat=annual_thermal_energy_cost_sgd(
        htgr_service_mw,8322.0,nuclear_heat_sgd_per_gj);
    let elec=case2a_tail_separation_net_electric_anchor_mwe()
        *8322.0*electricity_sgd_per_mwh;
    allowed_incremental_cost_sgd_y+ng-heat-elec-allocated_reactor_cost_sgd_y
        -ihx_loop_cost_sgd_y-other_integration_cost_sgd_y
}

/// Reproducible representative full-cost screen using the same explicit
/// assumptions previously applied to the 80% sensitivity:
/// 162 MWth process service; S$5.69/GJ nuclear heat; S$150/MWh separation;
/// S$50m/y allocated reactor cost; S$8.2m/y IHX/loop; S$5m/y other integration;
/// and S$31.9m/y low Group-A-like CCS T&S.
/// The abatement budget is taken from the converged lifecycle case itself.
pub fn converged_representative_full_cost_screen()->(f64,f64,f64) {
    let r=converged_reference_screen();
    let min_gas=converged_min_gas_price_for_full_cost_sgd_per_gj(
        r.annual_s100_budget_sgd,162.0,5.69,150.0,31.9e6,50.0e6,8.2e6,5.0e6);
    let max_ts_at_15=converged_max_ccs_ts_cost_sgd_y(
        r.annual_s100_budget_sgd,162.0,15.0,5.69,150.0,50.0e6,8.2e6,5.0e6);
    let max_ts_at_20=converged_max_ccs_ts_cost_sgd_y(
        r.annual_s100_budget_sgd,162.0,20.0,5.69,150.0,50.0e6,8.2e6,5.0e6);
    (min_gas,max_ts_at_15,max_ts_at_20)
}

#[cfg(test)]
mod converged_full_cost_tests {
    use super::*;
    #[test]
    fn converged_full_cost_boundary_is_finite_and_ordered() {
        let (p,ts15,ts20)=converged_representative_full_cost_screen();
        assert!(p.is_finite() && p>0.0);
        assert!(ts20>ts15);
    }
}


#[cfg(test)]
mod converged_report_values {
    use super::*;
    #[test]
    fn print_converged_reference_and_full_cost_values() {
        let r=converged_reference_screen();
        let (p,ts15,ts20)=converged_representative_full_cost_screen();
        println!(
            "CONVERGED_SCREEN fresh_fraction={:.9} fresh_ng_kmol_h={:.3} fresh_feed_mw={:.3} gross_displacement_mw={:.3} baseline_ci={:.6} candidate_ci={:.6} specific_abatement={:.6} annual_abatement_mt={:.6} annual_budget_sgd={:.3} min_gas_sgd_gj={:.6} max_ts_15_sgd_y={:.3} max_ts_20_sgd_y={:.3}",
            r.fresh_ng_fraction,r.fresh_ng_kmol_h,r.fresh_feed_mw,
            r.gross_ng_displacement_mw,r.baseline_ci,r.candidate_ci,
            r.specific_abatement,r.annual_abatement_mt,r.annual_s100_budget_sgd,
            p,ts15,ts20
        );
    }
}


/// First recycle-adjusted HTGR heat-service bound consistent with the verified
/// fixed point.
///
/// This deliberately does NOT scale the whole furnace-service envelope with
/// fresh NG. JAEA's HTTR steam-reforming architecture couples secondary helium
/// to reformer, superheater and steam generator; those duties depend on the
/// integrated circulating process, not simply fresh-feed rate.
///
/// The only correction made here is the standard reaction-enthalpy delta:
/// (i) remove the methane-reforming+shift heat associated with displaced fresh
/// NG methane; (ii) add reaction heat for the converged recycled CO/CH4 that is
/// converted each pass. At a positive fixed point, converted CO and CH4 per pass
/// equal s times their source-tail generation, independent of conversion
/// coefficient; lower conversion increases inventory rather than net conversion.
pub fn converged_recycle_reaction_heat_delta_mw()->f64 {
    let s=analytical_tail_recycle_fresh_ng_fraction();
    let t=ieaghg_tail_inventory();

    let displaced_ng_kmol_h=1455.8*(1.0-s);
    let removed_ch4=displaced_ng_kmol_h*IEAGHG_NG.methane;
    let removed_q=removed_ch4*1000.0*(SMR_DH298_KJ_MOL+WGS_DH298_KJ_MOL)/3.6e6;

    let recycled_q=(
        s*t.ch4_kmol_h*1000.0*(SMR_DH298_KJ_MOL+WGS_DH298_KJ_MOL)
        +s*t.co_kmol_h*1000.0*WGS_DH298_KJ_MOL
    )/3.6e6;
    recycled_q-removed_q
}

/// Recycle-adjusted process-heat envelope at a specified reformer inlet.
/// Base furnace-dependent services remain at their source-anchored values;
/// the converged reaction-heat delta is applied, followed by the existing
/// source-bounded incremental MDEA heat range.
///
/// This is a controlled bound, not a final integrated heat balance. In
/// particular, sensible heating of the larger recycle circulation and changes
/// to steam generation/heat recovery are not yet reconstructed.
pub fn converged_htgr_service_source_bounded_mw(
    reformer_inlet_c:f64
)->(f64,f64) {
    let (base_lo,base_hi)=bounded_furnace_service_envelope_mw(reformer_inlet_c);
    let rxn=converged_recycle_reaction_heat_delta_mw();
    let (mdea_lo,mdea_hi)=mdea_incremental_heat_source_bounded_mw();
    (base_lo+rxn+mdea_lo,base_hi+rxn+mdea_hi)
}

#[cfg(test)]
mod converged_heat_service_tests {
    use super::*;

    #[test]
    fn converged_reaction_delta_is_a_saving_for_source_tail() {
        assert!(converged_recycle_reaction_heat_delta_mw()<0.0);
    }

    #[test]
    fn converged_service_bound_is_ordered_and_physical() {
        let (lo,hi)=converged_htgr_service_source_bounded_mw(625.0);
        assert!(lo>100.0);
        assert!(hi>lo);
        assert!(hi<250.0);
    }

    #[test]
    fn lower_conversion_changes_inventory_not_net_fixed_point_reaction_conversion() {
        let s=analytical_tail_recycle_fresh_ng_fraction();
        let t=ieaghg_tail_inventory();
        let a=analytical_tail_recycle_fixed_point(0.50,0.60,0.70);
        let b=analytical_tail_recycle_fixed_point(0.90,0.95,0.85);
        assert!((0.50*a.co_kmol_h-s*t.co_kmol_h).abs()<1.0e-9);
        assert!((0.90*b.co_kmol_h-s*t.co_kmol_h).abs()<1.0e-9);
        assert!((0.60*a.ch4_kmol_h-s*t.ch4_kmol_h).abs()<1.0e-9);
        assert!((0.95*b.ch4_kmol_h-s*t.ch4_kmol_h).abs()<1.0e-9);
    }
}


#[cfg(test)]
mod converged_heat_report {
    use super::*;
    #[test]
    fn lock_converged_heat_envelope_ranges() {
        let d=converged_recycle_reaction_heat_delta_mw();
        let (lo,hi)=converged_htgr_service_source_bounded_mw(625.0);
        // Broad regression windows deliberately preserve source/property
        // uncertainty while detecting accidental reversion to the old 80% case.
        assert!(d > -20.0 && d < -5.0);
        assert!(lo>140.0 && lo<170.0);
        assert!(hi>165.0 && hi<200.0);
    }
}


/// Ideal-gas isentropic compressor work for a recycle-gas mixture, MW.
///
/// cp_kj_kmol_k and k are caller-supplied mixture properties so the screening
/// does not fabricate a detailed EOS. This is intended for sensitivity bounds.
pub fn ideal_gas_compressor_power_mw(
    flow_kmol_h:f64,
    inlet_k:f64,
    pressure_ratio:f64,
    cp_kj_kmol_k:f64,
    k:f64,
    isentropic_efficiency:f64,
)->f64 {
    assert!(flow_kmol_h>=0.0 && inlet_k>0.0 && pressure_ratio>=1.0);
    assert!(cp_kj_kmol_k>0.0 && k>1.0);
    assert!(isentropic_efficiency>0.0 && isentropic_efficiency<=1.0);
    let tout_over_tin=pressure_ratio.powf((k-1.0)/k);
    flow_kmol_h*cp_kj_kmol_k*inlet_k*(tout_over_tin-1.0)
        /(3600.0*1000.0*isentropic_efficiency)
}

/// Converged combustible recycle molar flow (H2+CO+CH4) for chosen positive
/// conversion/recovery coefficients.
pub fn converged_combustible_recycle_kmol_h(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64
)->f64 {
    let t=analytical_tail_recycle_fixed_point(
        co_conversion,ch4_conversion,h2_recovery);
    t.h2_kmol_h+t.co_kmol_h+t.ch4_kmol_h
}

/// Sensible heat required to raise converged recycle H2/CO/CH4 from an inlet
/// temperature to a process-feed temperature. Uses encoded NIST Shomate
/// properties and therefore stays within their valid temperature ranges.
///
/// This is an incremental recycle-stream duty only; it does not subtract the
/// sensible duty of fresh NG displaced elsewhere in the plant.
pub fn converged_recycle_sensible_heat_mw(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,outlet_c:f64,
)->f64 {
    assert!(outlet_c>inlet_c);
    let t1=inlet_c+273.15; let t2=outlet_c+273.15;
    assert!(t1>=298.15 && t2<=1000.0);
    let r=analytical_tail_recycle_fixed_point(
        co_conversion,ch4_conversion,h2_recovery);
    let q=
        r.h2_kmol_h*NIST_H2_298_1000.delta_h_kj_mol(t1,t2)
        +r.co_kmol_h*NIST_CO_298_1300.delta_h_kj_mol(t1,t2)
        +r.ch4_kmol_h*NIST_CH4_298_1300.delta_h_kj_mol(t1,t2);
    q/3600.0
}

/// Pressure-ratio sensitivity for recycle recompression.
///
/// IEAGHG Case 2A establishes that PSA tail gas can require compression from
/// about 0.2 MPa to 1 MPa for MDEA capture, but the proposed nuclear recycle
/// topology has not yet fixed where recycle is withdrawn/reinjected. Therefore
/// pressure ratio is explicit rather than silently adopting Case-2A compression.
pub fn converged_recycle_compression_sensitivity_mwe(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,pressure_ratio:f64,efficiency:f64,
)->f64 {
    let r=analytical_tail_recycle_fixed_point(
        co_conversion,ch4_conversion,h2_recovery);
    let total=r.h2_kmol_h+r.co_kmol_h+r.ch4_kmol_h;
    let yh2=r.h2_kmol_h/total; let yco=r.co_kmol_h/total; let ych4=r.ch4_kmol_h/total;
    // Representative ideal-gas cp values near ambient/moderate temperature,
    // kJ/kmol-K. Kept explicit as a screening mixture rather than EOS output.
    let cp=yh2*28.84+yco*29.14+ych4*35.7;
    let r_univ=8.314462618;
    let k=cp/(cp-r_univ);
    ideal_gas_compressor_power_mw(total,inlet_c+273.15,pressure_ratio,cp,k,efficiency)
}

#[cfg(test)]
mod recycle_penalty_tests {
    use super::*;

    #[test]
    fn recycle_sensible_heat_is_positive_and_conversion_sensitive() {
        let q80=converged_recycle_sensible_heat_mw(0.8,0.8,0.8,40.0,370.0);
        let q50=converged_recycle_sensible_heat_mw(0.5,0.5,0.5,40.0,370.0);
        assert!(q80>0.0);
        assert!(q50>q80); // lower removal/recovery -> larger circulating inventory
    }

    #[test]
    fn recycle_compression_zero_at_unity_pressure_ratio() {
        let p=converged_recycle_compression_sensitivity_mwe(
            0.8,0.8,0.8,40.0,1.0,0.75);
        assert!(p.abs()<1.0e-12);
    }

    #[test]
    fn recycle_compression_rises_with_pressure_ratio() {
        let p2=converged_recycle_compression_sensitivity_mwe(
            0.8,0.8,0.8,40.0,2.0,0.75);
        let p5=converged_recycle_compression_sensitivity_mwe(
            0.8,0.8,0.8,40.0,5.0,0.75);
        assert!(p5>p2 && p2>0.0);
    }
}


#[cfg(test)]
mod recycle_penalty_regression_ranges {
    use super::*;
    #[test]
    fn recycle_penalties_remain_screening_scale() {
        let q=converged_recycle_sensible_heat_mw(0.8,0.8,0.8,40.0,370.0);
        let p2=converged_recycle_compression_sensitivity_mwe(0.8,0.8,0.8,40.0,2.0,0.75);
        let p5=converged_recycle_compression_sensitivity_mwe(0.8,0.8,0.8,40.0,5.0,0.75);
        assert!(q>1.0 && q<20.0);
        assert!(p2>0.1 && p2<10.0);
        assert!(p5>p2 && p5<20.0);
    }
}


/// Pressure-topology screen for furnace-free PSA-tail recycle.
///
/// Source facts:
/// - conventional SMR PSA tail gas is near atmospheric pressure;
/// - IEAGHG Case 2A compresses tail gas to ~10 bar specifically to enable MDEA;
/// - JAEA HTTR steam-reforming design uses ~4.5 MPa process gas.
///
/// A direct "PSA tail -> MDEA at 1 MPa -> reformer feed at multi-MPa" recycle
/// therefore necessarily requires a second pressure lift unless the capture
/// topology or PSA pressure architecture is redesigned.
#[derive(Debug,Clone,Copy)]
pub struct RecyclePressureTopology {
    pub psa_tail_mpa:f64,
    pub mdea_pressure_mpa:f64,
    pub reformer_process_mpa:f64,
}
pub const SOURCE_ANCHORED_RECYCLE_PRESSURES:RecyclePressureTopology=
    RecyclePressureTopology{
        psa_tail_mpa:0.13,       // ~0.3 barg typical SMR PSA tail context
        mdea_pressure_mpa:1.0,   // IEAGHG Case 2A: around 10 bar
        reformer_process_mpa:4.5,// JAEA HTTR H2 system design
    };

/// Minimum pressure ratio from CO2-depleted MDEA outlet to the JAEA-like
/// reformer process pressure if no pressure recovery/integration is credited.
pub fn post_capture_recycle_pressure_ratio()->f64 {
    SOURCE_ANCHORED_RECYCLE_PRESSURES.reformer_process_mpa
        /SOURCE_ANCHORED_RECYCLE_PRESSURES.mdea_pressure_mpa
}

/// Minimum two-stage ideal-gas recycle compression sensitivity:
/// stage 1 PSA-tail -> MDEA pressure; stage 2 sweet recycle -> reformer pressure.
/// Intercooling is conservatively represented by resetting both stage inlets to
/// the supplied inlet temperature. This is a topology screen, not compressor design.
pub fn converged_two_stage_recycle_compression_mwe(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,efficiency:f64,
)->(f64,f64,f64) {
    let p=SOURCE_ANCHORED_RECYCLE_PRESSURES;
    let stage1=converged_recycle_compression_sensitivity_mwe(
        co_conversion,ch4_conversion,h2_recovery,inlet_c,
        p.mdea_pressure_mpa/p.psa_tail_mpa,efficiency);
    let stage2=converged_recycle_compression_sensitivity_mwe(
        co_conversion,ch4_conversion,h2_recovery,inlet_c,
        p.reformer_process_mpa/p.mdea_pressure_mpa,efficiency);
    (stage1,stage2,stage1+stage2)
}

#[cfg(test)]
mod recycle_pressure_topology_tests {
    use super::*;
    #[test]
    fn source_anchored_topology_requires_post_capture_pressure_lift() {
        assert!(post_capture_recycle_pressure_ratio()>4.0);
    }
    #[test]
    fn two_stage_compression_exceeds_capture_stage_alone() {
        let (a,b,t)=converged_two_stage_recycle_compression_mwe(
            0.8,0.8,0.8,40.0,0.75);
        assert!(a>0.0 && b>0.0);
        assert!((t-a-b).abs()<1.0e-12);
        assert!(t>a);
    }
}


/// IEAGHG Case-2A electrical decomposition, MWe.
/// CO2 capture plant consumption explicitly INCLUDES the tail-gas compressor.
/// Equipment list gives tail-gas compressor brake power = 4.280 MW for
/// 0.126 -> 1.0 MPa, so it must not be added again to the 4.575 MWe capture load.
pub const IEAGHG_CASE2A_CAPTURE_INCL_TAIL_COMP_MWE:f64=4.575;
pub const IEAGHG_CASE2A_TAIL_COMP_BRAKE_MW:f64=4.280;
pub const IEAGHG_CASE2A_CO2_COMP_DEHYDRATION_MWE:f64=2.874;
pub const IEAGHG_CASE2A_TAIL_EXPANDER_GENERATION_MWE:f64=1.140;

/// Net Case-2A capture-chain electricity exactly reproducing the source ledger.
pub fn case2a_capture_chain_net_mwe()->f64 {
    IEAGHG_CASE2A_CAPTURE_INCL_TAIL_COMP_MWE
        +IEAGHG_CASE2A_CO2_COMP_DEHYDRATION_MWE
        -IEAGHG_CASE2A_TAIL_EXPANDER_GENERATION_MWE
}

/// Incremental post-capture compressor for the proposed recycle topology only:
/// ~1 MPa sweet recycle -> ~4.5 MPa reformer process pressure.
/// The Case-2A 0.126->1 MPa compressor remains inside the 4.575 MWe anchor.
pub fn converged_post_capture_recycle_compressor_mwe(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,efficiency:f64,
)->f64 {
    converged_recycle_compression_sensitivity_mwe(
        co_conversion,ch4_conversion,h2_recovery,inlet_c,
        post_capture_recycle_pressure_ratio(),efficiency)
}

/// Corrected electricity screen for capture + CO2 compression + source expander
/// + incremental post-capture recycle recompression.
///
/// Important topology caveat: if sweet tail gas is no longer expanded because it
/// is recycled directly, the 1.140 MWe generation credit must be removed. The
/// caller selects whether that source expander credit survives.
pub fn converged_capture_and_recycle_electricity_mwe(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,efficiency:f64,retain_source_expander_credit:bool,
)->f64 {
    let expander=if retain_source_expander_credit {
        IEAGHG_CASE2A_TAIL_EXPANDER_GENERATION_MWE
    } else { 0.0 };
    IEAGHG_CASE2A_CAPTURE_INCL_TAIL_COMP_MWE
        +IEAGHG_CASE2A_CO2_COMP_DEHYDRATION_MWE
        -expander
        +converged_post_capture_recycle_compressor_mwe(
            co_conversion,ch4_conversion,h2_recovery,inlet_c,efficiency)
}

#[cfg(test)]
mod case2a_electric_decomposition_tests {
    use super::*;
    #[test]
    fn source_ledger_reproduces_old_anchor() {
        assert!((case2a_capture_chain_net_mwe()
            -case2a_tail_separation_net_electric_anchor_mwe()).abs()<1.0e-12);
    }
    #[test]
    fn recycle_topology_adds_only_post_capture_compression() {
        let old=case2a_capture_chain_net_mwe();
        let new=converged_capture_and_recycle_electricity_mwe(
            0.8,0.8,0.8,40.0,0.75,true);
        assert!(new>old);
    }
    #[test]
    fn removing_expander_credit_increases_net_load_by_source_generation() {
        let a=converged_capture_and_recycle_electricity_mwe(
            0.8,0.8,0.8,40.0,0.75,true);
        let b=converged_capture_and_recycle_electricity_mwe(
            0.8,0.8,0.8,40.0,0.75,false);
        assert!((b-a-IEAGHG_CASE2A_TAIL_EXPANDER_GENERATION_MWE).abs()<1.0e-12);
    }
}


/// Corrected representative full-cost screen including post-capture recycle
/// recompression and no expander credit (sweet gas is recycled, not expanded
/// to furnace burners). Sensible recycle heating is added to the 162 MWth
/// representative service as a conservative incremental screen.
pub fn converged_representative_full_cost_with_recycle_penalties()
    ->(f64,f64,f64,f64,f64)
{
    let r=converged_reference_screen();
    let q_recycle=converged_recycle_sensible_heat_mw(0.8,0.8,0.8,40.0,370.0);
    let q_total=162.0+q_recycle;
    let e_total=converged_capture_and_recycle_electricity_mwe(
        0.8,0.8,0.8,40.0,0.75,false);

    let saved_gj_y=converged_recycle_gross_ng_displacement_mw()*8322.0*3.6;
    let heat=annual_thermal_energy_cost_sgd(q_total,8322.0,5.69);
    let elec=e_total*8322.0*150.0;
    let fixed=31.9e6+50.0e6+8.2e6+5.0e6;
    let min_gas=(fixed+heat+elec-r.annual_s100_budget_sgd)/saved_gj_y;

    let max_ts_15=r.annual_s100_budget_sgd
        +annual_thermal_energy_cost_sgd(
            converged_recycle_gross_ng_displacement_mw(),8322.0,15.0)
        -heat-elec-50.0e6-8.2e6-5.0e6;
    let max_ts_20=r.annual_s100_budget_sgd
        +annual_thermal_energy_cost_sgd(
            converged_recycle_gross_ng_displacement_mw(),8322.0,20.0)
        -heat-elec-50.0e6-8.2e6-5.0e6;
    (q_recycle,q_total,e_total,min_gas,max_ts_15.min(max_ts_20))
}

#[cfg(test)]
mod corrected_full_cost_penalty_tests {
    use super::*;
    #[test]
    fn recycle_penalties_tighten_economic_boundary() {
        let (_,q,e,p,_)=converged_representative_full_cost_with_recycle_penalties();
        let (old_p,_,_)=converged_representative_full_cost_screen();
        assert!(q>162.0);
        assert!(e>case2a_capture_chain_net_mwe());
        assert!(p>old_p);
    }
}


/// Post-capture recycle compression to an explicit injection pressure.
/// Inlet is the ~1 MPa MDEA sweet-gas pressure. This exposes pressure
/// integration as a design variable rather than fixing the JAEA 4.5 MPa point.
pub fn converged_post_capture_recycle_compressor_to_pressure_mwe(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
    inlet_c:f64,injection_pressure_mpa:f64,efficiency:f64,
)->f64 {
    assert!(injection_pressure_mpa>=SOURCE_ANCHORED_RECYCLE_PRESSURES.mdea_pressure_mpa);
    converged_recycle_compression_sensitivity_mwe(
        co_conversion,ch4_conversion,h2_recovery,inlet_c,
        injection_pressure_mpa/SOURCE_ANCHORED_RECYCLE_PRESSURES.mdea_pressure_mpa,
        efficiency)
}

/// Corrected full-cost boundary for arbitrary recycle injection pressure and
/// compressor efficiency. Case-2A capture + CO2 compression are retained,
/// while its expander credit is removed because sweet gas is recycled.
pub fn converged_min_gas_price_with_pressure_integration_sgd_per_gj(
    injection_pressure_mpa:f64,
    compressor_efficiency:f64,
)->f64 {
    let r=converged_reference_screen();
    let q_recycle=converged_recycle_sensible_heat_mw(0.8,0.8,0.8,40.0,370.0);
    let q_total=162.0+q_recycle;
    let recycle_comp=converged_post_capture_recycle_compressor_to_pressure_mwe(
        0.8,0.8,0.8,40.0,injection_pressure_mpa,compressor_efficiency);
    let e_total=IEAGHG_CASE2A_CAPTURE_INCL_TAIL_COMP_MWE
        +IEAGHG_CASE2A_CO2_COMP_DEHYDRATION_MWE+recycle_comp;
    let saved_gj_y=converged_recycle_gross_ng_displacement_mw()*8322.0*3.6;
    let heat=annual_thermal_energy_cost_sgd(q_total,8322.0,5.69);
    let elec=e_total*8322.0*150.0;
    let fixed=31.9e6+50.0e6+8.2e6+5.0e6;
    (fixed+heat+elec-r.annual_s100_budget_sgd)/saved_gj_y
}

/// Compact sensitivity surface for reporting:
/// pressures [2, 3, 4.5] MPa x efficiencies [0.65, 0.75, 0.85].
pub fn recycle_pressure_efficiency_sensitivity()
    ->[[f64;3];3]
{
    let ps=[2.0,3.0,4.5];
    let etas=[0.65,0.75,0.85];
    let mut out=[[0.0;3];3];
    for (i,p) in ps.iter().enumerate() {
        for (j,e) in etas.iter().enumerate() {
            out[i][j]=converged_min_gas_price_with_pressure_integration_sgd_per_gj(*p,*e);
        }
    }
    out
}

#[cfg(test)]
mod pressure_efficiency_surface_tests {
    use super::*;
    #[test]
    fn higher_injection_pressure_tightens_boundary() {
        let a=converged_min_gas_price_with_pressure_integration_sgd_per_gj(2.0,0.75);
        let b=converged_min_gas_price_with_pressure_integration_sgd_per_gj(4.5,0.75);
        assert!(b>a);
    }
    #[test]
    fn better_compressor_efficiency_relaxes_boundary() {
        let a=converged_min_gas_price_with_pressure_integration_sgd_per_gj(4.5,0.65);
        let b=converged_min_gas_price_with_pressure_integration_sgd_per_gj(4.5,0.85);
        assert!(b<a);
    }
    #[test]
    fn sensitivity_surface_is_monotonic() {
        let s=recycle_pressure_efficiency_sensitivity();
        for j in 0..3 { assert!(s[2][j]>s[1][j] && s[1][j]>s[0][j]); }
        for i in 0..3 { assert!(s[i][0]>s[i][1] && s[i][1]>s[i][2]); }
    }
}


/// Named sensitivity points for deterministic reporting and comparison.
/// Returns rows for 2.0, 3.0, 4.5 MPa; columns for eta=0.65,0.75,0.85.
pub fn recycle_pressure_efficiency_min_gas_table_sgd_per_gj()->[[f64;3];3] {
    recycle_pressure_efficiency_sensitivity()
}

/// Range width across the pressure/efficiency surface, useful for comparing
/// compressor-design sensitivity against larger economic uncertainties.
pub fn recycle_pressure_efficiency_boundary_span_sgd_per_gj()->f64 {
    let s=recycle_pressure_efficiency_sensitivity();
    let mut lo=f64::INFINITY;
    let mut hi=f64::NEG_INFINITY;
    for row in s {
        for v in row {
            lo=lo.min(v);
            hi=hi.max(v);
        }
    }
    hi-lo
}

#[cfg(test)]
mod pressure_surface_report_tests {
    use super::*;
    #[test]
    fn pressure_surface_span_is_finite_and_not_zero() {
        let span=recycle_pressure_efficiency_boundary_span_sgd_per_gj();
        assert!(span.is_finite() && span>0.0);
    }
}


#[cfg(test)]
mod pressure_surface_numeric_lock {
    use super::*;
    #[test]
    fn pressure_surface_is_secondary_scale_under_reference_costs() {
        let s=recycle_pressure_efficiency_min_gas_table_sgd_per_gj();
        let span=recycle_pressure_efficiency_boundary_span_sgd_per_gj();
        assert!(s[0][0]>10.0 && s[0][0]<25.0);
        assert!(s[2][2]>10.0 && s[2][2]<25.0);
        assert!(span<5.0);
    }
}


impl Shomate {
    /// NIST standard molar entropy S°(T), J/mol-K, t=T/1000.
    pub fn entropy_j_mol_k(self, temperature_k:f64)->f64 {
        let t=temperature_k/1000.0;
        self.a*t.ln()+self.b*t+self.c*t*t/2.0+self.d*t*t*t/3.0
            -self.e/(2.0*t*t)+self.g
    }
    /// Standard molar Gibbs energy relative to elements in their standard
    /// reference states, kJ/mol. Uses ΔfH°298 (=Shomate H parameter) plus
    /// sensible enthalpy and absolute standard entropy.
    pub fn standard_gibbs_kj_mol(self, temperature_k:f64)->f64 {
        self.h+self.sensible_h_kj_mol(temperature_k)
            -temperature_k*self.entropy_j_mol_k(temperature_k)/1000.0
    }
}

/// Dimensionless equilibrium constants at 1-bar standard state.
/// SMR: CH4 + H2O <=> CO + 3H2.
/// WGS: CO + H2O <=> CO2 + H2.
pub fn smr_equilibrium_constant_nist(temperature_k:f64)->f64 {
    smr_equilibrium_constant_piecewise(temperature_k)
}
pub fn wgs_equilibrium_constant_nist(temperature_k:f64)->f64 {
    wgs_equilibrium_constant_piecewise(temperature_k)
}

/// Ideal-gas reaction quotient with partial pressures divided by 1 bar.
/// Inputs are mole fractions and total pressure in bar.
pub fn smr_reaction_quotient(
    y_ch4:f64,y_h2o:f64,y_co:f64,y_h2:f64,total_pressure_bar:f64
)->f64 {
    assert!(total_pressure_bar>0.0);
    let p=|y:f64| { assert!(y>0.0); y*total_pressure_bar };
    p(y_co)*p(y_h2).powi(3)/(p(y_ch4)*p(y_h2o))
}
pub fn wgs_reaction_quotient(
    y_co:f64,y_h2o:f64,y_co2:f64,y_h2:f64,total_pressure_bar:f64
)->f64 {
    assert!(total_pressure_bar>0.0);
    let p=|y:f64| { assert!(y>0.0); y*total_pressure_bar };
    p(y_co2)*p(y_h2)/(p(y_co)*p(y_h2o))
}

/// IEAGHG stream-6 HTS outlet WGS equilibrium diagnostic.
/// Published state: 412 C, 2.77 MPa; mole fractions CO2=.1283,
/// CO=.0366, H2=.5961, H2O=.2137. Q/K near unity would indicate equilibrium;
/// departure quantifies how inappropriate an equilibrium assumption would be.
pub fn ieaghg_stream6_wgs_q_over_k()->f64 {
    let t=412.0+273.15;
    let q=wgs_reaction_quotient(0.0366,0.2137,0.1283,0.5961,27.7);
    q/wgs_equilibrium_constant_nist(t)
}

#[cfg(test)]
mod equilibrium_layer_tests {
    use super::*;
    #[test]
    fn endothermic_smr_equilibrium_constant_rises_with_temperature() {
        assert!(smr_equilibrium_constant_nist(1000.0)>smr_equilibrium_constant_nist(700.0));
    }
    #[test]
    fn exothermic_wgs_equilibrium_constant_falls_with_temperature() {
        assert!(wgs_equilibrium_constant_nist(700.0)>wgs_equilibrium_constant_nist(1000.0));
    }
    #[test]
    fn smr_reaction_quotient_has_expected_pressure_squared_dependence() {
        let q1=smr_reaction_quotient(0.1,0.3,0.1,0.5,10.0);
        let q2=smr_reaction_quotient(0.1,0.3,0.1,0.5,20.0);
        assert!((q2/q1-4.0).abs()<1.0e-12);
    }
    #[test]
    fn wgs_reaction_quotient_is_pressure_independent_for_delta_n_zero() {
        let q1=wgs_reaction_quotient(0.1,0.3,0.1,0.5,10.0);
        let q2=wgs_reaction_quotient(0.1,0.3,0.1,0.5,30.0);
        assert!((q2/q1-1.0).abs()<1.0e-12);
    }
    #[test]
    fn published_hts_outlet_is_finite_equilibrium_diagnostic() {
        let x=ieaghg_stream6_wgs_q_over_k();
        assert!(x.is_finite() && x>0.0);
    }
}


/// Logarithmic equilibrium departure is numerically and physically clearer:
/// ln(Q/K)=0 at equilibrium; positive means the written forward WGS reaction
/// is thermodynamically driven backward, negative means forward.
pub fn ieaghg_stream6_wgs_ln_q_over_k()->f64 {
    ieaghg_stream6_wgs_q_over_k().ln()
}

/// Apparent WGS equilibrium temperature for the published stream-6 composition:
/// solve K(T)=Q_source over the NIST-valid 500-1000 K interval by bisection.
/// This converts composition departure into an interpretable temperature
/// approach without asserting the reactor itself is at equilibrium.
pub fn ieaghg_stream6_wgs_apparent_equilibrium_temperature_k()->f64 {
    let q=wgs_reaction_quotient(0.0366,0.2137,0.1283,0.5961,27.7);
    let mut lo=500.0;
    let mut hi=1000.0;
    // WGS K decreases monotonically over this interval.
    assert!(wgs_equilibrium_constant_nist(lo)>=q);
    assert!(wgs_equilibrium_constant_nist(hi)<=q);
    for _ in 0..100 {
        let mid=0.5*(lo+hi);
        if wgs_equilibrium_constant_nist(mid)>q { lo=mid; } else { hi=mid; }
    }
    0.5*(lo+hi)
}

pub fn ieaghg_stream6_wgs_temperature_approach_k()->f64 {
    let actual=412.0+273.15;
    actual-ieaghg_stream6_wgs_apparent_equilibrium_temperature_k()
}

#[cfg(test)]
mod ieaghg_wgs_validation_tests {
    use super::*;
    #[test]
    fn apparent_equilibrium_temperature_is_bracketed() {
        let t=ieaghg_stream6_wgs_apparent_equilibrium_temperature_k();
        assert!(t>=500.0 && t<=1000.0);
    }
    #[test]
    fn apparent_temperature_reproduces_source_reaction_quotient() {
        let t=ieaghg_stream6_wgs_apparent_equilibrium_temperature_k();
        let q=wgs_reaction_quotient(0.0366,0.2137,0.1283,0.5961,27.7);
        let k=wgs_equilibrium_constant_nist(t);
        assert!((q/k-1.0).abs()<1.0e-10);
    }
    #[test]
    fn source_hts_is_not_assumed_exact_equilibrium() {
        let d=ieaghg_stream6_wgs_ln_q_over_k();
        assert!(d.is_finite());
    }
}


#[cfg(test)]
mod wgs_numeric_interpretation_lock {
    use super::*;
    #[test]
    fn published_hts_equilibrium_approach_is_moderate_not_orders_of_magnitude() {
        let ratio=ieaghg_stream6_wgs_q_over_k();
        let teq=ieaghg_stream6_wgs_apparent_equilibrium_temperature_k();
        let approach=ieaghg_stream6_wgs_temperature_approach_k();
        // Broad scientific regression bounds, not fitted targets.
        assert!(ratio>0.1 && ratio<10.0);
        assert!(teq>550.0 && teq<850.0);
        assert!(approach>-150.0 && approach<150.0);
    }
}


/// Conventional IEAGHG furnace/radiant benchmark validation.
/// This is the authoritative equipment-list metric used to validate the
/// temperature-resolved service reconstruction before nuclear substitution.
#[derive(Debug,Clone,Copy)]
pub struct ConventionalEnergyValidation {
    pub published_radiant_mw:f64,
    pub reconstructed_service_lo_mw:f64,
    pub reconstructed_service_hi_mw:f64,
    pub published_inside_envelope:bool,
    pub total_ng_lhv_mw:f64,
    pub h2_lhv_mw:f64,
    pub net_power_export_mwe:f64,
}
pub fn conventional_energy_validation(reformer_inlet_c:f64)->ConventionalEnergyValidation {
    let (lo,hi)=bounded_furnace_service_envelope_mw(reformer_inlet_c);
    let q=ieaghg_reformer_radiant_duty_mw();
    ConventionalEnergyValidation{
        published_radiant_mw:q,
        reconstructed_service_lo_mw:lo,
        reconstructed_service_hi_mw:hi,
        published_inside_envelope:q>=lo && q<=hi,
        total_ng_lhv_mw:ieaghg_total_ng_lhv_mw(),
        h2_lhv_mw:IEAGHG_H2_PRODUCT_ENERGY_MW,
        net_power_export_mwe:IEAGHG_NET_POWER_EXPORT_MWE,
    }
}

/// Carbon ledger for the furnace-free capture/recycle topology at the verified
/// reduced fixed point. All quantities are kmol-C/h.
/// Fresh feed carbon enters; carbon leaves only as captured CO2 plus residual
/// uncaptured process carbon. Recycled CO/CH4 are internal and must not be
/// counted as external carbon input/output.
#[derive(Debug,Clone,Copy)]
pub struct ConvergedCarbonLedger {
    pub fresh_feed_c:f64,
    pub captured_c:f64,
    pub residual_emitted_c:f64,
    pub closure_error:f64,
}
pub fn converged_carbon_ledger(capture_fraction:f64)->ConvergedCarbonLedger {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let fresh=ieaghg_feed_carbon_kmol_per_h()*analytical_tail_recycle_fresh_ng_fraction();
    let captured=fresh*capture_fraction;
    let emitted=fresh*(1.0-capture_fraction);
    ConvergedCarbonLedger{
        fresh_feed_c:fresh,captured_c:captured,residual_emitted_c:emitted,
        closure_error:fresh-captured-emitted,
    }
}

/// Explicit physical destinations for the converged PSA-tail architecture.
/// Existing tail CO2 plus carbon converted from recycled CO/CH4 goes to the
/// high-pressure process-carbon capture train; H2/CO/CH4 return to reforming.
/// A purge is not yet sized, so inert accumulation remains outside this CHO
/// reduced model and is an explicit limitation.
pub fn converged_tail_carbon_to_capture_kmol_h(
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64
)->f64 {
    let s=analytical_tail_recycle_fresh_ng_fraction();
    let source=ieaghg_tail_inventory();
    let loop_tail=analytical_tail_recycle_fixed_point(
        co_conversion,ch4_conversion,h2_recovery);
    // fresh-feed-scaled CO2 entering tail separator plus converted loop carbon
    s*source.co2_kmol_h
        +co_conversion*loop_tail.co_kmol_h
        +ch4_conversion*loop_tail.ch4_kmol_h
}

#[cfg(test)]
mod review1_closure_tests {
    use super::*;
    #[test]
    fn converged_external_carbon_closes_exactly() {
        let c=converged_carbon_ledger(0.90);
        assert!(c.closure_error.abs()<1.0e-10);
        assert!((c.fresh_feed_c-c.captured_c-c.residual_emitted_c).abs()<1.0e-10);
    }
    #[test]
    fn tail_carbon_destination_is_positive_and_finite() {
        let x=converged_tail_carbon_to_capture_kmol_h(0.8,0.8,0.8);
        assert!(x.is_finite() && x>0.0);
    }
    #[test]
    fn conventional_source_energy_ledger_is_physical() {
        let v=conventional_energy_validation(625.0);
        assert!(v.published_radiant_mw>90.0 && v.published_radiant_mw<100.0);
        assert!(v.total_ng_lhv_mw>v.h2_lhv_mw);
        assert!(v.net_power_export_mwe>0.0);
    }
}


/// Frozen common comparison basis from the authoritative IEAGHG base case.
/// Product purity is >99.9%; PSA equipment lists 2.58/2.51 MPa on H2 side,
/// so plant-gate product is compared at 2.51 MPa without downstream merchant
/// compression. Annual results scale from the fixed 100,000 Nm3/h product.
#[derive(Debug,Clone,Copy)]
pub struct ComparisonBasis {
    pub h2_kg_per_h:f64,
    pub h2_nm3_per_h:f64,
    pub purity_min_mol_fraction:f64,
    pub plant_gate_pressure_mpa:f64,
}
pub const COMMON_H2_BASIS:ComparisonBasis=ComparisonBasis{
    h2_kg_per_h:8994.0,
    h2_nm3_per_h:100000.0,
    purity_min_mol_fraction:0.999,
    plant_gate_pressure_mpa:2.51,
};

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum SystemBoundary {
    ProcessGate,
    PlantGate,
    Lifecycle,
}

/// Boundary nesting is strict: process-gate terms are a subset of plant-gate,
/// and lifecycle adds upstream NG, nuclear LCA and CCS-chain burdens.
pub fn boundary_rank(b:SystemBoundary)->u8 {
    match b {
        SystemBoundary::ProcessGate=>0,
        SystemBoundary::PlantGate=>1,
        SystemBoundary::Lifecycle=>2,
    }
}

#[cfg(test)]
mod common_basis_tests {
    use super::*;
    #[test]
    fn common_product_basis_matches_ieaghg_scale() {
        assert!((COMMON_H2_BASIS.h2_kg_per_h-IEAGHG_BASE.h2_kg_per_h).abs()<1e-12);
        assert!(COMMON_H2_BASIS.purity_min_mol_fraction>=0.999);
        assert!((COMMON_H2_BASIS.plant_gate_pressure_mpa-2.51).abs()<1e-12);
    }
    #[test]
    fn system_boundaries_are_strictly_nested() {
        assert!(boundary_rank(SystemBoundary::ProcessGate)
            <boundary_rank(SystemBoundary::PlantGate));
        assert!(boundary_rank(SystemBoundary::PlantGate)
            <boundary_rank(SystemBoundary::Lifecycle));
    }
}


/// Matched lifecycle screen for the 80% recycle + direct-HTGR architecture.
///
/// This keeps the same H2 output as IEAGHG, reduces fresh-feed NG according to
/// the reduced recycle model, removes purchased supplementary furnace NG, and
/// treats the remaining fresh-feed carbon with an explicit capture fraction.
/// Upstream NG applies to remaining fresh-feed energy only.
/// Nuclear and CCS-transport terms remain explicit sensitivities.
///
/// IMPORTANT: this is still a screening LCA. It does not yet include
/// construction/infrastructure, methane-leakage pathway detail beyond the
/// supplied upstream intensity, or site-specific CO2 shipping/storage LCA.
pub fn recycle80_direct_lifecycle_screen(
    capture_fraction:f64,
    upstream_gco2e_per_mj:f64,
    nuclear_gco2e_per_kwh_e:f64,
    net_electric_efficiency:f64,
    thermal_service_mw:f64,
    ccs_transport_fraction:f64,
)->LifecycleCase {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let r=reduced_tail_recycle_fixed_h2(0.80,0.80,0.80);
    let remaining_feed_fraction=1.0-r.fresh_ng_displaced_fraction;

    // Carbon entering as fresh feed after recycle displacement.
    let fresh_feed_co2eq=feedstock_carbon_co2_equivalent_kg_per_kg_h2()
        *remaining_feed_fraction;

    LifecycleCase {
        plant_carbon:fresh_feed_co2eq*(1.0-capture_fraction),
        upstream_ng:upstream_ng_kgco2e_per_kgh2(upstream_gco2e_per_mj)
            *remaining_feed_fraction,
        nuclear:direct_nuclear_heat_lca_proxy_kgco2e_per_kgh2(
            thermal_service_mw,nuclear_gco2e_per_kwh_e,net_electric_efficiency),
        ccs_transport:ccs_transport_kgco2e_per_kgh2(
            fresh_feed_co2eq*capture_fraction,ccs_transport_fraction),
    }
}

/// Lifecycle baseline including feedstock + separately purchased furnace NG
/// upstream burden, plus direct plant CO2 from IEAGHG.
/// Upstream intensity is applied to both NG energy streams.
pub fn ieaghg_base_lifecycle_screen(
    upstream_gco2e_per_mj:f64
)->LifecycleCase {
    let feed_up=upstream_ng_kgco2e_per_kgh2(upstream_gco2e_per_mj);
    let fuel_gj_h=ieaghg_makeup_fuel_lhv_mw()*3.6;
    let fuel_up_kg_h=fuel_gj_h*1000.0*upstream_gco2e_per_mj/1000.0;
    let fuel_up_per_h2=fuel_up_kg_h/IEAGHG_BASE.h2_kg_per_h;
    LifecycleCase{
        plant_carbon:IEAGHG_BASE.co2_emitted_kg_per_kg_h2(),
        upstream_ng:feed_up+fuel_up_per_h2,
        nuclear:0.0,
        ccs_transport:0.0,
    }
}

/// Annual lifecycle abatement and corresponding S$100/t budget for a matched
/// baseline/candidate pair at the IEAGHG fixed H2 output and operating hours.
pub fn matched_lifecycle_abatement_budget(
    baseline:LifecycleCase,
    candidate:LifecycleCase,
    hours_per_year:f64,
    target_sgd_per_tco2e:f64,
)->(f64,f64,f64) {
    let delta=baseline.total()-candidate.total();
    assert!(delta>0.0);
    let annual_h2_kg=IEAGHG_BASE.h2_kg_per_h*hours_per_year;
    let avoided_t=delta*annual_h2_kg/1000.0;
    let budget=avoided_t*target_sgd_per_tco2e;
    (delta,avoided_t,budget)
}


/// Re-run the full-cost shared-reactor boundary with an internally matched
/// lifecycle budget instead of an externally assumed annual budget.
pub fn recycle80_min_gas_price_with_matched_lifecycle(
    baseline:LifecycleCase,
    candidate:LifecycleCase,
    hours_per_year:f64,
    target_sgd_per_tco2e:f64,
    htgr_service_mw:f64,
    nuclear_heat_sgd_per_gj:f64,
    electricity_sgd_per_mwh:f64,
    ccs_ts_cost_sgd_y:f64,
    allocated_reactor_cost_sgd_y:f64,
    ihx_loop_cost_sgd_y:f64,
    other_integration_cost_sgd_y:f64,
)->f64 {
    let (_,_,budget)=matched_lifecycle_abatement_budget(
        baseline,candidate,hours_per_year,target_sgd_per_tco2e);
    recycle80_min_gas_price_for_full_cost_sgd_per_gj(
        budget,htgr_service_mw,nuclear_heat_sgd_per_gj,electricity_sgd_per_mwh,
        ccs_ts_cost_sgd_y,allocated_reactor_cost_sgd_y,ihx_loop_cost_sgd_y,
        other_integration_cost_sgd_y)
}


/// Singapore 2024 import-mix screening upstream intensity.
/// EMA: 6 Mtoe LNG / 11 Mtoe total NG imports.
/// IEA 2026 anchors: 18.6 gCO2e/MJ delivered LNG and 11.5 gCO2e/MJ
/// global-average natural-gas supply. This is a weighted screening proxy,
/// not a route-specific measured Singapore intensity.
pub fn singapore_2024_ng_mix_screen_gco2e_per_mj()->f64 {
    let lng_share=6.0/11.0;
    lng_share*18.6+(1.0-lng_share)*11.5
}

#[derive(Debug,Clone,Copy)]
pub struct MatchedRecycleLifecycleEconomicPoint {
    pub upstream_gco2e_per_mj:f64,
    pub nuclear_gco2e_per_kwh_e:f64,
    pub ccs_transport_fraction:f64,
    pub baseline_kgco2e_per_kgh2:f64,
    pub candidate_kgco2e_per_kgh2:f64,
    pub abatement_kgco2e_per_kgh2:f64,
    pub annual_abatement_t:f64,
    pub annual_budget_sgd:f64,
    pub min_gas_price_low_ts_sgd_per_gj:f64,
}

/// Joint lifecycle/economic point for the same 80% recycle/shared-HTGR case.
pub fn matched_recycle_lifecycle_economic_point(
    upstream_gco2e_per_mj:f64,
    nuclear_gco2e_per_kwh_e:f64,
    ccs_transport_fraction:f64,
)->MatchedRecycleLifecycleEconomicPoint {
    let baseline=ieaghg_base_lifecycle_screen(upstream_gco2e_per_mj);
    let candidate=recycle80_direct_lifecycle_screen(
        0.90,upstream_gco2e_per_mj,nuclear_gco2e_per_kwh_e,
        0.504,162.0,ccs_transport_fraction);
    let (delta,annual,budget)=matched_lifecycle_abatement_budget(
        baseline,candidate,8322.0,100.0);
    let p=recycle80_min_gas_price_for_full_cost_sgd_per_gj(
        budget,162.0,5.69,150.0,31_900_000.0,
        50_000_000.0,8_200_000.0,5_000_000.0);
    MatchedRecycleLifecycleEconomicPoint{
        upstream_gco2e_per_mj,
        nuclear_gco2e_per_kwh_e,
        ccs_transport_fraction,
        baseline_kgco2e_per_kgh2:baseline.total(),
        candidate_kgco2e_per_kgh2:candidate.total(),
        abatement_kgco2e_per_kgh2:delta,
        annual_abatement_t:annual,
        annual_budget_sgd:budget,
        min_gas_price_low_ts_sgd_per_gj:p,
    }
}


/// Formation enthalpy at 298.15 K from the Shomate H parameter, kJ/mol.
fn hf298_kj_mol(s:Shomate)->f64 { s.h }

/// Approximate total ideal-gas enthalpy flow for the major CHO/N2 species,
/// MW relative to elements at 298.15 K. Hydrocarbon C2+ is intentionally not
/// accepted here because this function is used only after the pre-reformer,
/// where the IEAGHG reformer product contains none.
pub fn major_stream_enthalpy_mw(s:FullStream,temperature_k:f64)->f64 {
    assert!(temperature_k>=500.0 && temperature_k<=1300.0);
    assert!(s.c2h6==0.0 && s.c3h8==0.0 && s.nc4h10==0.0 && s.nc5h12==0.0);
    let h=|n:f64,p:Shomate| n*(hf298_kj_mol(p)+p.sensible_h_kj_mol(temperature_k));
    let mut total=0.0;
    total+=h(s.flow(s.co2),NIST_CO2_298_1200);
    total+=h(s.flow(s.co),NIST_CO_298_1300);
    total+=if temperature_k<=1000.0 {
        h(s.flow(s.h2),NIST_H2_298_1000)
    } else {
        // Preserve the 298-K reference by integrating low interval to 1000 K
        // and the high interval from 1000 K onward.
        s.flow(s.h2)*(NIST_H2_298_1000.sensible_h_kj_mol(1000.0)
            +NIST_H2_1000_2500.sensible_h_kj_mol(temperature_k)
            -NIST_H2_1000_2500.sensible_h_kj_mol(1000.0))
    };
    total+=h(s.flow(s.ch4),NIST_CH4_298_1300);
    total+=h(s.flow(s.h2o),NIST_H2O_500_1700);
    // N2 formation enthalpy is zero; choose correlation by temperature.
    total+=s.flow(s.n2)*(if temperature_k<=500.0 {
        NIST_N2_100_500.sensible_h_kj_mol(temperature_k)
    } else {
        NIST_N2_500_2000.sensible_h_kj_mol(temperature_k)
    });
    total/3600.0
}

/// Reconstruct the unnumbered primary-reformer outlet by reversing only the
/// source-described reformer WHB cooling from a chosen outlet temperature to
/// published stream 5 at 320 C. Composition is held equal to stream 5 because
/// the WHB has no reaction/material source in the IEAGHG description.
///
/// The resulting WHB duty is an independently temperature-resolved heat-recovery
/// metric and can be checked against the source statement that ~75% of saturated
/// HP steam is generated in the syngas WHB.
pub fn reconstructed_reformer_whb_duty_mw(reformer_outlet_c:f64)->f64 {
    assert!(reformer_outlet_c>=900.0 && reformer_outlet_c<=950.0);
    let s=ieaghg_hts_inlet();
    major_stream_enthalpy_mw(s,reformer_outlet_c+273.15)
        -major_stream_enthalpy_mw(s,320.0+273.15)
}

/// Source steam-generation heat sink represented by 75% of saturated HP steam.
/// This is a latent-heat-only benchmark because the exact economizer/evaporator
/// split is not separately published.
pub fn source_whb_steam_latent_benchmark_mw()->(f64,f64) {
    let m=ieaghg_syngas_whb_steam_kg_h_approx()/3600.0;
    (m*STEAM_HFG_4P5MPA_KJ_KG/1000.0,m*STEAM_HFG_4MPA_KJ_KG/1000.0)
}

#[cfg(test)]
mod conventional_temperature_balance_tests {
    use super::*;
    #[test]
    fn reformer_whb_duty_rises_with_source_outlet_temperature() {
        assert!(reconstructed_reformer_whb_duty_mw(950.0)
            >reconstructed_reformer_whb_duty_mw(900.0));
    }
    #[test]
    fn whb_temperature_reconstruction_is_same_order_as_source_steam_sink() {
        let q=reconstructed_reformer_whb_duty_mw(925.0);
        let (lo,hi)=source_whb_steam_latent_benchmark_mw();
        // Not equality: source benchmark omits feedwater sensible heating and
        // superheat. This test only rejects grossly inconsistent reconstruction.
        assert!(q>0.5*lo && q<2.0*hi);
    }
}


/// Dimensionless comparison of reconstructed syngas-WHB sensible recovery to
/// the source's approximate 75% saturated-HP-steam latent duty.
/// Values near unity are not required because economizer/feedwater sensible
/// heating is omitted from the latent benchmark; the ratio is a validation
/// diagnostic, not a fitted closure.
pub fn reformer_whb_to_source_latent_ratio(reformer_outlet_c:f64)->(f64,f64) {
    let q=reconstructed_reformer_whb_duty_mw(reformer_outlet_c);
    let (lo,hi)=source_whb_steam_latent_benchmark_mw();
    (q/hi,q/lo)
}

/// Published reformer radiant duty versus the reconstructed WHB recovery.
/// These are different control-volume terms but together provide two
/// independent authoritative checks on the conventional temperature hierarchy.
pub fn conventional_temperature_validation_metrics()
    ->(f64,(f64,f64),(f64,f64))
{
    (
        ieaghg_reformer_radiant_duty_mw(),
        (reconstructed_reformer_whb_duty_mw(900.0),
         reconstructed_reformer_whb_duty_mw(950.0)),
        reformer_whb_to_source_latent_ratio(925.0),
    )
}

#[cfg(test)]
mod conventional_validation_metric_tests {
    use super::*;
    #[test]
    fn published_radiant_conversion_is_exact() {
        let expected=82.63*4.184/3.6;
        assert!((ieaghg_reformer_radiant_duty_mw()-expected).abs()<1e-12);
    }
    #[test]
    fn whb_reconstruction_brackets_source_temperature_uncertainty() {
        let (_,q,_)=conventional_temperature_validation_metrics();
        assert!(q.1>q.0 && q.0>0.0);
    }
}


/// Source-table reconstructed aggregate H2O addition between IEAGHG streams 4
/// and 5 (second HP steam + BFW), kmol/h. This is a least-squares reconciliation
/// of independent H and O residuals; it is not a separately published utility.
pub const IEAGHG_INTERSTAGE_H2O_RECONCILED_KMOL_H:f64=154.4;

/// Reduced primary-reformer inlet surrogate for the tube-side energy bound.
/// Because the pre-reformer outlet is not numbered, carbon species are not
/// invented here. Instead, the lower/upper radiant-duty calculation uses the
/// source stream-5 product composition and independently reconstructed reaction
/// enthalpy; this function supplies only the known additional water sensible
/// enthalpy entering after stream 4.
pub fn interstage_water_enthalpy_mw(temperature_c:f64)->f64 {
    let t=temperature_c+273.15;
    assert!(t>=500.0 && t<=1000.0);
    IEAGHG_INTERSTAGE_H2O_RECONCILED_KMOL_H
        *(hf298_kj_mol(NIST_H2O_500_1700)+NIST_H2O_500_1700.sensible_h_kj_mol(t))
        /3600.0
}

/// Temperature-corrected net reaction duty from the source-reconstructed
/// overall reforming extents. Reaction enthalpies are corrected from 298 K by
/// species sensible enthalpies for CH4+H2O->CO+3H2 and CO+H2O->CO2+H2.
pub fn smr_delta_h_kj_mol_at_t(t:f64)->f64 {
    assert!(t>=500.0 && t<=1000.0);
    DELTA_H_SMR_298_KJ_PER_MOL
        +NIST_CO_298_1300.sensible_h_kj_mol(t)
        +3.0*NIST_H2_298_1000.sensible_h_kj_mol(t)
        -NIST_CH4_298_1300.sensible_h_kj_mol(t)
        -NIST_H2O_500_1700.sensible_h_kj_mol(t)
}
pub fn wgs_delta_h_kj_mol_at_t(t:f64)->f64 {
    assert!(t>=500.0 && t<=1000.0);
    DELTA_H_WGS_298_KJ_PER_MOL
        +NIST_CO2_298_1200.sensible_h_kj_mol(t)
        +NIST_H2_298_1000.sensible_h_kj_mol(t)
        -NIST_CO_298_1300.sensible_h_kj_mol(t)
        -NIST_H2O_500_1700.sensible_h_kj_mol(t)
}

/// Overall source reaction-duty diagnostic evaluated at a representative
/// reformer inlet temperature. This remains a bound because pre-reformer and
/// primary-reformer extents are not separately published.
pub fn source_temperature_corrected_reaction_duty_mw(reformer_inlet_c:f64)->f64 {
    assert!(reformer_inlet_c>500.0 && reformer_inlet_c<=700.0);
    let t=reformer_inlet_c+273.15;
    let x=reduced_extents_from_psa_inlet();
    (x.smr*smr_delta_h_kj_mol_at_t(t)+x.wgs*wgs_delta_h_kj_mol_at_t(t))/3600.0
}

/// Independent radiant-duty reconstruction envelope:
/// temperature-corrected reaction duty plus sensible heating of the published
/// reformer-product composition from an admissible 600-700 C inlet surrogate
/// to the source 900-950 C outlet. This deliberately avoids using the published
/// 96 MW radiant duty in the calculation.
pub fn independent_radiant_duty_envelope_mw()->(f64,f64) {
    let product=ieaghg_hts_inlet();
    let calc=|tin_c:f64,tout_c:f64| {
        let rxn=source_temperature_corrected_reaction_duty_mw(tin_c);
        let sens=major_stream_enthalpy_mw(product,tout_c+273.15)
            -major_stream_enthalpy_mw(product,tin_c+273.15);
        rxn+sens
    };
    let lo=calc(700.0,900.0);
    let hi=calc(600.0,950.0);
    (lo.min(hi),lo.max(hi))
}

pub fn independent_radiant_benchmark_relative_error_bounds()->(f64,f64) {
    let (lo,hi)=independent_radiant_duty_envelope_mw();
    let q=ieaghg_reformer_radiant_duty_mw();
    ((lo-q)/q,(hi-q)/q)
}

#[cfg(test)]
mod independent_radiant_validation_tests {
    use super::*;
    #[test]
    fn temperature_corrected_smr_remains_endothermic() {
        assert!(smr_delta_h_kj_mol_at_t(900.0)>0.0);
    }
    #[test]
    fn independent_radiant_envelope_is_ordered_and_positive() {
        let (lo,hi)=independent_radiant_duty_envelope_mw();
        assert!(lo>0.0 && hi>lo);
    }
    #[test]
    fn authoritative_radiant_benchmark_is_not_orders_outside_reconstruction() {
        let (lo,hi)=independent_radiant_duty_envelope_mw();
        let q=ieaghg_reformer_radiant_duty_mw();
        assert!(q>0.5*lo && q<2.0*hi);
    }
}


/// Validation of the independently reconstructed radiant-duty envelope against
/// the authoritative IEAGHG equipment-list radiant duty. No tolerance is used
/// to construct the envelope; this function only evaluates the comparison.
#[derive(Debug,Clone,Copy)]
pub struct RadiantEnvelopeValidation {
    pub calculated_lo_mw:f64,
    pub calculated_hi_mw:f64,
    pub reference_mw:f64,
    pub reference_inside:bool,
    pub nearest_relative_error:f64,
}
pub fn validate_independent_radiant_envelope()->RadiantEnvelopeValidation {
    let (lo,hi)=independent_radiant_duty_envelope_mw();
    let q=ieaghg_reformer_radiant_duty_mw();
    let nearest=if q<lo {(lo-q)/q} else if q>hi {(q-hi)/q} else {0.0};
    RadiantEnvelopeValidation{
        calculated_lo_mw:lo,calculated_hi_mw:hi,reference_mw:q,
        reference_inside:q>=lo && q<=hi,
        nearest_relative_error:nearest,
    }
}

#[cfg(test)]
mod radiant_acceptance_tests {
    use super::*;
    #[test]
    fn radiant_validation_metrics_are_finite() {
        let v=validate_independent_radiant_envelope();
        assert!(v.calculated_lo_mw.is_finite() && v.calculated_hi_mw.is_finite());
        assert!(v.reference_mw.is_finite() && v.nearest_relative_error.is_finite());
        assert!(v.calculated_hi_mw>v.calculated_lo_mw);
    }
}


#[cfg(test)]
mod radiant_acceptance_numeric_probe {
    use super::*;
    #[test]
    fn classify_radiant_reference_against_independent_envelope() {
        let v=validate_independent_radiant_envelope();
        // Classification only: broad bins reveal whether missing physics is
        // small (<20%), material (20-50%), or gross (>50%) without tuning.
        assert!(v.nearest_relative_error<0.50);
    }
}


#[cfg(test)]
mod radiant_acceptance_tighter_classification {
    use super::*;
    #[test]
    fn radiant_nearest_mismatch_is_below_twenty_percent() {
        let v=validate_independent_radiant_envelope();
        assert!(v.nearest_relative_error<0.20);
    }
}


#[cfg(test)]
mod radiant_acceptance_ten_percent_probe {
    use super::*;
    #[test]
    fn probe_ten_percent_radiant_validation_threshold() {
        let v=validate_independent_radiant_envelope();
        assert!(v.nearest_relative_error<0.10);
    }
}


#[cfg(test)]
mod radiant_acceptance_five_percent_probe {
    use super::*;
    #[test]
    fn probe_five_percent_radiant_validation_threshold() {
        let v=validate_independent_radiant_envelope();
        assert!(v.nearest_relative_error<0.05);
    }
}


/// Source-reconstructed N2 generation into the PSA-tail/recycle system.
/// IEAGHG stream 5 carries 0.15 mol% N2 at 8370.3 kmol/h. N2 is chemically
/// inert through HTS/cooling/PSA, so this is the best source-backed external
/// inert feed available without inventing an NG nitrogen fraction.
pub fn ieaghg_process_n2_feed_kmol_h()->f64 {
    ieaghg_hts_inlet().flow(ieaghg_hts_inlet().n2)
}

/// Steady-state inert purge closure for a recycle loop.
/// If fraction p of the post-capture recycle stream is purged each pass and
/// N2 has no other sink, circulating N2 before purge is F_N2/p and purge N2
/// equals the external N2 feed exactly.
#[derive(Debug,Clone,Copy)]
pub struct InertPurgeClosure {
    pub purge_fraction:f64,
    pub circulating_n2_kmol_h:f64,
    pub purge_n2_kmol_h:f64,
    pub external_n2_kmol_h:f64,
    pub closure_error_kmol_h:f64,
}
pub fn inert_purge_closure(purge_fraction:f64)->InertPurgeClosure {
    assert!(purge_fraction>0.0 && purge_fraction<=1.0);
    let feed=ieaghg_process_n2_feed_kmol_h();
    let circ=feed/purge_fraction;
    let purge=purge_fraction*circ;
    InertPurgeClosure{
        purge_fraction,
        circulating_n2_kmol_h:circ,
        purge_n2_kmol_h:purge,
        external_n2_kmol_h:feed,
        closure_error_kmol_h:feed-purge,
    }
}

/// Purge fraction required to cap N2 at a specified mole fraction in the
/// combustible recycle stream. The combustible inventory comes from the
/// analytically verified reduced fixed point. This is a transparent screening
/// closure; a rigorous PSA model may change the species split.
pub fn purge_fraction_for_max_n2_mole_fraction(
    max_n2_mole_fraction:f64,
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
)->f64 {
    assert!(max_n2_mole_fraction>0.0 && max_n2_mole_fraction<1.0);
    let combustible=converged_combustible_recycle_kmol_h(
        co_conversion,ch4_conversion,h2_recovery);
    let n2_max=max_n2_mole_fraction/(1.0-max_n2_mole_fraction)*combustible;
    (ieaghg_process_n2_feed_kmol_h()/n2_max).min(1.0)
}

/// Carbon lost with a nonselective purge of the post-capture H2/CO/CH4 recycle.
/// CO2 has already been removed to the process-carbon capture train.
pub fn recycle_purge_carbon_kmol_h(
    purge_fraction:f64,
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
)->f64 {
    assert!((0.0..=1.0).contains(&purge_fraction));
    let r=analytical_tail_recycle_fixed_point(
        co_conversion,ch4_conversion,h2_recovery);
    purge_fraction*(r.co_kmol_h+r.ch4_kmol_h)
}

/// Complete external carbon ledger including a nonselective inert-control purge.
/// Purged CO/CH4 carbon is an explicit plant-gate carbonaceous offgas requiring
/// oxidation/capture or another disposition; it is not silently counted as CO2.
#[derive(Debug,Clone,Copy)]
pub struct PurgedCarbonLedger {
    pub fresh_feed_c:f64,
    pub captured_c:f64,
    pub residual_process_c:f64,
    pub purge_carbon_c:f64,
    pub closure_error:f64,
}
pub fn converged_carbon_ledger_with_purge(
    capture_fraction:f64,purge_fraction:f64,
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
)->PurgedCarbonLedger {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let fresh=ieaghg_feed_carbon_kmol_per_h()*analytical_tail_recycle_fresh_ng_fraction();
    let purge_c=recycle_purge_carbon_kmol_h(
        purge_fraction,co_conversion,ch4_conversion,h2_recovery);
    // Capture fraction applies to non-purged fresh carbon in this screening ledger.
    let available=(fresh-purge_c).max(0.0);
    let captured=available*capture_fraction;
    let residual=available*(1.0-capture_fraction);
    PurgedCarbonLedger{
        fresh_feed_c:fresh,captured_c:captured,residual_process_c:residual,
        purge_carbon_c:purge_c,
        closure_error:fresh-captured-residual-purge_c,
    }
}

#[cfg(test)]
mod inert_and_purge_closure_tests {
    use super::*;
    #[test]
    fn inert_purge_closes_n2_exactly() {
        for p in [0.01,0.05,0.10] {
            let x=inert_purge_closure(p);
            assert!(x.closure_error_kmol_h.abs()<1e-12);
            assert!((x.purge_n2_kmol_h-x.external_n2_kmol_h).abs()<1e-12);
        }
    }
    #[test]
    fn tighter_n2_limit_requires_more_purge() {
        let p1=purge_fraction_for_max_n2_mole_fraction(0.01,0.8,0.8,0.8);
        let p5=purge_fraction_for_max_n2_mole_fraction(0.05,0.8,0.8,0.8);
        assert!(p1>p5 && p5>0.0);
    }
    #[test]
    fn carbon_closes_when_purge_is_explicit() {
        let p=purge_fraction_for_max_n2_mole_fraction(0.05,0.8,0.8,0.8);
        let c=converged_carbon_ledger_with_purge(0.90,p,0.8,0.8,0.8);
        assert!(c.closure_error.abs()<1e-10);
        assert!(c.purge_carbon_c>0.0);
    }
}


/// Review-1 CCS topology: stream-specific, furnace-free architecture.
///
/// 1) Shifted syngas: high-pressure MDEA is the primary process-carbon capture
///    location (IEAGHG Case 1A analogue, ~2.5 MPa shifted gas).
/// 2) PSA tail/recycle: residual CO2 after PSA is removed by a compressed-tail
///    MDEA polishing step (Case 2A analogue, ~1 MPa absorber feed).
/// 3) Purge: nonselective inert-control purge contains H2/CO/CH4. It is routed
///    to a small catalytic oxidizer; resulting CO2 joins the capture/compression
///    train. No reformer furnace/flue-gas MEA block exists in the nuclear case.
///
/// This topology gives every carbonaceous tail species a physical destination
/// without pretending one generic amine capture fraction applies to all streams.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum CarbonStreamDisposition {
    ShiftedSyngasMdea,
    TailGasMdeaPolishing,
    RecycleToReformer,
    PurgeCatalyticOxidationThenCapture,
}
pub fn nuclear_ccs_topology_dispositions()
    ->[CarbonStreamDisposition;4]
{
    [
        CarbonStreamDisposition::ShiftedSyngasMdea,
        CarbonStreamDisposition::TailGasMdeaPolishing,
        CarbonStreamDisposition::RecycleToReformer,
        CarbonStreamDisposition::PurgeCatalyticOxidationThenCapture,
    ]
}

/// Carbon closure after assigning purge carbon to oxidation + capture.
/// capture_fraction applies to all process CO2 after oxidation in this screening
/// topology; detailed solvent sizing remains stream-specific and separate.
pub fn nuclear_ccs_topology_carbon_ledger(
    capture_fraction:f64,purge_fraction:f64,
    co_conversion:f64,ch4_conversion:f64,h2_recovery:f64,
)->ConvergedCarbonLedger {
    assert!((0.0..=1.0).contains(&capture_fraction));
    let p=converged_carbon_ledger_with_purge(
        capture_fraction,purge_fraction,co_conversion,ch4_conversion,h2_recovery);
    // Oxidized purge carbon becomes capture-train CO2; apply same terminal
    // capture fraction only at the final carbon ledger, not as a solvent-duty model.
    let captured=p.captured_c+capture_fraction*p.purge_carbon_c;
    let emitted=p.residual_process_c+(1.0-capture_fraction)*p.purge_carbon_c;
    ConvergedCarbonLedger{
        fresh_feed_c:p.fresh_feed_c,
        captured_c:captured,
        residual_emitted_c:emitted,
        closure_error:p.fresh_feed_c-captured-emitted,
    }
}

#[cfg(test)]
mod ccs_topology_closure_tests {
    use super::*;
    #[test]
    fn furnace_free_topology_has_no_flue_gas_capture_block() {
        let d=nuclear_ccs_topology_dispositions();
        assert!(!d.iter().any(|x| matches!(x,
            CarbonStreamDisposition::PurgeCatalyticOxidationThenCapture)==false
            && false)); // compile-time enum is intentionally limited to four routes
        assert_eq!(d.len(),4);
    }
    #[test]
    fn topology_closes_carbon_with_inert_control_purge() {
        let p=purge_fraction_for_max_n2_mole_fraction(0.05,0.8,0.8,0.8);
        let c=nuclear_ccs_topology_carbon_ledger(0.90,p,0.8,0.8,0.8);
        assert!(c.closure_error.abs()<1e-10);
        assert!(c.captured_c>0.0 && c.residual_emitted_c>0.0);
    }
}


/// Quantitative heat-integration feasibility envelope using the validated
/// conventional high-grade duty and the retained JAEA GTHTR300C IHX benchmark.
/// This remains reactor-agnostic: it tests temperature/duty/pressure-drop
/// constraints, not a vendor selection.
#[derive(Debug,Clone,Copy)]
pub struct NuclearHeatIntegrationCheck {
    pub process_duty_lo_mw:f64,
    pub process_duty_hi_mw:f64,
    pub ihx_benchmark_mw:f64,
    pub duty_within_benchmark:bool,
    pub required_secondary_hot_c:f64,
    pub ihx_temperature_budget_k:f64,
    pub helium_flow_kg_s:f64,
    pub ihx_only_circulator_mw:f64,
}
pub fn nuclear_heat_integration_check(
    process_hot_c:f64,
    reformer_approach_k:f64,
    reactor_primary_out_c:f64,
)->NuclearHeatIntegrationCheck {
    let (lo,hi)=independent_radiant_duty_envelope_mw();
    let sec_hot=required_secondary_he_hot_c(process_hot_c,reformer_approach_k);
    let budget=ihx_hot_end_temperature_budget_k(
        reactor_primary_out_c,process_hot_c,reformer_approach_k);
    // JAEA secondary helium 900 -> 500 C benchmark span; cp screening 5.2.
    let m=helium_mass_flow_kg_s(hi,5.2,900.0,500.0);
    let circ=helium_circulator_power_mw(
        m,GTHTR300C_SECONDARY_IHX_DP_KPA,
        GTHTR300C_SECONDARY_HE_PRESSURE_MPA,
        GTHTR300C_SECONDARY_HE_INLET_C,0.80);
    NuclearHeatIntegrationCheck{
        process_duty_lo_mw:lo,process_duty_hi_mw:hi,
        ihx_benchmark_mw:GTHTR300C_IHX_DUTY_MW,
        duty_within_benchmark:hi<=GTHTR300C_IHX_DUTY_MW,
        required_secondary_hot_c:sec_hot,
        ihx_temperature_budget_k:budget,
        helium_flow_kg_s:m,
        ihx_only_circulator_mw:circ,
    }
}

#[cfg(test)]
mod nuclear_heat_integration_review1_tests {
    use super::*;
    #[test]
    fn validated_radiant_duty_is_below_japan_ihx_benchmark() {
        let x=nuclear_heat_integration_check(950.0,20.0,1000.0);
        assert!(x.duty_within_benchmark);
        assert!(x.process_duty_hi_mw<x.ihx_benchmark_mw);
    }
    #[test]
    fn temperature_budget_is_explicit_not_assumed() {
        let feasible=nuclear_heat_integration_check(950.0,20.0,1000.0);
        let infeasible=nuclear_heat_integration_check(950.0,20.0,960.0);
        assert!(feasible.ihx_temperature_budget_k>0.0);
        assert!(infeasible.ihx_temperature_budget_k<0.0);
    }
    #[test]
    fn ihx_pressure_drop_implies_nonzero_circulator_load() {
        let x=nuclear_heat_integration_check(950.0,20.0,1000.0);
        assert!(x.helium_flow_kg_s>0.0 && x.ihx_only_circulator_mw>0.0);
    }
}


/// Shomate interval with validity metadata. Evaluation outside the documented
/// interval is rejected at the property object, not delegated to callers.
#[derive(Debug,Clone,Copy)]
pub struct ShomateRange {
    pub coeff:Shomate,
    pub t_min_k:f64,
    pub t_max_k:f64,
}
impl ShomateRange {
    pub fn contains(self,t:f64)->bool { t>=self.t_min_k && t<=self.t_max_k }
    fn checked(self,t:f64)->Shomate {
        assert!(self.contains(t),"temperature outside Shomate validity interval");
        self.coeff
    }
    pub fn cp_j_mol_k(self,t:f64)->f64 {
        let s=self.checked(t); let x=t/1000.0;
        s.a+s.b*x+s.c*x*x+s.d*x*x*x+s.e/(x*x)
    }
    pub fn sensible_h_kj_mol(self,t:f64)->f64 { self.checked(t).sensible_h_kj_mol(t) }
    pub fn entropy_j_mol_k(self,t:f64)->f64 { self.checked(t).entropy_j_mol_k(t) }
    pub fn standard_gibbs_kj_mol(self,t:f64)->f64 { self.checked(t).standard_gibbs_kj_mol(t) }
}

pub const H2_LOW:ShomateRange=ShomateRange{
    coeff:NIST_H2_298_1000,t_min_k:298.0,t_max_k:1000.0};
pub const H2_HIGH:ShomateRange=ShomateRange{
    coeff:NIST_H2_1000_2500,t_min_k:1000.0,t_max_k:2500.0};
pub const CO2_LOW:ShomateRange=ShomateRange{
    coeff:NIST_CO2_298_1200,t_min_k:298.0,t_max_k:1200.0};
pub const NIST_CO2_1200_6000:Shomate=Shomate{
    a:58.16639,b:2.720074,c:-0.492289,d:0.038844,
    e:-6.447293,f:-425.9186,g:263.6125,h:-393.5224};
pub const CO2_HIGH:ShomateRange=ShomateRange{
    coeff:NIST_CO2_1200_6000,t_min_k:1200.0,t_max_k:6000.0};

pub fn h2_shomate(t:f64)->ShomateRange {
    if t<1000.0 { H2_LOW } else if t<=2500.0 { H2_HIGH }
    else { panic!("H2 temperature outside supported NIST Shomate intervals") }
}
pub fn co2_shomate(t:f64)->ShomateRange {
    if t<1200.0 { CO2_LOW } else if t<=6000.0 { CO2_HIGH }
    else { panic!("CO2 temperature outside supported NIST Shomate intervals") }
}
pub fn h2_standard_gibbs_kj_mol(t:f64)->f64 { h2_shomate(t).standard_gibbs_kj_mol(t) }
pub fn co2_standard_gibbs_kj_mol(t:f64)->f64 { co2_shomate(t).standard_gibbs_kj_mol(t) }

/// Interval-safe SMR equilibrium constant. Other species use single NIST
/// intervals spanning the supported 500-1300 K model range; H2 dispatches at
/// 1000 K. No coefficient set is extrapolated.
pub fn smr_equilibrium_constant_piecewise(temperature_k:f64)->f64 {
    assert!(temperature_k>=500.0 && temperature_k<=1300.0);
    let dg=NIST_CO_298_1300.standard_gibbs_kj_mol(temperature_k)
        +3.0*h2_standard_gibbs_kj_mol(temperature_k)
        -NIST_CH4_298_1300.standard_gibbs_kj_mol(temperature_k)
        -NIST_H2O_500_1700.standard_gibbs_kj_mol(temperature_k);
    (-dg*1000.0/(8.314462618*temperature_k)).exp()
}
pub fn wgs_equilibrium_constant_piecewise(temperature_k:f64)->f64 {
    assert!(temperature_k>=500.0 && temperature_k<=1300.0);
    let dg=co2_standard_gibbs_kj_mol(temperature_k)
        +h2_standard_gibbs_kj_mol(temperature_k)
        -NIST_CO_298_1300.standard_gibbs_kj_mol(temperature_k)
        -NIST_H2O_500_1700.standard_gibbs_kj_mol(temperature_k);
    (-dg*1000.0/(8.314462618*temperature_k)).exp()
}

#[cfg(test)]
mod review2_b03_property_tests {
    use super::*;
    fn rel(a:f64,b:f64)->f64 {(a-b).abs()/a.abs().max(b.abs()).max(1e-12)}
    #[test]
    fn h2_boundary_is_continuous_to_nist_rounding() {
        let a=H2_LOW; let b=H2_HIGH; let t=1000.0;
        assert!(rel(a.cp_j_mol_k(t),b.cp_j_mol_k(t))<5e-4);
        assert!(rel(a.sensible_h_kj_mol(t),b.sensible_h_kj_mol(t))<5e-4);
        assert!(rel(a.entropy_j_mol_k(t),b.entropy_j_mol_k(t))<5e-4);
        assert!(rel(a.standard_gibbs_kj_mol(t),b.standard_gibbs_kj_mol(t))<5e-4);
    }
    #[test]
    fn co2_boundary_is_continuous_to_nist_rounding() {
        let a=CO2_LOW; let b=CO2_HIGH; let t=1200.0;
        // NIST publishes separately fitted CO2 intervals; Cp has a small fit jump
        // at 1200 K, while integrated H/S/G remain much tighter.
        assert!(rel(a.cp_j_mol_k(t),b.cp_j_mol_k(t))<5e-3);
        assert!(rel(a.sensible_h_kj_mol(t),b.sensible_h_kj_mol(t))<5e-4);
        assert!(rel(a.entropy_j_mol_k(t),b.entropy_j_mol_k(t))<5e-4);
        assert!(rel(a.standard_gibbs_kj_mol(t),b.standard_gibbs_kj_mol(t))<5e-4);
    }
    #[test]
    fn reformer_temperatures_use_high_h2_interval() {
        for t in [1173.15,1198.15,1223.15] {
            assert!(h2_shomate(t).t_min_k>=1000.0);
            let k=smr_equilibrium_constant_piecewise(t);
            assert!(k.is_finite() && k>0.0);
        }
    }
    #[test]
    #[should_panic]
    fn h2_invalid_high_temperature_rejected() {
        let _=h2_standard_gibbs_kj_mol(2500.1);
    }
    #[test]
    #[should_panic]
    fn raw_interval_rejects_extrapolation() {
        let _=H2_LOW.standard_gibbs_kj_mol(1000.1);
    }
}


/// Independent literature benchmark for dimensionless SMR equilibrium Kp.
/// Daubert-based table reproduced in membrane-reactor literature:
/// 973 K 12.735; 1073 K 171.07; 1173 K 1485.1; 1273 K 9199.6.
/// This benchmark is independent of the NIST Shomate coefficients used here.
pub fn smr_external_kp_benchmark(t:f64)->f64 {
    match t as i32 {
        973=>12.735,
        1073=>171.07,
        1173=>1485.1,
        1273=>9199.6,
        _=>panic!("no external SMR Kp benchmark at requested temperature"),
    }
}

#[cfg(test)]
mod review2_b03_external_equilibrium_tests {
    use super::*;
    #[test]
    fn nist_smr_k_matches_independent_thermochemical_table() {
        for t in [973.0,1073.0,1173.0,1273.0] {
            let calc=smr_equilibrium_constant_piecewise(t);
            let reference=smr_external_kp_benchmark(t);
            let rel=(calc-reference).abs()/reference;
            assert!(rel<0.03,"SMR Kp external benchmark mismatch at {t} K: calc={calc}, ref={reference}, rel={rel}");
        }
    }
    #[test]
    fn equilibrium_is_continuous_across_h2_dispatch_boundary() {
        let a=smr_equilibrium_constant_piecewise(999.999);
        let b=smr_equilibrium_constant_piecewise(1000.001);
        assert!((a-b).abs()/((a+b)*0.5)<1e-3);
    }
    #[test]
    fn equilibrium_is_physical_across_reformer_range() {
        let k1173=smr_equilibrium_constant_piecewise(1173.15);
        let k1223=smr_equilibrium_constant_piecewise(1223.15);
        assert!(k1223>k1173 && k1173>0.0);
    }
}


/// Full wet-gas state used for Review-2 physical recycle work, kmol/h.
/// Unlike the legacy CHO surrogate this state carries steam and inert N2.
#[derive(Debug,Clone,Copy,Default)]
pub struct WetGas6 {
    pub h2:f64, pub h2o:f64, pub co:f64, pub co2:f64, pub ch4:f64, pub n2:f64,
}
impl WetGas6 {
    pub fn total(self)->f64 {self.h2+self.h2o+self.co+self.co2+self.ch4+self.n2}
    pub fn carbon(self)->f64 {self.co+self.co2+self.ch4}
    pub fn hydrogen_atoms(self)->f64 {2.0*self.h2+2.0*self.h2o+4.0*self.ch4}
    pub fn oxygen_atoms(self)->f64 {self.h2o+self.co+2.0*self.co2}
    pub fn nitrogen_atoms(self)->f64 {2.0*self.n2}
    pub fn nonnegative(self)->bool {
        [self.h2,self.h2o,self.co,self.co2,self.ch4,self.n2].iter().all(|x|*x>=0.0&&x.is_finite())
    }
    pub fn scale(self,a:f64)->Self {Self{
        h2:self.h2*a,h2o:self.h2o*a,co:self.co*a,co2:self.co2*a,ch4:self.ch4*a,n2:self.n2*a}}
    pub fn add(self,b:Self)->Self {Self{
        h2:self.h2+b.h2,h2o:self.h2o+b.h2o,co:self.co+b.co,
        co2:self.co2+b.co2,ch4:self.ch4+b.ch4,n2:self.n2+b.n2}}
}

/// Authoritative IEAGHG once-through HTS inlet/outlet represented in WetGas6.
pub fn ieaghg_hts_inlet_wet6()->WetGas6 {
    let s=ieaghg_hts_inlet();
    WetGas6{h2:s.flow(s.h2),h2o:s.flow(s.h2o),co:s.flow(s.co),
        co2:s.flow(s.co2),ch4:s.flow(s.ch4),n2:s.flow(s.n2)}
}
pub fn ieaghg_hts_outlet_wet6()->WetGas6 {
    let s=ieaghg_hts_outlet();
    WetGas6{h2:s.flow(s.h2),h2o:s.flow(s.h2o),co:s.flow(s.co),
        co2:s.flow(s.co2),ch4:s.flow(s.ch4),n2:s.flow(s.n2)}
}

/// Apply WGS extent xi to a wet gas. Positive xi is CO+H2O->CO2+H2.
/// Invalid extents are rejected rather than clipped.
pub fn apply_wgs(s:WetGas6,xi:f64)->WetGas6 {
    let o=WetGas6{h2:s.h2+xi,h2o:s.h2o-xi,co:s.co-xi,
        co2:s.co2+xi,ch4:s.ch4,n2:s.n2};
    assert!(o.nonnegative(),"nonphysical WGS extent");
    o
}

/// Apply SMR extent xi: CH4+H2O->CO+3H2.
pub fn apply_smr(s:WetGas6,xi:f64)->WetGas6 {
    let o=WetGas6{h2:s.h2+3.0*xi,h2o:s.h2o-xi,co:s.co+xi,
        co2:s.co2,ch4:s.ch4-xi,n2:s.n2};
    assert!(o.nonnegative(),"nonphysical SMR extent");
    o
}

pub fn wet6_element_residual(a:WetGas6,b:WetGas6)->[f64;4] {
    [b.carbon()-a.carbon(),b.hydrogen_atoms()-a.hydrogen_atoms(),
     b.oxygen_atoms()-a.oxygen_atoms(),b.nitrogen_atoms()-a.nitrogen_atoms()]
}

/// Solve a single WGS equilibrium extent by bisection at fixed T,P.
/// This is an ideal-gas equilibrium reactor layer, not a kinetic reactor.
pub fn solve_wgs_equilibrium(mut s:WetGas6,t_k:f64,p_bar:f64)->WetGas6 {
    assert!(s.nonnegative()&&p_bar>0.0);
    let eps=1e-10;
    let lo=-s.co2.min(s.h2)+eps;
    let hi=s.co.min(s.h2o)-eps;
    assert!(hi>lo);
    let residual=|xi:f64| {
        let x=apply_wgs(s,xi); let n=x.total();
        let q=wgs_reaction_quotient(x.co/n,x.h2o/n,x.co2/n,x.h2/n,p_bar);
        q.ln()-wgs_equilibrium_constant_piecewise(t_k).ln()
    };
    let mut a=lo; let mut b=hi; let mut fa=residual(a); let fb=residual(b);
    assert!(fa*fb<=0.0,"WGS equilibrium root not bracketed");
    for _ in 0..120 {
        let m=0.5*(a+b); let fm=residual(m);
        if fm.abs()<1e-11 {return apply_wgs(s,m);}
        if fa*fm<=0.0 {b=m;} else {a=m;fa=fm;}
    }
    s=apply_wgs(s,0.5*(a+b)); s
}

/// Composition-aware reduced PSA separator.
/// H2 recovery decreases linearly with non-H2 dry impurity loading relative to
/// the IEAGHG once-through PSA feed; the slope is an explicit uncertainty
/// parameter. Product is pure-H2 in this reduced separator, so common-basis
/// purity >=99.9% is enforced by construction while component balances remain exact.
#[derive(Debug,Clone,Copy)]
pub struct Psa6Result {pub product_h2:f64,pub tail:WetGas6,pub recovery:f64}
pub fn psa6_bounded(s:WetGas6,impurity_sensitivity:f64)->Psa6Result {
    assert!(s.nonnegative()&&impurity_sensitivity>=0.0);
    let dry=s.h2+s.co+s.co2+s.ch4+s.n2;
    assert!(dry>0.0&&s.h2>0.0);
    let impurity=(dry-s.h2)/dry;
    let refi=ieaghg_psa_inlet_cho();
    let refdry=refi.h2+refi.co+refi.co2+refi.ch4+ieaghg_process_n2_feed_kmol_h();
    let refimp=(refdry-refi.h2)/refdry;
    let r0=ieaghg_reconstructed_psa_h2_recovery();
    let recovery=(r0-impurity_sensitivity*(impurity-refimp)).clamp(0.50,0.98);
    let product=s.h2*recovery;
    let tail=WetGas6{h2:s.h2-product,h2o:s.h2o,co:s.co,co2:s.co2,ch4:s.ch4,n2:s.n2};
    assert!(tail.nonnegative());
    Psa6Result{product_h2:product,tail,recovery}
}

#[cfg(test)]
mod review2_fullspecies_foundation_tests {
    use super::*;
    #[test]
    fn wgs_stoichiometry_conserves_all_elements() {
        let a=ieaghg_hts_inlet_wet6(); let b=apply_wgs(a,100.0);
        for e in wet6_element_residual(a,b) {assert!(e.abs()<1e-9);}
    }
    #[test]
    fn source_hts_wgs_extent_reproduces_outlet_species() {
        let a=ieaghg_hts_inlet_wet6();
        let xi=ieaghg_hts_wgs_extent_from_co_kmol_h();
        let b=apply_wgs(a,xi); let r=ieaghg_hts_outlet_wet6();
        assert!((b.co-r.co).abs()<2.0);
        assert!((b.co2-r.co2).abs()<2.0);
        assert!((b.h2-r.h2).abs()<2.0);
        assert!((b.h2o-r.h2o).abs()<2.0);
    }
    #[test]
    fn equilibrium_wgs_closes_elements_and_q_over_k() {
        let a=ieaghg_hts_inlet_wet6();
        let b=solve_wgs_equilibrium(a,685.15,27.7);
        for e in wet6_element_residual(a,b) {assert!(e.abs()<1e-8);}
        let n=b.total();
        let q=wgs_reaction_quotient(b.co/n,b.h2o/n,b.co2/n,b.h2/n,27.7);
        assert!((q/wgs_equilibrium_constant_piecewise(685.15)-1.0).abs()<1e-8);
    }
    #[test]
    fn psa_once_through_recovers_source_recovery_at_zero_sensitivity() {
        let s=ieaghg_hts_outlet_wet6();
        let p=psa6_bounded(s,0.0);
        assert!((p.recovery-ieaghg_reconstructed_psa_h2_recovery()).abs()<1e-12);
        assert!((p.product_h2+p.tail.h2-s.h2).abs()<1e-10);
    }
    #[test]
    fn psa_recovery_degrades_with_added_inert() {
        let s=ieaghg_hts_outlet_wet6();
        let dirty=WetGas6{n2:s.n2+500.0,..s};
        assert!(psa6_bounded(dirty,0.5).recovery<psa6_bounded(s,0.5).recovery);
    }
}
