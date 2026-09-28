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
    close(ieaghg_reconstructed_psa_h2_recovery(), 0.8992, 0.001);
}

#[test]
fn published_psa_streams_reconstruct_h2_product_flow() {
    use nuclear_assisted_smr::ieaghg_reconstructed_h2_product_kmol_per_h;
    close(ieaghg_reconstructed_h2_product_kmol_per_h(), 4453.5, 2.0);
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
