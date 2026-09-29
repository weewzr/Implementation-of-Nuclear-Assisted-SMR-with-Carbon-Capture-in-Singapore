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
