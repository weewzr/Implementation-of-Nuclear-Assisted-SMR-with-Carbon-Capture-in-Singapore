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
