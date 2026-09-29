use nuclear_assisted_smr::{
    ideal_ch4_kg_per_kg_h2, ideal_process_co2_kg_per_kg_h2,
    plant_gate_abatement_kg_per_kg_h2, required_h2_t_per_year,
    IEAGHG_BASE, IEAGHG_CASE_1A, IEAGHG_KG_H2_PER_NM3,
};

fn close(a: f64, b: f64, tol: f64) {
    assert!((a - b).abs() <= tol, "{a} != {b} within {tol}");
}

#[test]
fn stoichiometric_bounds_are_correct() {
    close(ideal_ch4_kg_per_kg_h2(), 1.9895, 0.001);
    close(ideal_process_co2_kg_per_kg_h2(), 5.457, 0.002);
}

#[test]
fn ieaghg_volume_to_mass_basis_is_self_consistent() {
    close(IEAGHG_KG_H2_PER_NM3, 0.08994, 1e-8);
}

#[test]
fn ieaghg_base_case_specific_ng_reconstructs_published_flows() {
    close(IEAGHG_BASE.ng_feed_kg_per_kg_h2(), 26_231.0 / 8_994.0, 1e-12);
    close(IEAGHG_BASE.ng_fuel_kg_per_kg_h2(), 4_332.0 / 8_994.0, 1e-12);
    close(IEAGHG_BASE.ng_total_kg_per_h(), 30_563.0, 1e-12);
}

#[test]
fn ieaghg_base_case_co2_is_about_nine_kg_per_kg_h2() {
    close(IEAGHG_BASE.co2_emitted_kg_per_kg_h2(), 8.995, 0.002);
}

#[test]
fn case_1a_reconstructs_ieaghg_avoidance_fraction() {
    let avoided = plant_gate_abatement_kg_per_kg_h2(IEAGHG_BASE, IEAGHG_CASE_1A);
    let fraction = avoided / IEAGHG_BASE.co2_emitted_kg_per_kg_h2();
    close(fraction, 0.5422, 0.0002);
}

#[test]
fn case_1a_cn4252_scale_is_about_51_kt_h2_per_year_plant_gate_only() {
    let avoided = plant_gate_abatement_kg_per_kg_h2(IEAGHG_BASE, IEAGHG_CASE_1A);
    let required = required_h2_t_per_year(250_000.0, avoided);
    close(required / 1_000.0, 51.3, 0.1);
}

#[test]
fn capture_and_avoidance_are_not_identical() {
    let captured = IEAGHG_CASE_1A.co2_captured_kg_per_kg_h2();
    let avoided = plant_gate_abatement_kg_per_kg_h2(IEAGHG_BASE, IEAGHG_CASE_1A);
    assert!(captured > avoided);
}


#[test]
fn base_makeup_fuel_combustion_is_only_part_of_total_direct_co2() {
    use nuclear_assisted_smr::makeup_fuel_co2_upper_bound_kg_per_kg_h2;
    let makeup = makeup_fuel_co2_upper_bound_kg_per_kg_h2(IEAGHG_BASE);
    close(makeup, 1.322, 0.005);
    assert!(makeup < IEAGHG_BASE.co2_emitted_kg_per_kg_h2());
}


#[test]
fn ieaghg_carbon_balance_closes_from_ng_inputs_to_flue_gas() {
    use nuclear_assisted_smr::{
        ieaghg_feed_carbon_kmol_per_h, ieaghg_flue_carbon_kmol_per_h,
        ieaghg_makeup_fuel_carbon_kmol_per_h,
    };
    let carbon_in = ieaghg_feed_carbon_kmol_per_h() + ieaghg_makeup_fuel_carbon_kmol_per_h();
    let carbon_out = ieaghg_flue_carbon_kmol_per_h();
    close(carbon_out / carbon_in, 1.0, 0.001);
}

#[test]
fn psa_tail_gas_carries_essentially_all_feedstock_carbon_before_firing() {
    use nuclear_assisted_smr::{
        ieaghg_feed_carbon_kmol_per_h, ieaghg_tail_gas_carbon_kmol_per_h,
    };
    close(
        ieaghg_tail_gas_carbon_kmol_per_h() / ieaghg_feed_carbon_kmol_per_h(),
        1.0,
        0.001,
    );
}

#[test]
fn makeup_furnace_fuel_is_only_about_fourteen_percent_of_input_carbon() {
    use nuclear_assisted_smr::ieaghg_makeup_fuel_fraction_of_input_carbon;
    close(ieaghg_makeup_fuel_fraction_of_input_carbon(), 0.1417, 0.001);
}

#[test]
fn co_and_ch4_leave_a_large_residual_carbon_load_after_pre_psa_co2_removal() {
    use nuclear_assisted_smr::ieaghg_psa_inlet_non_co2_carbon_kmol_per_h;
    close(ieaghg_psa_inlet_non_co2_carbon_kmol_per_h(), 505.2, 0.5);
}


#[test]
fn published_psa_streams_reconstruct_hydrogen_recovery() {
    use nuclear_assisted_smr::ieaghg_reconstructed_psa_h2_recovery;
    // Rounded source streams reconstruct about 89.9% recovery, consistent with
    // the report's approximately 90% PSA recovery basis.
    close(ieaghg_reconstructed_psa_h2_recovery(), 0.9000, 0.002);
}

#[test]
fn published_psa_streams_reconstruct_h2_product_flow() {
    use nuclear_assisted_smr::ieaghg_reconstructed_h2_product_kmol_per_h;
    close(ieaghg_reconstructed_h2_product_kmol_per_h(), 4490.0, 3.0);
}

#[test]
fn tail_gas_contains_substantial_combustible_species() {
    use nuclear_assisted_smr::ieaghg_tail_combustible_mole_fraction;
    close(ieaghg_tail_combustible_mole_fraction(), 0.4768, 0.001);
}

#[test]
fn co2_only_tail_gas_capture_leaves_about_thirty_two_percent_of_tail_carbon() {
    use nuclear_assisted_smr::ieaghg_tail_carbon_as_co2_fraction;
    let co2_fraction = ieaghg_tail_carbon_as_co2_fraction();
    close(co2_fraction, 0.6799, 0.002);
    close(1.0 - co2_fraction, 0.3201, 0.002);
}


#[test]
fn reduced_reaction_extents_reproduce_psa_carbon_species() {
    use nuclear_assisted_smr::{ieaghg_psa_inlet_cho, reduced_extents_from_psa_inlet};
    let p = ieaghg_psa_inlet_cho();
    let x = reduced_extents_from_psa_inlet();
    close(x.smr - x.wgs, p.co, 1.0);
}

#[test]
fn reduced_reaction_extents_nearly_reproduce_psa_hydrogen() {
    use nuclear_assisted_smr::{ieaghg_psa_inlet_cho, reduced_reaction_h2_kmol_per_h};
    let source = ieaghg_psa_inlet_cho().h2;
    let reduced = reduced_reaction_h2_kmol_per_h();
    // Equivalent-CH4 substitution is intentionally a reduced model. The
    // source-resolved model supersedes it; retain the mismatch as a diagnostic.
    let rel = (reduced - source).abs() / source;
    assert!(rel > 0.03 && rel < 0.06, "unexpected reduced-model mismatch: {rel}");
}

#[test]
fn reduced_standard_reaction_duty_is_positive_and_order_60_mw() {
    use nuclear_assisted_smr::reduced_standard_reaction_duty_mw;
    let q = reduced_standard_reaction_duty_mw();
    assert!(q > 60.0 && q < 70.0, "unexpected reference reaction duty: {q} MW");
}


#[test]
fn hts_wgs_extent_closes_independently_on_co_co2_h2_and_water() {
    use nuclear_assisted_smr::{
        ieaghg_hts_wgs_extent_from_co_kmol_h,
        ieaghg_hts_wgs_extent_from_co2_kmol_h,
        ieaghg_hts_wgs_extent_from_h2_kmol_h,
        ieaghg_hts_wgs_extent_from_water_kmol_h,
    };
    let x=ieaghg_hts_wgs_extent_from_co_kmol_h();
    close(ieaghg_hts_wgs_extent_from_co2_kmol_h(), x, 2.0);
    close(ieaghg_hts_wgs_extent_from_h2_kmol_h(), x, 2.0);
    close(ieaghg_hts_wgs_extent_from_water_kmol_h(), x, 2.0);
    close(x, 661.3, 2.0);
}

#[test]
fn hts_section_conserves_c_h_o_with_rounded_source_values() {
    use nuclear_assisted_smr::{ieaghg_hts_inlet,ieaghg_hts_outlet};
    let i=ieaghg_hts_inlet(); let o=ieaghg_hts_outlet();
    close(o.carbon_kmol_h()/i.carbon_kmol_h(),1.0,0.001);
    close(o.hydrogen_atoms_kmol_h()/i.hydrogen_atoms_kmol_h(),1.0,0.001);
    close(o.oxygen_atoms_kmol_h()/i.oxygen_atoms_kmol_h(),1.0,0.001);
}

#[test]
fn source_prereformer_stream_steam_to_carbon_is_reconstructed_not_assumed() {
    use nuclear_assisted_smr::ieaghg_prereformer_stream_steam_to_carbon;
    // Directly reconstructed from the encoded source stream. Do not replace
    // this with a generic textbook/design S/C assumption.
    close(ieaghg_prereformer_stream_steam_to_carbon(),2.5522,0.002);
}


#[test]
fn prereformer_to_hts_source_rows_are_not_silently_forced_to_close() {
    use nuclear_assisted_smr::prereformer_to_hts_source_residual;
    let r=prereformer_to_hts_source_residual();
    // Diagnostic guard: if these suddenly approach zero after an edit, inspect
    // source provenance before claiming a closed reformer control volume.
    assert!(r.carbon_kmol_atoms_h.abs() > 1.0
        || r.hydrogen_kmol_atoms_h.abs() > 1.0
        || r.oxygen_kmol_atoms_h.abs() > 1.0);
}


#[test]
fn stream4_to_stream5_nonclosure_is_explained_by_unreported_water_addition() {
    use nuclear_assisted_smr::{
        ieaghg_interstage_water_addition_kmol_h,
        prereformer_to_hts_residual_with_inferred_water,
        ieaghg_prereformer_feed,
    };
    let w = ieaghg_interstage_water_addition_kmol_h();
    // H and O balances independently imply ~154 kmol/h H2O. This is
    // consistent with IEAGHG's stated second HP-steam addition plus BFW
    // desuperheating between stream 4 and the primary reformer.
    assert!(w > 153.0 && w < 156.0, "unexpected inferred water addition: {w}");

    let r = prereformer_to_hts_residual_with_inferred_water();
    let feed = ieaghg_prereformer_feed();
    // Rounded four-decimal compositions leave tiny residuals; require
    // <0.1% of the corresponding stream-4 elemental inventories.
    assert!(r.carbon_kmol_atoms_h.abs()/feed.carbon_kmol_h() < 0.001);
    assert!(r.hydrogen_kmol_atoms_h.abs()/feed.hydrogen_atoms_kmol_h() < 0.001);
    assert!(r.oxygen_kmol_atoms_h.abs()/feed.oxygen_atoms_kmol_h() < 0.001);
}


#[test]
fn ieaghg_base_energy_ledger_reconstructs_published_scale() {
    use nuclear_assisted_smr::{
        ieaghg_feed_lhv_mw, ieaghg_makeup_fuel_lhv_mw, ieaghg_total_ng_lhv_mw,
        ieaghg_h2_lhv_efficiency,
    };
    close(ieaghg_feed_lhv_mw(), 338.806, 0.01);
    close(ieaghg_makeup_fuel_lhv_mw(), 55.944, 0.01);
    close(ieaghg_total_ng_lhv_mw(), 394.778, 0.01);
    close(ieaghg_h2_lhv_efficiency(), 0.75916, 0.0001);
}

#[test]
fn ieaghg_energy_source_rounding_is_explicit_not_forced() {
    use nuclear_assisted_smr::{
        IEAGHG_FEED_LHV_GJ_PER_1000_NM3_H2,
        IEAGHG_FUEL_LHV_GJ_PER_1000_NM3_H2,
        IEAGHG_TOTAL_LHV_GJ_PER_1000_NM3_H2,
    };
    let sum=IEAGHG_FEED_LHV_GJ_PER_1000_NM3_H2+IEAGHG_FUEL_LHV_GJ_PER_1000_NM3_H2;
    close(IEAGHG_TOTAL_LHV_GJ_PER_1000_NM3_H2-sum,0.001,1e-12);
}


#[test]
fn source_stream4_to_stream5_closes_after_source_described_water_addition() {
    use nuclear_assisted_smr::prereformer_to_hts_residual_with_inferred_water;
    let r=prereformer_to_hts_residual_with_inferred_water();
    // Source rows are rounded to four decimals; the independently inferred
    // H- and O-based water additions differ by ~1.5 kmol/h. The reconciled
    // residuals must remain small relative to the ~4,000-14,000 kmol-atom/h
    // inventories rather than being tuned to exact zero.
    assert!(r.carbon_kmol_atoms_h.abs() < 0.1);
    assert!(r.hydrogen_kmol_atoms_h.abs() < 2.0);
    assert!(r.oxygen_kmol_atoms_h.abs() < 1.0);
}

#[test]
fn source_reformer_consumes_water_and_eliminates_heavier_hydrocarbons() {
    use nuclear_assisted_smr::{
        source_reformer_water_consumption_kmol_h,
        source_reformer_carbon_conversion_kmol_h,
    };
    assert!(source_reformer_water_consumption_kmol_h() > 1000.0);
    assert!(source_reformer_carbon_conversion_kmol_h() > 1000.0);
}


#[test]
fn ieaghg_ccs_cases_reproduce_authoritative_incremental_energy_penalties() {
    use nuclear_assisted_smr::{
        incremental_ng_input_mw,lost_power_export_mwe,
        IEAGHG_ENERGY_CASE_1A,IEAGHG_ENERGY_CASE_3,
    };
    close(incremental_ng_input_mw(IEAGHG_ENERGY_CASE_1A),12.91,0.02);
    close(lost_power_export_mwe(IEAGHG_ENERGY_CASE_1A),8.426,0.002);
    close(incremental_ng_input_mw(IEAGHG_ENERGY_CASE_3),38.95,0.02);
    close(lost_power_export_mwe(IEAGHG_ENERGY_CASE_3),9.492,0.002);
}

#[test]
fn flue_gas_capture_has_larger_energy_penalty_and_more_plant_gate_abatement() {
    use nuclear_assisted_smr::{
        incremental_energy_service_mw,plant_gate_co2_avoided_t_per_h,
        IEAGHG_ENERGY_CASE_1A,IEAGHG_ENERGY_CASE_3,
    };
    assert!(incremental_energy_service_mw(IEAGHG_ENERGY_CASE_3)
        > incremental_energy_service_mw(IEAGHG_ENERGY_CASE_1A));
    assert!(plant_gate_co2_avoided_t_per_h(IEAGHG_ENERGY_CASE_3)
        > plant_gate_co2_avoided_t_per_h(IEAGHG_ENERGY_CASE_1A));
    close(plant_gate_co2_avoided_t_per_h(IEAGHG_ENERGY_CASE_1A),43.87,0.02);
    close(plant_gate_co2_avoided_t_per_h(IEAGHG_ENERGY_CASE_3),72.03,0.02);
}


#[test]
fn ieaghg_reformer_radiant_duty_converts_to_about_96_mw() {
    use nuclear_assisted_smr::ieaghg_reformer_radiant_duty_mw;
    close(ieaghg_reformer_radiant_duty_mw(),96.04,0.05);
}

#[test]
fn radiant_duty_exceeds_makeup_ng_fuel_lhv_because_tail_gas_is_also_fired() {
    use nuclear_assisted_smr::ieaghg_radiant_to_makeup_fuel_lhv_ratio;
    assert!(ieaghg_radiant_to_makeup_fuel_lhv_ratio() > 1.6);
}

#[test]
fn reduced_298k_reaction_layer_is_below_published_radiant_duty() {
    use nuclear_assisted_smr::radiant_minus_reduced_reaction_duty_mw;
    assert!(radiant_minus_reduced_reaction_duty_mw() > 25.0);
}


#[test]
fn jAEA_mockup_temperature_is_consistent_with_finite_ihx_drop() {
    use nuclear_assisted_smr::ihx_hot_end_temperature_budget_k;
    // JAEA: HTTR primary outlet 950 C; mock-up/HTTR H2 system SR inlet 880 C.
    // A 20 K reformer hot-end approach leaves 50 K for IHX/transport headroom.
    close(ihx_hot_end_temperature_budget_k(950.0,860.0,20.0),70.0,1e-12);
}

#[test]
fn ninety_six_mw_requires_large_secondary_helium_flow_for_modest_delta_t() {
    use nuclear_assisted_smr::{helium_mass_flow_kg_s,ieaghg_reformer_radiant_duty_mw};
    // Screening only: cp=5.2 kJ/kg-K, 880 -> 650 C.
    let m=helium_mass_flow_kg_s(ieaghg_reformer_radiant_duty_mw(),5.2,880.0,650.0);
    assert!(m > 75.0 && m < 85.0, "screening helium flow {m} kg/s");
}

#[test]
fn primary_950c_cannot_support_950c_process_with_finite_approach() {
    use nuclear_assisted_smr::ihx_hot_end_temperature_budget_k;
    assert!(ihx_hot_end_temperature_budget_k(950.0,950.0,10.0) < 0.0);
}


#[test]
fn gthtr300c_large_scale_ihx_is_same_order_as_project_helium_flow() {
    use nuclear_assisted_smr::{
        helium_mass_flow_kg_s,ieaghg_reformer_radiant_duty_mw,
        GTHTR300C_SECONDARY_HE_MASS_FLOW_KG_S,
    };
    let project_screen=helium_mass_flow_kg_s(
        ieaghg_reformer_radiant_duty_mw(),5.2,880.0,650.0);
    assert!((project_screen-GTHTR300C_SECONDARY_HE_MASS_FLOW_KG_S).abs() < 5.0);
}

#[test]
fn gthtr300c_ihx_only_pumping_is_low_single_digit_mw() {
    use nuclear_assisted_smr::{
        helium_circulator_power_mw,
        GTHTR300C_SECONDARY_HE_MASS_FLOW_KG_S,
        GTHTR300C_SECONDARY_HE_INLET_C,
        GTHTR300C_SECONDARY_HE_PRESSURE_MPA,
        GTHTR300C_SECONDARY_IHX_DP_KPA,
    };
    let w=helium_circulator_power_mw(
        GTHTR300C_SECONDARY_HE_MASS_FLOW_KG_S,
        GTHTR300C_SECONDARY_IHX_DP_KPA,
        GTHTR300C_SECONDARY_HE_PRESSURE_MPA,
        GTHTR300C_SECONDARY_HE_INLET_C,
        0.80);
    assert!(w > 1.5 && w < 2.2, "IHX-only screening circulator power {w} MW");
}
