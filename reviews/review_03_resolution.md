# Independent Review 3 resolution record

Independent Review 3 / Gate 4: **OPEN**.

This file tracks corrective work against
`reviews/review_03_integrated_model_results.md`. It is not a new review.

## R3-B01 — One physical canonical stream graph

**Disposition: RESOLVED for the declared screening-model scope.**

### Correction
The new R3 canonical path no longer scales the already-reacted HTS-inlet gas as
fresh feed. It begins from IEAGHG stream 4 pre-reformer feed. C2+ hydrocarbons
are reduced into WetGas6 with an explicit complete-steam-prereforming
stoichiometric screen that conserves C/H/O, then the source-reconstructed
interstage water is added.

The ordered loop is now:
external NG/steam-equivalent feed + recycle -> coupled SMR/WGS reformer ->
HTS/WGS -> explicit CO2 removal -> PSA -> N2 purge -> recycle.

Captured CO2 is removed from the downstream state before PSA and therefore
cannot simultaneously remain in recycle.

### Code/equation location
`model/src/lib.rs`:
- `r3_ieaghg_external_reformer_feed_wet6`
- `remove_co2_fraction`
- `r3_solve_at_fixed_fresh`
- `r3_canonical_recycle_case`
- `r3_external_element_residuals`
- `r3_external_normalized_element_residuals`
- `wet6_mass_kg_h`
- `r3_external_mass_residual_fraction`
- `r3_once_through_reformer_relative_errors`

### Validation evidence
The canonical external boundary treats recycle as internal and explicitly
closes fresh external feed against H2 product + captured CO2 + purge.
Normalized C/H/O/N residuals and total-mass residual are required below 1e-6.

The zero-recycle external-feed reformer is independently compared with IEAGHG
stream 5 before recycle fitting. Because the exact pre-reformer outlet and
primary-reformer inlet are unpublished and the prereformer is a reduced
stoichiometric screen, the source-compatibility criterion is deliberately broad:
major reactive species must be within 50% of the rounded source row and N2
within 2%. This is a screening validation, not an exact source-state claim.

### CI evidence
- commit `0b4daaa08734c2b2e738850497a13dea639f04da`, CI run 36540122585: PASS.
- commit `81e272bbda62d1ad4780667935151d2a398b22b1`, CI run 36540261744: PASS.

### Acceptance criterion status
- actual source NG/steam-derived external state: PASS for reduced prereformer scope
- physical unit ordering: PASS
- captured CO2 absent downstream: PASS
- normalized plant C/H/O/N residuals <=1e-6: PASS
- plant total-mass residual <=1e-6: PASS
- fixed H2 product and finite purge: PASS
- zero-recycle source compatibility: PASS at declared screening tolerance

R3-B01 is closed. This does not validate detailed prereformer kinetics.

## R3-B02 — Temperature-feasible integrated heat cascade

**Disposition: RESOLVED for the declared bounded screening scope.**

### Correction
The R3 heat cascade consumes the corrected R3-B01 canonical stream state and a
new R3 CCS ledger. Candidate MDEA regeneration duty is therefore derived from
the same captured-CO2/purge-carbon state used by the flowsheet.

The process hot end is 900 C. A positive 20 K process approach requires
secondary helium at 920 C; a further 30 K IHX approach requires a 950 C primary
outlet. The previous 900 C helium -> 900 C process pairing is no longer used on
the R3 path.

Reformer external duty is total-enthalpy difference on the corrected stream
state. WHB heat recovery is reported explicitly and credited at most once
against the candidate-scaled low-grade MDEA regeneration duty. Remaining MDEA
heat plus reformer external heat defines the nuclear-process-heat bound.
Helium flow is recomputed from that duty and the feasible 920 C -> 500 C
secondary-helium span.

### Code/equation location
`model/src/lib.rs`:
- `R3CcsLedger`, `r3_ccs_ledger`
- `R3HeatCascade`, `r3_heat_cascade`

### Validation evidence
CI asserts:
- strictly positive process and IHX terminal approaches;
- identical MDEA duties in CCS and energy ledgers;
- WHB recovery cannot be allocated beyond available heat;
- recovered MDEA heat is subtracted exactly once;
- explicit first-law residual is numerically zero;
- 600/650/700 C inlet screens retain the feasible temperature hierarchy and
  positive heat/helium flow.

### CI evidence
Commit `166b7be59910f019e206da2344e1536fda1698fd`;
GitHub Actions run 36540973483: PASS.

### Acceptance criterion status
- positive finite approaches: PASS
- one candidate-scaled MDEA duty across CCS/energy: PASS
- recovered heat allocated once: PASS
- first-law residual: PASS
- helium flow recomputed from feasible state: PASS
- reference and inlet-temperature screening hierarchy: PASS

R3-B02 is closed as a bounded heat-integration screen. Detailed exchanger
area/pinch-network design remains outside this scope.

## R3-B03 — Forward lifecycle and economics

**Disposition: RESOLVED AS A FORWARD SCREENING MODEL; conservative case
falsifies robustness.**

### Correction
The R3 lifecycle now consumes the physical R3 carbon/capture/purge state,
R3-B02 nuclear heat, R3 CCS compression/tail-compression loads and the
R3 full-loop helium circulator parasitic. Auxiliary electricity has an explicit
emissions factor rather than disappearing from the lifecycle boundary.

The economic screen is forward: baseline and candidate annual energy/fixed
costs are calculated first and abatement cost is then
`incremental annual cost / annual avoided tCO2e`. S$100/t is not an input.

### Code/equation location
`model/src/lib.rs`:
- `r3_helium_circulator_hi_mwe`
- `R3Lifecycle`, `r3_candidate_lifecycle`
- `R3AnnualCost`, `r3_forward_economic_scenario`
- `R3ThresholdCase`, `r3_reference_threshold_case`,
  `r3_conservative_threshold_case`

### Validation and falsification evidence
The model independently reproduces annual H2 from 8994 kg/h * 8322 h/y and
checks the abatement-cost identity directly. Low-carbon auxiliary electricity
and Singapore-grid auxiliary electricity produce different lifecycle results,
as required.

The deliberately conservative credible corner (18.6 gCO2e/MJ upstream gas,
6.4 g/kWh nuclear LCA, 402 g/kWh auxiliary electricity, 3.5% CCS-chain factor)
produces **non-positive lifecycle abatement**. The initial threshold test failed
because the economic function correctly rejected a non-positive denominator.
The wrapper now records this as a falsification state: annual-abatement and
economic thresholds both fail and abatement cost is +infinity. The case was not
weakened to obtain a pass.

### CI evidence
- `c82c259193c9c4e6101d230cd266e30a4af9828b`: forward lifecycle/economics,
  CI run 36541546244 PASS.
- `564964e67de2a992314c8d534a66bf5ebf485537`: conservative threshold test
  correctly FAILED on non-positive abatement.
- `2d34384c41ef4c277c740e4fb9657a66abc8dbbc`: explicit no-abatement
  falsification handling, CI run 36541687545 PASS.

### Acceptance criterion status
- lifecycle uses actual R3 carbon ledger: PASS
- auxiliary CCS/helium electricity represented: PASS
- annual H2/avoided emissions independently reconstructed: PASS
- baseline/candidate annual costs forward-calculated: PASS for declared
  scenario-cost inputs
- S$/t calculated without target budget: PASS
- reference and conservative cases evaluated: PASS
- robust CN4252 compliance: **FAIL / FALSIFIED by conservative case**

R3-B03 is closed as a modelling blocker because the forward model now answers
the question correctly; its scientific result is that threshold robustness is
not established.

## R3-M01 — Coupled uncertainty and threshold region

**Disposition: RESOLVED AS A FALSIFICATION RESULT.**

### Correction
A 64-point coupled uncertainty design now propagates reformer temperature
(900/950 C), pressure (20/28 bar), PSA recovery (70/90%), capture fraction
(85/95%), paired upstream-gas/auxiliary-electricity carbon conditions, and
paired nuclear-heat/fixed-cost conditions through the R3 fixed-H2 flowsheet and
forward threshold calculations. Grid-carbon auxiliary electricity is paired
with its higher electricity-price scenario rather than combined arbitrarily
with the low-carbon case.

### Scientific result
The first CI run deliberately required evidence of a joint passing region and
FAILED because **zero of the 64 tested physically paired cases passed both
CN4252 thresholds**. The model/domain was not made more favourable. The final
test locks `both_pass == 0` as the current falsification result until model
physics or evidence-backed inputs change.

Driver contrasts explicitly perturb reformer temperature, PSA recovery,
gas/auxiliary-carbon conditions and heat/fixed costs and require a measurable
change in abatement and/or S$/t.

### Code/equation location
`model/src/lib.rs`:
- `r3_solve_case`
- `R3UncertaintyPoint`, `r3_uncertainty_point`
- `r3_uncertainty_design`, `R3UncertaintySummary`,
  `r3_uncertainty_summary`
- `R3DriverContrast`, `r3_driver_contrasts`

### CI evidence
- `e46e8956d1d60f974410c67cce7a6f4e00921d74`: first uncertainty grid;
  CI 36542375779 FAILED because there was no joint passing region.
- `3b12b4017abdb70225ffe7f85d6b46131d8d2013`: preserves no-pass
  falsification and driver diagnostics; subsequent failure exposed only an
  incorrect expected design cardinality.
- `f16b3cf967d587109a9d0977ab4f72684ff5db02`: cardinality corrected to 64;
  CI 36542537230 PASS.

### Acceptance criterion status
- coupled physically compatible uncertainty region: PASS for declared design
- both CN4252 thresholds mapped jointly: PASS
- conservative failure preserved: PASS
- dominant driver contrasts represented: PASS
- robust connected passing region: **ABSENT / FALSIFIED in tested domain**

R3-M01 is closed as an uncertainty-analysis task. Its result does not support a
robust CN4252 pass.

## Next corrective action

R3-M02 — complete common-boundary comparator falsification. Compare the
corrected nuclear case with conventional SMR, conventional SMR+CCS and
electrified reforming on matched H2/output/lifecycle/economic boundaries;
include electrolysis only where source data support a defensible screen.
