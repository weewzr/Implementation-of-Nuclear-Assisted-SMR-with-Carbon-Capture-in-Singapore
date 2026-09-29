# Canonical R3 results — reproducibility contract

**Status:** machine-derived result schema for the corrected R3 model.

The authoritative implementation is `r3_canonical_results()` in
`model/src/lib.rs`. This artifact records the exact fields and scientific
interpretation that a fresh checkout must reproduce. It deliberately preserves
adverse results rather than selecting favourable cases.

## Canonical physical state

Generated from:
- `r3_canonical_recycle_case()`
- `r3_heat_cascade(650 C, 20 K process approach, 30 K IHX approach, 500 C He return)`
- `r3_ccs_ledger()`
- `r3_singapore_scale()`

Fields:
- fresh external-feed fraction;
- purge fraction;
- annual H2 production, t/y;
- captured CO2 sent to storage, t/y;
- residual direct CO2, t/y;
- high-side nuclear process heat, MWth;
- secondary-He hot temperature, C;
- secondary-He mass flow, kg/s.

## Canonical threshold state

Generated from:
- `r3_reference_threshold_case()`
- `r3_conservative_threshold_case()`
- `r3_uncertainty_summary()`

Fields:
- reference lifecycle intensity, kgCO2e/kgH2;
- reference annual avoided tCO2e/y;
- reference forward abatement cost, S$/tCO2e;
- reference pass/fail for >0.25 Mt/y;
- reference pass/fail for <S$100/t;
- conservative annual avoided tCO2e/y;
- conservative pass/fail flags;
- uncertainty design point count;
- uncertainty joint-pass count.

## Adverse results that MUST reproduce

- Conservative credible case: annual-abatement threshold **FAIL** and economic
  threshold **FAIL**; when lifecycle abatement is non-positive, abatement cost
  is represented as +infinity rather than forced through the cost equation.
- Coupled uncertainty design cardinality: **64 points**.
- Joint CN4252 passes in that design: **0**.
- IEAGHG Case 1A at the common H2 scale exceeds 0.25 Mt/y direct avoided CO2.

## Important correction history

An earlier Review-3 narrative incorrectly called the uncertainty design
128-point. Gate-4 adversarial testing exposed the counting error. The actual
Cartesian design is 2^6 = 64 points. The zero-joint-pass scientific result is
unchanged. Review records were corrected rather than preserving the stale count.

## Reproduction

From a fresh checkout, run:

`cargo test --all-targets`

The Gate-4 test
`r3_m04_gate4_adversarial_tests::g_canonical_results_snapshot_is_self_consistent`
requires the canonical snapshot to retain 64 uncertainty points, zero joint
passes, positive H2/capture scale and a secondary-He hot temperature above the
900 C process hot end.

This Markdown file is not an independently maintained numerical model. The Rust
functions are canonical; this file is the human-readable result contract.
