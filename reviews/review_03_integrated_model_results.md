# Independent Review 03 — Verified Computational Model, Integrated Results and Falsification

## Repository state reviewed

- Repository: `weewzr/Implementation-of-Nuclear-Assisted-SMR-with-Carbon-Capture-in-Singapore`
- Frozen commit reviewed: `763113d4e3d217bb31995a73caec57460fb66f8d`
- Commit message: `Close Review2 after integrated propagation`
- Commit date: 2026-09-29
- STATUS at frozen state: Gate 3 mathematical/model foundation COMPLETE;
  Independent Reviews 1 and 2 CLOSED; Gate 4 next.
- Current CI at the frozen SHA: GitHub Actions run `36538193806`,
  `cargo test --all-targets`, **PASS**.
- No subsequent Main Research commit existed at review freeze.

This review is read-only with respect to the scientific implementation. No
model correction, optimisation, Gate-5 work, or repository reorganisation was
performed.

## Scope

Review 3 asks a different question from Reviews 1 and 2: whether the integrated
computational model and its results are scientifically verified strongly enough
to support experiments/results and eventual CN4252 conclusions.

Review-2 work is preserved where it remains valid. CI success is treated as
software evidence, not scientific validation.

The CN4252 thresholds are:
- annual abatement >0.25 MtCO2e/y;
- abatement cost <S$100/tCO2e.

## Canonical model identified

The repository identifies `canonical_integrated_screen()` in
`model/src/lib.rs` as the current integrated reference calculation. Its
declared dependency chain is:

1. `thermo_recycle_reference_case()`;
2. `candidate_energy_ledger(650 C)`;
3. `candidate_ccs_duty_ledger()`;
4. `candidate_helium_loop_ledger(650 C)`;
5. `thermo_recycle_lifecycle_screen(...)`;
6. annual abatement/S$100 budget;
7. representative economic break-even gas-value calculation.

The canonical reference uses:
- reformer equilibrium T = 1173.15 K (900 C);
- reformer P = 28 bar;
- shift T = 685.15 K (412 C);
- shift P = 27.7 bar;
- PSA impurity sensitivity = 0.35 in the canonical solver;
- max recycle N2 dry mole fraction = 0.10;
- candidate lifecycle capture fraction = 0.90;
- upstream NG factor = 11.5 gCO2e/MJ;
- nuclear electricity LCA anchor = 5.5 gCO2e/kWh_e converted to a thermal proxy;
- nuclear process heat = high side of `candidate_energy_ledger(650 C)`;
- 8322 operating h/y;
- H2 production basis = 8994 kg/h / 100,000 Nm3/h source scale.

### Canonical versus superseded results

**Canonical calculation:** `canonical_integrated_screen()` and its upstream
B02/M01/M04 functions.

**Superseded/provenance only:** analytical ~0.737 fresh-feed result; 80% reduced
recycle; 162 MWth representative heat case; old matched-recycle and Singapore
joint lifecycle/economic result files. These old Markdown result files still
contain numerical claims but explicitly describe the retired physical model.

A major reproducibility limitation is that the current canonical function's
actual numerical outputs are not persisted in a generated result artifact or
CLI output. CI verifies identities/inequalities but does not publish the
canonical fresh fraction, purge, heat duty, CCS flow, lifecycle intensity,
annual abatement or economic result.

## Independent checks performed

1. Traced the actual Rust call graph from `canonical_integrated_screen()` down
   through recycle, equilibrium, PSA, purge, energy, CCS, helium, lifecycle and
   economic functions.
2. Reconstructed source carbon independently:
   - IEAGHG feed NG carbon = ~1578.815 kmol-C/h;
   - IEAGHG HTS-inlet carbon = ~1578.639 kmol-C/h;
   - difference is only ~0.011%, consistent with rounded source rows.
   This shows the HTS-inlet state preserves source carbon approximately, but
   does **not** make it a physical fresh NG+steam inlet.
3. Reconstructed annual H2 output:
   `8994 kg/h * 8322 h/y = 74.848 kt/y`.
4. Therefore the 0.25 Mt/y threshold requires specific lifecycle abatement
   greater than ~3.340 kgCO2e/kgH2 at this plant scale.
5. Reconstructed IEAGHG Case-1A direct-plant avoidance at the same source scale:
   `(0.8091-0.3704) kg/Nm3 * 100,000 Nm3/h * 8322 h/y`
   = ~0.365 MtCO2/y. Thus the annual scale threshold is not uniquely enabled by
   the nuclear architecture.
6. Checked current official Singapore nuclear/CCS context and JAEA heat-use
   precedent separately from repository claims.
7. Inspected test structure and failure handling; no scientific implementation
   was modified.

## Review-2 regression check

The following Review-2 corrections remain present:
- interval-safe H2/CO2 Shomate dispatch;
- WetGas6 full-species state;
- explicit N2 purge relation;
- outer fixed-H2 product solve;
- coupled ideal-gas SMR/WGS equilibrium;
- candidate energy/CCS/helium ledger functions;
- PSA 70-90% recovery uncertainty functions;
- integrated lifecycle/economic propagation function.

The retired analytical ~0.737 and 162 MWth values are not directly called by
`canonical_integrated_screen()`.

However, Review 3 finds that the modules are not yet one physically consistent
plant. This is a new integrated-model finding, not a repetition of Review 2.

## Work that survived review

The following components remain scientifically useful and should not be
rewritten merely because Gate 4 remains open:

- IEAGHG source-stream reconstruction and source carbon consistency.
- Interval-safe NIST Shomate thermochemistry.
- SMR and WGS equilibrium equations and their reaction-level elemental
  conservation tests.
- Explicit numerical failure for unbracketed equilibrium roots.
- Nested fixed-H2 recycle numerical architecture as a solver pattern.
- Explicit N2 purge concept and inert-balance requirement.
- IEAGHG radiant-duty and WHB source-limited validation.
- Separation of source precedent from project-specific nuclear feasibility in
  the documented limitations.
- Common H2 production basis and 8322 h/y scaling.
- Source decomposition of Case-2A electricity and the warning against applying
  low-pressure tail compression to high-pressure process gas.
- General CRF and abatement-cost equations.
- The explicit retirement of the old 0.737/162-MW predictive path.

These pieces are foundations; the failure is in how current canonical modules
are connected and interpreted.

# Blockers

## R3-B01 — Canonical process chain does not represent fresh NG + steam → reformer → capture → PSA → recycle as one physical state

### Finding
The canonical recycle solver uses the already-reformed IEAGHG HTS-inlet gas as
its “fresh” stream, scales it by a fresh fraction, adds recycle, and reforms it
again. It therefore does not model fresh natural gas + steam/pre-reforming as
declared.

Separately, the CCS ledger counts shifted-gas CO2 as captured, but the canonical
recycle solver never removes that CO2 before PSA/recycle. The PSA tail therefore
contains/recycles CO2 that another module simultaneously treats as captured.

### Exact repository evidence
`model/src/lib.rs`:
- `ieaghg_hts_inlet_wet6()` is explicitly the authoritative HTS inlet.
- `solve_recycle6_at_fixed_fresh_thermo()`:
  `source=ieaghg_hts_inlet_wet6(); feed=source.scale(fresh_fraction).add(recycle)`;
  that feed is then passed to `solve_smr_wgs_equilibrium`.
- `thermo_reference_reformer_states()` repeats the same construction.
- `candidate_ccs_duty_ledger()` sets `process_co2=shifted.co2` and treats it
  as capture throughput, but `solve_recycle6_at_fixed_fresh_thermo()` passes
  `shifted` directly to `psa6_bounded` without a CO2-removal operation.
- `psa6_bounded` sends all non-product species, including CO2, to tail gas.

### Independent scientific check
IEAGHG source carbon in feed NG (~1578.815 kmol-C/h) and HTS inlet
(~1578.639 kmol-C/h) is nearly equal, so scaling HTS inlet happens to preserve
carbon throughput. But HTS inlet already contains ~51.7% H2, ~11.6% CO,
~4.9% CO2 and only ~2.4% CH4; it is a reaction product state, not NG+steam.
Carbon equality cannot justify re-reforming that composition as fresh feed.

### Affected results
Fresh-NG reduction, steam demand, reaction extents, recycle composition, purge,
PSA loading/recovery, CCS throughput, reformer heat, upstream NG lifecycle
burden, annual abatement and economics.

### Why it matters
A solver can conserve elements and converge while solving the wrong flowsheet.
The current “fresh fraction” is a scale factor on a transformed syngas state,
then interpreted downstream as a fraction of natural-gas feed energy/carbon.

The capture/recycle inconsistency also allows the same CO2 to be both counted as
captured and remain in the state sent to PSA/recycle.

### Required correction
Create one canonical stream graph beginning from actual fresh NG composition,
steam/water additions and recycle. Model or source-anchor pre-reforming, primary
reforming, WGS, CO2 removal, PSA, purge treatment and recycle in their physical
order. A capture operation must remove captured CO2 from the state that proceeds
downstream.

If a reduced equivalent-feed representation is retained, derive and prove the
mapping from NG+steam elements/enthalpy to that state before reaction, rather
than using an already-reacted HTS row.

### Acceptance criterion
For the canonical case:
- one stream state is passed consistently between every unit;
- overall and unit C/H/O/N residuals are reported numerically and normalized to
  inlet elemental throughput, each <=1e-6 (or a separately justified tolerance
  for rounded source validation only);
- total mass residual is reported on the plant boundary;
- H2 product, purge and captured CO2 are explicit outlets;
- CO2 removed by capture cannot appear in downstream PSA/recycle;
- fresh-NG energy/carbon used by lifecycle/economics is derived from the actual
  fresh NG inlet, not a syngas scale factor.

### Recommended falsification test
At zero recycle, the canonical model should reproduce the IEAGHG once-through
material state within declared source tolerance from NG+steam inputs. Then turn
recycle on and verify that total external C/H/O/N still closes and that every
captured-carbon molecule is absent from downstream recycle.

## R3-B02 — Canonical heat-integration state is thermodynamically inconsistent and the CCS heat duty is not the same duty used by the CCS ledger

### Finding
The canonical reformer equilibrium outlet is 900 C, while
`candidate_helium_loop_ledger()` assumes secondary helium cools from 900 C to
500 C. A 900 C helium hot stream cannot transfer finite heat to a process whose
hot end is also 900 C with a positive terminal approach.

The repository separately has an approach-temperature function and a test that
950 C process + 20 K approach requires >970 C secondary helium, but that
constraint is not applied to the canonical candidate loop.

In addition, `candidate_energy_ledger()` uses the old unscaled
`mdea_incremental_heat_source_bounded_mw()`, whereas
`candidate_ccs_duty_ledger()` computes candidate-scaled MDEA heat. The
canonical integrated heat duty therefore does not consume the candidate-specific
M01 capture heat.

### Exact repository evidence
`model/src/lib.rs`:
- `thermo_recycle_reference_case()`: reformer T = 1173.15 K = 900 C.
- `candidate_energy_ledger()`: outlet enthalpy evaluated at 1173.15 K and
  `reformer_outlet_c=900.0`.
- `candidate_helium_loop_ledger()`: mass flow uses `t_hot=900.0`,
  `t_cold=500.0`.
- `required_secondary_he_hot_c()` and
  `ihx_hot_end_temperature_budget_k()` exist but are not enforced by the
  canonical loop.
- `candidate_energy_ledger()` calls
  `mdea_incremental_heat_source_bounded_mw()`.
- `candidate_ccs_duty_ledger()` independently calculates
  `mdea_heat_lo_mw`/`mdea_heat_hi_mw` by candidate CO2 scaling.
- `canonical_integrated_screen()` uses nuclear heat from
  `candidate_energy_ledger()`, not the M01-scaled MDEA heat.

### Independent scientific check
A heat exchanger needs a positive local temperature difference. At the
canonical hot end, 900 C helium and a 900 C reformer target imply zero driving
force and unbounded required area in the ideal limit.

JAEA precedent explicitly distinguishes reactor/primary heat near 950 C from
secondary helium near 900 C delivered to a hydrogen system. JAEA's HTTR steam
reforming mock-up heated helium to about 880 C at the steam-reformer inlet;
historical helium-heated reformer work cited by JAEA used reforming temperature
around 820 C with helium around 950 C. Source precedent therefore does not
validate a 900 C process hot end heated by 900 C helium.

### Affected results
Helium flow, IHX/process-heater feasibility, nuclear duty, circulator power,
candidate lifecycle intensity and all cost calculations.

### Why it matters
MW balance alone is insufficient for process heat. The canonical temperature
hierarchy is infeasible at its hot end. Separately, using one MDEA heat duty for
energy and a different scaled duty for CCS means the integrated modules describe
different plants.

### Required correction
Enforce finite temperature approaches throughout the canonical secondary-He
cascade. Solve a consistent reactor-primary/IHX-secondary/process temperature
hierarchy, including return temperature and steam-generation duties.

Use the candidate-specific M01 MDEA regeneration duty in the candidate energy
cascade and allocate feasible recovered heat once only.

### Acceptance criterion
- minimum terminal approach >0 at every heat exchanger and process hot end;
- canonical loop temperatures satisfy the approach constraints under reference
  and worst credible conditions;
- the MDEA heat in the energy ledger equals the M01 candidate-scaled MDEA duty
  before heat recovery;
- every recovered-heat stream has a single sink/allocation;
- first-law residual is reported for the complete candidate heat cascade;
- canonical helium flow and circulator power are recomputed from that feasible
  temperature/duty state.

### Recommended falsification test
Run the worst credible reformer-temperature case with minimum allowed approach.
The solver must reject any state where secondary-He hot temperature is <=
process hot-end temperature + approach. Perturb MDEA throughput and verify that
the nuclear duty changes consistently in both CCS and energy ledgers.

## R3-B03 — Lifecycle and economic outputs do not establish either CN4252 threshold

### Finding
The canonical lifecycle calculation omits the lifecycle emissions associated
with CCS compression and helium-circulator electricity even though those
electrical loads are calculated and charged economically.

Plant residual carbon is calculated as
`external_feed_carbon*(1-capture_fraction)` with an imposed 90% capture
fraction rather than from the candidate CCS carbon ledger.

The canonical economic output `min_gas_value_sgd_per_gj` is not an abatement
cost. It is a break-even natural-gas value obtained after first assuming the
S$100/t annual abatement budget. The canonical function does not compute
candidate annual cost, baseline annual cost and then
`(C_candidate-C_baseline)/(E_baseline-E_candidate)`.

### Exact repository evidence
`model/src/lib.rs`:
- `LifecycleCase` contains only plant_carbon, upstream_ng, nuclear and
  ccs_transport.
- `thermo_recycle_lifecycle_screen()` has no CCS/circulator electricity
  emissions term and does not consume `CandidateCcsDutyLedger`.
- `canonical_integrated_screen()` calculates `ccs_e` and helium circulator
  electricity for **cost**, but passes only nuclear thermal service to the
  lifecycle function.
- `canonical_integrated_screen()` hard-codes capture fraction 0.90.
- The generic correct `abatement_cost_per_tco2e()` function exists, but
  `canonical_integrated_screen()` does not call it.
- Instead it computes a S$100/t annual budget, assumes fixed annual costs
  (S$31.9m + S$50m + S$8.2m + S$5m), and solves for
  `min_gas_value_sgd_per_gj`.

### Independent scientific check
At 8994 kgH2/h and 8322 h/y, annual H2 is ~74.848 kt/y. Meeting
0.25 MtCO2e/y therefore requires >3.340 kgCO2e/kgH2 specific abatement.

The repository does not currently provide a complete canonical candidate
lifecycle intensity from all energy/parasitic sources, so this threshold cannot
be independently verified for the canonical physical plant.

Likewise, a break-even gas value conditional on a S$100/t budget is not an
independent demonstration that abatement cost is <S$100/t.

If incremental electricity were Singapore-grid supplied, the omission could be
material: EMA reports a 2024 average grid emission factor of 0.402 kgCO2/kWh.
If supplied by nuclear, the nuclear allocation must explicitly include it.
Either choice must be stated and propagated.

### Affected results
Candidate kgCO2e/kgH2, annual avoided CO2e, S$100/t compliance, economic
feasibility region and all assignment-level conclusions.

### Why it matters
These are the two CN4252 quantitative decision metrics. A model that assumes the
target budget and solves for another variable has not independently passed the
target.

### Required correction
Build lifecycle emissions directly from the closed plant carbon ledger and all
incremental energy streams, with an explicit electricity-supply assumption.

Build baseline and candidate annual costs on a common price year/currency,
annualisation, capacity factor and boundary. Then call the actual abatement-cost
equation.

Classify every cost as source, assumed, derived or break-even.

### Acceptance criterion
- lifecycle functional unit remains kgCO2e/kgH2 at common H2 purity/pressure;
- direct residual/purge/captured carbon reconciles to the physical CCS ledger;
- CCS compression, recycle compression and helium circulation emissions are
  included or explicitly allocated to a zero/low-carbon source with evidence;
- annual H2 and avoided emissions are independently reproduced;
- baseline and candidate annual costs are explicit;
- canonical S$/tCO2e is calculated from incremental cost / avoided emissions,
  not reverse-solved from the S$100/t target;
- reference and conservative credible cases are both evaluated;
- threshold crossing is reported as a region, not a single favourable point.

### Recommended falsification test
Recompute candidate CI with (a) low-carbon/nuclear auxiliary electricity and
(b) Singapore-grid electricity. Independently reconstruct annual cost and
abatement cost without using the S$100/t budget anywhere in the forward
calculation. The target should be checked only after the result is produced.

# Major findings

## R3-M01 — Uncertainty is not propagated through the integrated threshold model

### Finding
The repository has separate uncertainty pieces (PSA 70-90%, inlet temperature,
helium pressure-loss bounds, earlier upstream-gas ranges), but
`canonical_integrated_screen()` evaluates one point: 900 C/28 bar reformer,
650 C inlet, PSA impurity sensitivity 0.35, 10% max N2, 90% capture, 11.5 g/MJ
upstream NG, high-side heat/helium loads and fixed cost assumptions.

The PSA recovery envelope is not the canonical PSA implementation; the
canonical solver still uses `psa6_bounded(...,0.35)`.

### Affected results
Robustness of fresh-NG reduction, annual abatement, heat duty and economics.

### Why it matters
The assignment requires a defensible passing region, not proof that one
favourable or arbitrary point can be made to pass.

### Required correction
After R3-B01/B02/B03, propagate a physically coupled uncertainty design over at
least reformer T/P, steam/carbon, PSA recovery, inert/purge constraint, capture
fraction, upstream gas, heat-recovery/MDEA duty, helium loop, NG price, reactor
cost and CCS T&S.

Correlated quantities must be sampled jointly where physics requires it.

### Acceptance criterion
Report threshold maps for annual abatement and S$/t, identify dominant
parameters, and show reference plus conservative credible corners. A “robust”
claim requires a nontrivial connected passing region under physically compatible
parameter combinations.

### Recommended falsification test
Search explicitly for the smallest perturbation from the reference case that
causes either CN4252 threshold to fail.

## R3-M02 — Comparator fairness and central-hypothesis falsification are incomplete

### Finding
The repository contains useful IEAGHG conventional cases and old Singapore
screens, but no current integrated common-boundary comparison using the corrected
canonical candidate against all planned comparators. Electrified SMR,
nuclear-electric SMR and electrolysis are not developed enough in the current
canonical result chain to test H2-H7 fairly.

### Independent check / falsification status
- H1 conventional high-capture SMR+CCS: **not falsified**. IEAGHG Case 1A
  shifted-syngas MDEA avoids ~54% of source plant emissions. At the source
  100,000 Nm3/h scale, direct annual avoidance is ~0.365 MtCO2/y, already above
  0.25 Mt/y. Its Singapore lifecycle/economic comparison is not current enough
  to decide the <S$100/t criterion.
- H2 electrified reforming: **not tested on current integrated boundary**.
- H3 imported-gas lifecycle emissions: **not tested through current canonical
  uncertainty propagation**.
- H4 PSA/recycle/purge penalties: bounded pieces exist, but **not propagated to
  assignment thresholds**.
- H5 CCS T&S cost: canonical calculation uses a fixed assumed annual amount;
  **threshold fragility not currently established**.
- H6 reactor/IHX/integration cost: canonical uses fixed annual allowances;
  **not independently annualised/validated**.
- H7 low-carbon electrolysis: **no current fair integrated comparator**.
- H8 Singapore deployment constraints: **not quantitatively modelled**.

### Why it matters
The project hypothesis is comparative. A technically functioning nuclear case
does not establish that it is the appropriate CN4252 solution.

### Required correction
Construct a compact common-basis comparator table/model after the canonical
nuclear case is repaired. Use the same H2 output/purity/pressure, operating
hours, lifecycle boundary, cost year/currency and CCS T&S convention.

### Acceptance criterion
At minimum, current conventional SMR, conventional SMR+CCS, electrified SMR+CCS
and direct nuclear-heat SMR+CCS must be comparable on the two CN4252 metrics.
Electrolysis should be included where sufficient data support a defensible
screen; otherwise mark it data-limited rather than infer superiority.

### Recommended falsification test
Search comparator parameter ranges for any non-nuclear configuration satisfying
both thresholds with lower incremental cost or lower integration complexity.

## R3-M03 — Singapore deployment-scale feasibility is not yet demonstrated

### Finding
The canonical model is process-scale but does not translate its result into a
Singapore deployment package: reactor module count/thermal capacity, CO2
captured and shipped per year, storage route, helium-loop equipment scale,
footprint or deployment readiness.

### Independent external check
Singapore currently states that it has **not made a decision to deploy nuclear
energy** and is building capability to assess advanced nuclear options. An IAEA
INIR Phase-1 mission is planned from 2027. Singapore also lacks domestic
geological storage and is developing cross-border CCS arrangements; official
2025 statements say clearer capture/transport/storage cost estimates are still
being studied.

### Why it matters
Technical process feasibility and Singapore deployment feasibility are distinct.

### Required correction
Translate the corrected canonical plant into annual NG, H2, captured/residual
CO2, required reactor thermal capacity, plausible module count/range, helium
flow and cross-border CO2 throughput. Compare these with source-backed
Singapore infrastructure/readiness constraints without assuming deployment.

### Acceptance criterion
A deployment-scale table separates process requirements from infrastructure
readiness and labels nuclear adoption and cross-border storage as scenario
conditions, not established facts.

### Recommended falsification test
Assume no domestic nuclear deployment before the relevant project horizon and
test whether the concept still qualifies as an implementable CN4252 pathway or
only as a longer-term scenario.

## R3-M04 — Gate-4 test evidence is insufficiently adversarial and canonical numerical results are not persisted

### Finding
The suite contains many useful A-D tests and some E/F checks, but relatively few
true integrated/adversarial tests. Important tests can pass while the flowsheet
is physically wrong; for example, reaction elemental conservation passes even
when an HTS-inlet product state is incorrectly reused as fresh reformer feed.

The canonical integrated function has no persisted generated result artifact or
CLI output. Current Markdown results largely describe retired 80%/162-MW
models.

### Test classification
- A identity/unit: strong (unit conversions, cost identities, property helpers).
- B regression: extensive.
- C limiting-case: moderate (temperature/pressure monotonicity, PSA envelope).
- D conservation: good at reaction/unit level.
- E independent analytical: moderate (fixed-point/equilibrium checks).
- F external benchmark: useful but limited (SMR K(T), IEAGHG radiant/WHB).
- G integrated/system: present mainly as dependency equality and positivity,
  not independent plant-wide closure.
- H adversarial/failure-mode: insufficient for Gate 4.

### Required correction
Add independent plant-wide residual tests, topology tests that verify removed
species are absent downstream, finite-approach heat tests, lifecycle
reconstruction tests and solver adversarial cases. Persist canonical numerical
results from the frozen model in a reproducible generated artifact.

### Acceptance criterion
Gate-4 CI must include F/G/H tests capable of failing for each R3 blocker.
A fresh checkout must reproduce a canonical results table without editing code.

### Recommended falsification test
Deliberately introduce a duplicate captured-CO2 stream, zero hot-end approach,
or wrong fresh-feed state in a test fixture and verify the integrated tests
fail.

# Minor findings

## R3-m01 — Stale result documents can be mistaken for current evidence

`results/singapore_joint_lifecycle_economic_v2.md`,
`results/matched_recycle_lifecycle_economics_v1.md` and several economics
files still present numerical claims based on 80% recycle and 162 MWth. Their
text often correctly states limitations, but they are easy to mistake for
current results.

**Correction:** add a prominent superseded/provenance banner or move them under
a clearly archival index after current results are reproducibly generated.

## R3-m02 — README research status is stale

`README.md` still says Gate 1 is in progress while STATUS says Gate 3 is
complete and Gate 4 is next.

**Correction:** update status pointer only; no scientific rewrite is needed.

# Integrated mass/energy verification

## Mass and element closure

Reaction operators conserve C/H/O/N and source rows reconstruct carbon well.
Those checks survive.

The integrated external plant boundary does **not** yet have a valid numerical
mass/element closure because the external “fresh” stream in the canonical
solver is an HTS-inlet syngas state rather than NG+steam, and captured CO2 is
not removed from the state sent through PSA/recycle.

The `CandidateCcsDutyLedger.carbon_closure_error_kmol_h` calculation checks
carbon across fresh+recycle to shifted gas before separation; it does not close
the external plant boundary through H2 product, purge, capture and residual
emissions. No canonical test asserts that field.

Therefore unit-level conservation must not be described as plant-wide closure.

## Energy closure

`Q_reformer = H_out-H_in` is internally exact for the state supplied to it,
but that state is affected by R3-B01. The full heat cascade is additionally
invalidated by:
- zero hot-end approach in the canonical 900 C He / 900 C process pairing;
- candidate-scaled MDEA duty not being the MDEA duty used by the energy ledger;
- recovered-heat allocation remaining source-bounded rather than fully
  candidate-state resolved.

The energy ledger is therefore a screening calculation, not a verified
integrated plant energy balance.

# Lifecycle verification

Functional unit is intended as kgCO2e/kgH2 and H2 scale is consistent with the
IEAGHG source plant.

Independent scale identity:
- annual H2 = ~74.848 kt/y;
- >0.25 Mt/y requires >3.340 kgCO2e/kgH2 avoided.

The canonical lifecycle function includes:
- residual feed-carbon term via imposed capture fraction;
- upstream NG;
- nuclear heat LCA proxy;
- CCS transport fractional proxy.

It omits:
- emissions associated with CCS compression electricity;
- helium-circulator electricity;
- any other auxiliary electricity represented in the physical/economic model;
- direct use of actual captured/residual carbon from the CCS ledger.

Consequently the canonical annual-abatement output is a **scenario/screening
result**, not a verified lifecycle result.

# Economic verification

The repository contains the correct generic equation:

`C_abatement = (C_candidate-C_baseline)/(E_baseline-E_candidate)`.

But the canonical integrated function does not evaluate that equation.

Instead it:
1. computes avoided emissions;
2. multiplies them by S$100/t to obtain the maximum allowed annual incremental
   cost;
3. inserts assumed annual fixed costs plus heat/electricity costs;
4. solves backward for the natural-gas value needed to fit that budget.

Therefore `min_gas_value_sgd_per_gj` is a **BREAK-EVEN VALUE**, not a
predicted project cost and not a canonical S$/tCO2e result.

The fixed annual amounts (S$31.9m T&S, S$50m reactor allocation, S$8.2m
IHX/loop, S$5m integration) are scenario assumptions in the canonical function;
they are not generated from a common CAPEX/CRF/currency-year model there.

A canonical S$/t point cannot be independently reconstructed from the current
function because baseline and candidate annual costs are not defined on the
same forward cost ledger.

# CN4252 threshold assessment

## >0.25 MtCO2e/y

**Current status: NOT VERIFIED for the canonical candidate.**

The scale is sufficient in principle: at 74.848 ktH2/y only
3.340 kgCO2e/kgH2 specific abatement is required. Old retired screens reported
much larger values, but those cannot be used as current evidence.

The canonical lifecycle denominator is incomplete and rests on the inconsistent
flowsheet identified in R3-B01, so neither reference nor conservative robust
compliance is established.

## <S$100/tCO2e

**Current status: NOT VERIFIED.**

The canonical calculation reverse-solves a gas-value boundary after assuming
the S$100/t budget. It does not produce a forward abatement cost.

## Robustness category

The current repository does not yet establish A/B/C/D cleanly for the corrected
canonical plant. It contains historical favourable screening points, but Review
3 does not accept them as current threshold evidence.

# Comparator fairness

Common source H2 scale exists, but comparator boundaries are not yet harmonised
with the corrected canonical candidate.

IEAGHG Case 1A is a strong falsification comparator: source data report ~54%
CO2 avoidance and, at the project operating scale, ~0.365 MtCO2/y direct
avoidance. This exceeds the annual scale threshold without nuclear integration.
That does not prove Case 1A meets <S$100/t in Singapore, because lifecycle,
price-year, cross-border T&S and current candidate boundaries differ.

Electrified SMR, nuclear-electric SMR and electrolysis are not currently
developed to the same integrated boundary. No technology ranking is supported.

# Falsification results

- **H1 conventional high-capture SMR+CCS comparable/cheaper:** remains viable
  as a falsification hypothesis. Annual abatement scale can exceed 0.25 Mt/y on
  IEAGHG Case-1A direct boundary; current Singapore cost comparison is
  insufficient.
- **H2 electrified reforming avoids nuclear heat-integration complexity:** not
  falsified; current integrated comparator absent.
- **H3 imported-gas lifecycle burden weakens concept:** not falsified; current
  canonical screen fixes 11.5 g/MJ and does not propagate LNG-like extremes.
- **H4 PSA/recycle/purge penalties erase NG advantage:** not falsified; PSA
  envelope is not propagated through assignment thresholds.
- **H5 CCS T&S makes <S$100/t impossible:** not falsified; current T&S is an
  assumed annual amount and official Singapore cost remains under study.
- **H6 reactor/IHX/integration cost makes direct nuclear heat inferior:** not
  falsified; current annual allowances are assumed rather than forward
  annualised on a common basis.
- **H7 low-carbon electrolysis becomes preferable:** not tested sufficiently.
- **H8 Singapore deployment constraints dominate:** remains a serious open
  falsification path; nuclear deployment is not decided and cross-border CCS is
  still being developed.

The central nuclear hypothesis therefore **survives only as an unverified
research hypothesis**, not as a supported CN4252 conclusion.

# Singapore-scale assessment

Source-scale H2 production is ~74.85 kt/y.

The repository does not yet persist a trustworthy canonical table for:
- actual fresh NG t/y or energy/y after corrected flowsheet closure;
- captured CO2 t/y;
- residual CO2 t/y;
- cross-border CO2 shipping/storage t/y;
- corrected nuclear MWth after finite-temperature integration;
- reactor module count/range;
- corrected helium flow;
- land/footprint.

External context must be kept separate:
- Singapore has not decided to deploy nuclear energy and is building assessment
  capability; INIR Phase 1 is planned from 2027.
- Singapore has no suitable domestic geological CO2 storage and is pursuing
  cross-border CCS cooperation; service/cost feasibility is still under study.

Thus **technical process feasibility** and **Singapore deployment feasibility**
are both still conditional.

# Claim-strength corrections

| Quantity/claim | Correct Review-3 classification |
|---|---|
| IEAGHG stream values / 96 MW radiant benchmark | SOURCE VALUE |
| Stream-4/5 water reconciliation | RECONSTRUCTED VALUE |
| Shomate/equilibrium unit results | VERIFIED MODEL RESULT at equation/unit scope |
| Fresh-NG reduction from canonical recycle | UNVALIDATED PREDICTION pending R3-B01 |
| Purge rate/composition | SCREENING RESULT pending integrated closure |
| H2 product fixed-output result | VERIFIED NUMERICAL CONSTRAINT, not independent yield validation |
| Candidate reformer heat | SCREENING RESULT pending correct fresh state |
| Helium flow/pressure-loss parasitic | BOUNDED ESTIMATE; current temperature hierarchy infeasible |
| CCS MDEA/compression duty | SCREENING/BOUNDED ESTIMATE; topology inconsistent with solver |
| Candidate lifecycle intensity | SCENARIO RESULT, incomplete |
| Annual abatement | SCENARIO RESULT, not threshold-verified |
| `min_gas_value_sgd_per_gj` | BREAK-EVEN VALUE |
| <S$100/t compliance | UNVALIDATED PREDICTION / NOT ESTABLISHED |
| Nuclear deployment in Singapore | SCENARIO CONDITION, not decided policy/infrastructure |

Repository prose should not call the latter screening/scenario quantities
“verified integrated results” until the blockers are resolved.

# Missing falsification tests

Highest-value missing tests:
1. zero-recycle once-through reproduction from actual NG+steam inputs;
2. plant-wide normalized C/H/O/N and mass residuals including capture/product/
   purge/water;
3. topology assertion that captured CO2 is absent from downstream PSA/recycle;
4. finite hot-end approach assertion in the canonical helium/process loop;
5. single-allocation recovered-heat test;
6. candidate-scaled MDEA heat equality between CCS and energy ledgers;
7. lifecycle reconstruction including all auxiliary electricity;
8. forward S$/t abatement-cost reconstruction without using target budget;
9. PSA 70/90% envelope propagated to both CN4252 thresholds;
10. reformer T/P/S:C adversarial grid with explicit solver failures;
11. increased inert-feed/purge stress test;
12. common-boundary comparator threshold tests.

# Gate-4 decision

Gate 4 is **not complete**.

Passing CI, equilibrium convergence and Review-2 closure do not compensate for
three integrated-model blockers:
1. the canonical flowsheet does not start from/propagate one physical NG+steam
   state and its CCS removal is not applied to the recycle state;
2. the canonical heat-integration temperature hierarchy is infeasible at the
   hot end and energy/CCS use different MDEA duties;
3. lifecycle/economic calculations do not independently evaluate the two CN4252
   thresholds.

Experiments/results generated from the current canonical function could be
numerically reproducible yet describe inconsistent physical plants.

# Main Research Handoff

1. **R3-B01 — Rebuild the canonical stream graph on one physical state.**
   Acceptance: actual fresh NG+steam/pre-reformer/reformer/WGS/capture/PSA/
   purge/recycle ordering; capture removes species from downstream state;
   normalized plant-wide C/H/O/N and total-mass residuals within declared
   tolerance; zero-recycle case reproduces IEAGHG source state.

2. **R3-B02 — Close a temperature-feasible integrated heat cascade.**
   Acceptance: finite positive approaches everywhere; candidate-scaled MDEA
   heat is identical in CCS and energy ledgers; recovered heat allocated once;
   complete first-law residual; helium flow/parasitic recomputed from feasible
   temperatures under reference and worst credible conditions.

3. **R3-B03 — Rebuild lifecycle and economics as forward calculations.**
   Acceptance: lifecycle consumes actual carbon ledger and all auxiliary energy;
   annual H2/CO2e independently reproduced; baseline/candidate annual costs use
   common currency-year/annualisation/boundary; S$/t computed forward from
   incremental cost/avoided emissions; reference and conservative cases tested
   without using S$100/t as an input.

4. **R3-M01 — Propagate coupled uncertainty to both assignment thresholds.**
   Acceptance: physically compatible uncertainty region covering reformer
   T/P/S:C, PSA, purge/inerts, capture, upstream NG, heat recovery, helium,
   NG/reactor/CCS costs; dominant drivers and connected pass/fail regions
   reported.

5. **R3-M02 — Complete common-boundary comparator falsification.**
   Acceptance: conventional SMR, conventional SMR+CCS, electrified SMR+CCS and
   direct nuclear-heat SMR+CCS compared at common H2 product, hours, lifecycle
   and cost basis; electrolysis included where data are adequate; no technology
   ranking inferred from unmatched boundaries.

6. **R3-M03 — Quantify Singapore deployment scale.**
   Acceptance: annual H2, NG, captured/residual CO2, cross-border storage
   throughput, corrected nuclear MWth/module range and helium-loop scale
   reported; nuclear adoption and storage access explicitly treated as scenario
   conditions.

7. **R3-M04 — Add Gate-4 F/G/H verification and reproducible result artifact.**
   Acceptance: independent external, integrated-system and adversarial tests can
   detect each blocker class; fresh checkout deterministically generates the
   canonical numerical results table.

# External sources used for independent context

- IEAGHG 2017-02 SMR+CCS technical report:
  https://publications.ieaghg.org/technicalreports/2017-02%20Techno%20-%20Economic%20Evaluation%20of%20SMR%20Based%20Standalone%20%28Merchant%29%20Hydrogen%20Plant%20with%20CCS.pdf
- JAEA HTTR hydrogen/heat-use programme:
  https://www.jaea.go.jp/04/o-arai/nhc/jp/use_htgr/production.html
- JAEA Technology 2018-004, HTTR steam-reforming system:
  https://jopss.jaea.go.jp/pdfdata/JAEA-Technology-2018-004.pdf
- Singapore MTI nuclear-energy status:
  https://www.mti.gov.sg/energy-and-carbon/energy-supply/low-carbon-alternatives/nuclear-energy/
- Singapore EMA energy statistics/grid emission factor:
  https://www.ema.gov.sg/resources/singapore-energy-statistics/chapter2
- Singapore MTI CCS status:
  https://www.mti.gov.sg/energy-and-carbon/carbon/
- Singapore MTI April-2025 CCS cost reply:
  https://www.mti.gov.sg/newsroom/written-reply-to-pq-on-projected-cost-per-tonne-of-carbon-dioxide-abated-through-carbon-capture-and-storage-in-singapore/

# Final decision

GATE 4 NOT YET VERIFIED


---

## Closure addendum — 2026-09-29

The original decision above applies to frozen commit
`763113d4e3d217bb31995a73caec57460fb66f8d`.

Main Research subsequently resolved R3-B01/B02/B03 and R3-M01/M02/M03/M04.
A focused closure check at commit
`c384e053e8ea1ec42f1d471fe7f7733dda022b75` verified the original acceptance
criteria against the corrected implementation and current CI (run
`36545720485`, PASS).

Canonical disposition/evidence is recorded in
`reviews/review_03_resolution.md`.

**Updated Gate-4 decision: VERIFIED FOR EXPERIMENTS/RESULTS.**

This closure does not reverse the adverse scientific results: the conservative
case fails both CN4252 thresholds and the corrected 64-point coupled uncertainty
design has zero joint passes.


---

# Current-state Review-3 re-verification addendum — 2026-09-30

## Repository state reviewed

- Current frozen commit: `fbbd3eccc137749c71b563069f6666271ac8feac`.
- Current STATUS: Gate 4 CLOSED; Gate 5 CLOSED; Gate 6 complete first manuscript / ready for Independent Review 4.
- Research CI run `36638378449`: PASS.
- Paper/reproducibility run `36638378430`: PASS.
- Original Review-3 freeze and findings above remain historical evidence; this addendum evaluates whether the corrected current implementation still satisfies the Review-3 gate.

## Scope

This is a current-state regression/re-verification of Independent Review 3, not a repetition of Reviews 1-2 and not an implementation pass. No scientific implementation was modified.

## Canonical model identified

The current canonical physical path is the corrected R3 chain:
external NG/steam-derived feed + recycle -> coupled reformer/WGS -> explicit CO2 removal -> PSA -> inert purge -> recycle/purge treatment -> candidate-specific CCS -> corrected heat cascade -> secondary-He parasitics -> lifecycle ledger -> annual abatement -> forward economics.

Canonical results are defined by `r3_canonical_results()`, `results/r3_canonical_results.md`, and the Gate-5 generated result contract. Superseded 0.737 fresh-feed and representative 162 MWth assumptions remain provenance only.

## Independent checks performed

1. Re-checked the corrected B01/B02/B03 call chain and its current tests.
2. Verified that captured process CO2 is removed before PSA/recycle and that the external plant boundary closes C/H/O/N and total mass to the declared (10^{-6}) criterion.
3. Verified that the heat cascade enforces a 900 C process hot end, 920 C secondary-He hot end and 950 C primary outlet for the reference 20 K process / 30 K IHX approaches.
4. Verified that the CCS and heat ledgers consume the same candidate-scaled MDEA duty and that recovered heat is credited once.
5. Reconstructed the common production scale independently: 8994 kg H2/h * 8322 h/y = 74.848068 kt H2/y. The 0.25 MtCO2e/y threshold therefore requires about 3.34 tCO2e avoided per t H2 at this scale.
6. Reconstructed IEAGHG Case 1A direct avoidance from the source values: (0.8091-0.3704) kgCO2/Nm3 H2 * 100000 Nm3/h * 8322 h/y is about 0.365 MtCO2/y, independently confirming that the annual scale threshold is not uniquely enabled by nuclear heat.
7. Re-checked the coupled uncertainty contract: 64 cases, zero joint passes, conservative case non-positive lifecycle abatement with infinite rather than fabricated finite abatement cost.
8. Checked later changes from Review-3 closure to the frozen state. They add Gate-5 experiments, deterministic renderers and paper/reproducibility infrastructure; no evidence was found that the corrected R3 physical chain was replaced by the retired model.

## Review-2 regression check

PASS. Interval-safe thermochemistry, full-species recycle, explicit inert purge, fixed-H2 closure, coupled SMR/WGS equilibrium, candidate-specific CCS, bounded PSA uncertainty, candidate-driven helium sizing and corrected lifecycle/economic propagation remain present. No retired 0.737 fresh-feed, 162 MWth, purge-free recycle or universal-PSA assumption was found on the canonical R3 predictive path.

## Work that survived review

- Source-stream reconstruction and NIST thermochemistry.
- Corrected external-feed/recycle/capture process ordering.
- Explicit plant C/H/O/N and mass closure.
- Fixed-H2 PSA/recycle/purge architecture.
- Candidate-specific CCS scaling.
- Positive-temperature-approach heat cascade and candidate-driven helium flow.
- Forward lifecycle and economic propagation.
- Explicit non-positive-abatement failure handling.
- 64-case coupled threshold falsification with 0 joint passes.
- Common-scale Case-1A direct-abatement comparator.
- Deterministic external/integrated/adversarial tests and result generation.

## Blockers

None found in the current corrected R3 implementation for its declared screening-model scope.

## Major findings

None found that invalidate Gate-4 use for generating bounded experiments/results.

## Minor findings

### R3-m05 — Real-gas sensitivity remains outside the equilibrium screen
The reformer/WGS solver remains ideal-gas based at 20-28 bar. Pressure effects are represented thermodynamically, but fugacity corrections are not. This does not invalidate the stated screening result, because the repository does not claim a kinetic or rigorous high-pressure reactor prediction, but a future higher-fidelity model should quantify real-gas sensitivity before upgrading process predictions.

### R3-m06 — Uncertainty design is falsification-oriented, not exhaustive
The 64-case design couples the dominant declared temperature, pressure, PSA, capture, carbon/electricity and heat/fixed-cost corners, but it does not independently span every possible variable listed in the Review-3 brief (for example S/C, purge/inert target, compressor efficiency, helium pressure drop and methane leakage). The repository correctly limits its conclusion to the tested domain; therefore this is not evidence for a hidden passing region, nor proof that none exists outside the domain.

### R3-m07 — Comparator economics remain intentionally incomplete on a matched Singapore basis
Case 1A has a defensible common-scale direct-abatement comparison, but eSMR/electrolysis and Case-1A Singapore total CAC lack a fully harmonised currency-year/lifecycle/economic boundary. The repository explicitly states this limitation and does not declare an economic winner.

## Integrated mass/energy verification

The current canonical external boundary explicitly treats recycle as internal and fresh external feed against H2 product, captured CO2 and purge as outlets. CI requires normalized C/H/O/N and total-mass residuals below (10^{-6}). The corrected capture operation removes CO2 from the downstream state before PSA.

The R3 heat cascade uses the same corrected physical state as CCS. The reference temperature hierarchy has strictly positive approaches (950 C primary > 920 C secondary He > 900 C process), candidate-scaled MDEA duty is shared between CCS and energy ledgers, recovered heat is bounded by available WHB heat and credited once, and the first-law residual is tested.

## Lifecycle verification

The lifecycle ledger propagates residual direct carbon, upstream NG, nuclear-heat allocation, auxiliary electricity including CCS/helium parasitics and CCS transport. The conservative credible case produces non-positive lifecycle abatement and is represented as a threshold failure with infinite abatement cost rather than forced through the denominator.

The functional unit remains kgCO2e/kgH2, with annualisation on 8994 kg/h and 8322 h/y. The independently reconstructed annual H2 scale is 74.848068 kt/y.

## Economic verification

The economic screen is forward: annual baseline/candidate costs are constructed before division by annual avoided emissions. S$100/t is a threshold test, not a fitted input. Non-positive abatement is rejected as having no finite abatement cost.

Economic quantities remain scenario results rather than predicted project costs. The repository does not mix IEAGHG EUR2014 non-T&S cost with Singapore SGD T&S contributions into a false common-basis total.

## CN4252 threshold assessment

Within the verified declared domain:
- conservative credible nuclear case: fails annual abatement and cost thresholds;
- coupled nuclear design: 64 cases, 0 joint passes;
- no robust passing region is established;
- IEAGHG Case 1A exceeds 0.25 Mt/y direct avoided CO2 on the common source production basis;
- the evidence does not establish a preferred Singapore technology on a fully matched economic basis.

## Comparator fairness

The conventional baseline and Case 1A share the source H2/output-hours basis. The repository explicitly refuses to fabricate fully matched eSMR/electrolysis economics where evidence is insufficient. Remaining comparator incompleteness is disclosed rather than used to advantage the nuclear case.

## Falsification results

The central nuclear hypothesis does not survive as a demonstrated robust CN4252 solution in the tested domain. Upstream/auxiliary carbon and heat/fixed-cost conditions can eliminate lifecycle abatement or make cost non-finite. Conventional Case 1A independently clears the annual direct-abatement scale. Singapore nuclear and cross-border CCS availability remain conditional deployment assumptions.

## Singapore-scale assessment

The corrected R3 state derives annual H2, fresh NG, captured/storage CO2, residual direct CO2, nuclear process heat and helium-loop scale. These are screening quantities. The repository does not infer reactor-module count, footprint, contracted storage or a Singapore nuclear deployment decision from them.

## Claim-strength corrections

No new claim-strength upgrade is required for Gate 4. The current repository consistently distinguishes source values, reconstructed/source-backed values, verified model results, screening results, sensitivity results, bounded estimates and unresolved comparator predictions.

## Missing falsification tests

Future higher-fidelity extensions should test:
1. fugacity/real-gas sensitivity at the 20-28 bar reformer pressure range;
2. coupled S/C and purge/inert variation rather than only fixed screening settings;
3. explicit compressor/helium pressure-drop efficiency uncertainty;
4. explicit methane-leakage parameterisation if upstream NG factors are decomposed;
5. a fully matched Singapore currency-year/lifecycle comparator if defensible data become available.

These are not prerequisites for preserving the present adverse tested-domain result.

## Gate-4 decision

Gate 4 remains verified for the declared screening-model scope. This decision means the computational framework is sufficiently internally consistent, falsifiable and reproducible to generate bounded experiments/results. It does not mean nuclear-assisted SMR+CCS satisfies either CN4252 threshold, is commercially deployable, or is preferred.

## Main Research Handoff

No BLOCKER or MAJOR corrective action is required by this current-state Review-3 re-verification. Preserve the adverse 0/64 result and the stated screening limitations; do not expand claims beyond the tested domain.

GATE 4 VERIFIED — READY FOR EXPERIMENTS AND RESULTS

MAIN RESEARCH HANDOFF

None.
