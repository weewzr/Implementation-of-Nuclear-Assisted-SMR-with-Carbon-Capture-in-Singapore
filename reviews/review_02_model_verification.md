# Independent Review 02 — Mathematical and Computational Model Verification

## Repository state reviewed

Frozen repository state reviewed: commit `1b28c6859cefbdcffca4c6b2d6ea9f0cf3ce2269`
("Mark Independent Review 1 gate closed"), main branch.

GitHub Actions run 326 for this commit completed successfully. CI success is
treated only as execution evidence, not physics validation.

## Scope

Independent adversarial verification of the mathematical, thermodynamic and
computational model after Review 1 closure. This review does not redo research
framing and does not modify the scientific implementation.

Examined: STATUS.md; Review-1 resolution; CN4252 problem statement; Project
Brief; equation notes; literature matrix; Cargo manifest; model/src/lib.rs;
model/tests; current recycle, CCS, lifecycle, nuclear-heat and economic screens;
and CI evidence.

Independent external checks included NIST SRD 69 Shomate definitions/ranges,
IAPWS-IF97 formulation, and JAEA HTTR/steam-reforming heat-use evidence.

## Model components examined

- IEAGHG source-stream reconstruction and elemental balances
- reformer/WGS reaction thermodynamics
- NIST Shomate property implementation
- IAPWS saturation/dew-point layer
- reduced iterative/analytical tail-recycle fixed point
- PSA recovery treatment
- purge/inert treatment
- recycle compression and sensible heating
- conventional heat-service envelope and 96.04 MW radiant benchmark
- secondary-He/IHX screening and circulator estimates
- Case-2A MDEA/capture electricity and heat anchors
- lifecycle emissions and annual-abatement scaling
- S$100/t economic screens
- Rust input validation, convergence and test structure

## Independent checks performed

1. Traced C/H/O source closure through the Review-1 reformer boundary.
2. Checked SMR/WGS stoichiometry and pressure exponents.
3. Checked Shomate equation units and interval usage against NIST SRD 69.
4. Checked reaction-quotient dimensional standard-state treatment.
5. Audited the reduced recycle fixed-point equations and their analytical
   benchmark.
6. Tested the physical zero-removal/purge/inert limits conceptually.
7. Traced current lifecycle/economic outputs backward to recycle and heat inputs.
8. Checked Case-2A compression/electricity decomposition for double counting.
9. Checked MW/GJ/h, kmol/h-kJ/mol, kgCO2e/kgH2 and annual-abatement conversions.
10. Checked IHX temperature hierarchy and JAEA evidence category.
11. Classified important tests by what they actually verify rather than pass count.

## Work that survived review

The following components are sufficiently sound to preserve:

- IEAGHG source carbon ledger and the corrected stream-4 -> stream-5 water-addition
  interpretation.
- HTS C/H/O conservation and four-way WGS extent reconstruction.
- Ideal reaction stoichiometry and 298 K reaction-enthalpy signs.
- 82.63 MMkcal/h -> 96.04 MW radiant-duty conversion.
- IEAGHG base energy ledger and fixed H2 comparison basis.
- distinction between captured and avoided CO2.
- explicit plant-gate versus lifecycle boundaries.
- Case-2A source electricity identity: 4.575 + 2.874 - 1.140 = 6.309 MWe.
- removal of the Case-2A expander credit when sweet tail gas is recycled rather
  than expanded to furnace burners.
- pressure-ratio monotonicity and basic compressor dimensional form as screening
  calculations.
- SMR Q pressure-squared dependence (Delta n = +2) and WGS ideal-gas pressure
  independence (Delta n = 0).
- IAPWS Region-4 saturation equation implementation concept and dew-point use.
- nuclear temperature hierarchy requiring finite IHX and reformer approaches.
- JAEA values are mostly labelled correctly as precedent/anchor rather than a
  validated Singapore reactor design.
- annual conversion from kgCO2e/kgH2 to t/y is dimensionally correct.

## Blockers

### R2-B01 — Reduced recycle fixed point is not a physical full-species steady state

**Finding**

The current recycle solution converges only H2/CO/CH4. It omits N2/inerts and
has no purge, while fresh NG continuously introduces inert material. It also
retains source-derived PSA recovery and prescribed CO/CH4 conversion rather
than solving the changed recycle composition through the reformer/WGS/PSA.

**Exact repository evidence**

`model/src/lib.rs` states: "A purge is not yet sized, so inert accumulation
remains outside this CHO reduced model." `iterative_tail_recycle_fixed_h2`
contains only H2, CO, CH4 and removed CO2. The analytical solution makes the
fresh-NG fraction independent of every positive conversion/recovery coefficient:

`s = P/(P + H_tail + CO_tail + 4 CH4_tail)`.

The zero-coefficient case explicitly does not converge.

**Scientific/mathematical reason**

With nonzero fresh inert input and no inert outlet, no finite physical steady
state exists. A CHO subspace can converge while the actual plant inventory
diverges. In addition, recycle changes H2/CO/CH4/H2O/inert partial pressures,
steam/carbon ratio, reformer equilibrium, WGS approach and PSA loading. Holding
the once-through PSA recovery and arbitrary conversion coefficients fixed
therefore cannot establish the physical fresh-feed displacement.

The coefficient-independence of fresh feed for every positive conversion is an
algebraic property of the surrogate, not evidence that real conversion does not
matter. The discontinuity between zero removal (no fixed point) and arbitrarily
small positive removal (same fresh-feed fraction, unbounded circulating
inventory) is an important limiting-case warning.

**Affected outputs**

~0.7371 fresh-NG fraction; ~26.3% fresh-feed displacement; circulating tail
composition; captured CO2; recycle sensible heat; recycle compressor work;
candidate lifecycle intensity; ~0.667 MtCO2e/y screen; S$100/t budget; all
downstream economic boundaries.

**Required correction**

Build a full-species recycle/purge balance including at minimum H2, H2O, CO,
CO2, CH4 and N2/inerts. Couple the recycle state to a physically constrained
reformer/WGS layer and an explicit PSA/recovery model or justified composition-
dependent surrogate. Solve purge from inert steady state and account for purge
H2/carbon oxidation/capture.

**Acceptance criterion**

For the recycled configuration:
- every represented element and total mass close over the complete plant;
- inert input equals inert purge/output at steady state;
- purge fraction is finite and positive for nonzero inert feed;
- all species flows/mole fractions are nonnegative and sum correctly;
- solution remains physical as conversion/recovery are varied toward zero;
- once-through topology is recovered through a clearly defined topology limit;
- fresh-feed displacement changes physically with equilibrium/conversion/PSA
  performance rather than being structurally invariant for all positive values;
- at least one independent process benchmark or source-consistent once-through
  case is reproduced before recycle predictions are accepted.

**Recommended falsification test**

Introduce the source N2 feed with purge=0 and demonstrate that the solver
rejects steady state. Sweep purge -> 0, conversion -> 0, PSA recovery, S/C,
pressure and temperature; require bounded inventories only where a physical
outlet exists.

---

### R2-B02 — Integrated thermal service is bounded, not thermodynamically closed, but is consumed as a point duty

**Finding**

The model has a useful service envelope but not a closed integrated energy
balance for the recycle+nuclear configuration. Nevertheless, lifecycle and
economic functions use 162 MWth as a representative point and then propagate it
into nuclear LCA and cost conclusions.

**Exact repository evidence**

`STATUS.md` says 162 MWth "remains usable only as a representative midpoint
inside the current bounded range, not as a solved integrated duty."
`converged_representative_full_cost_with_recycle_penalties` nevertheless sets
`q_total = 162.0 + q_recycle`.
`converged_recycle_shared_direct_lifecycle_screen` accepts the same thermal
service as the nuclear lifecycle basis.

The heat-envelope notes explicitly state that recycle sensible heating, changed
steam generation, pressure effects and altered syngas heat recovery are not
closed.

**Scientific/mathematical reason**

The proposed architecture removes the fired furnace but retains/reconfigures
radiant reforming, feed/pre-reformer heating, steam generation/superheat,
syngas-WHB recovery, WGS recovery, MDEA regeneration and nuclear-loop losses.
Adding individually plausible duties does not prove a first-law-closed heat
cascade. Heat recovered from hot syngas/shift must be allocated once, with
temperature/pinch feasibility, before residual nuclear heat is known.

**Affected outputs**

HTGR MWth requirement; helium flow and parasitics; nuclear lifecycle term;
nuclear heat operating cost; reactor allocation; direct-vs-electric comparison;
S$100/t feasibility region.

**Required correction**

Construct a single temperature-resolved process energy ledger for the candidate
recycle configuration. Recompute reaction/sensible/steam/capture duties from the
same converged material state; explicitly allocate internal recoverable heat by
temperature grade before calculating external nuclear heat.

**Acceptance criterion**

- first-law closure over the candidate process within a declared tolerance;
- every heat source/sink appears exactly once;
- recovered syngas/WGS heat cannot simultaneously reduce a duty and remain
  available elsewhere;
- steam generation/superheat and MDEA heat use a consistent steam/property
  basis;
- external nuclear duty is the residual after feasible heat recovery, not a
  chosen midpoint;
- the once-through IEAGHG case reproduces the 96.04 MW radiant benchmark and
  other available source energy metrics within predeclared tolerances;
- candidate heat duty is recomputed after recycle/purge composition changes.

**Recommended falsification test**

Disable all internal heat recovery and verify external duty rises by exactly the
removed recoverable services; restore each exchanger one at a time and enforce
energy conservation plus positive terminal temperature approaches.

---

### R2-B03 — SMR equilibrium thermodynamics silently extrapolate H2 outside the selected NIST interval

**Finding**

The new SMR equilibrium constant permits 500-1300 K but always evaluates H2 with
`NIST_H2_298_1000`, including the actual reformer regime above 1000 K.

**Exact repository evidence**

`smr_equilibrium_constant_nist` asserts
`temperature_k >= 500 && temperature_k <= 1300` and calls
`NIST_H2_298_1000.standard_gibbs_kj_mol(temperature_k)`.
The repository separately defines `NIST_H2_1000_2500` but does not use it in
this equilibrium function.

**Scientific/mathematical reason**

NIST Shomate coefficients are interval-specific. The intended reformer outlet
range 900-950 C is 1173-1223 K, beyond the 298-1000 K H2 interval. A
thermodynamic constraint cannot be considered verified while its principal
high-temperature equilibrium constant uses silent extrapolation.

The same property-layer design also allows `major_stream_enthalpy_mw` to
1300 K while the selected CO2 set is labelled 298-1200 K, creating a smaller
950 C (1223 K) extrapolation in the WHB reconstruction.

**Affected outputs**

SMR K(T), future equilibrium reformer solution, recycle conversion, reaction
duty, WHB/radiant reconstruction and therefore all later integrated results.

**Required correction**

Implement species property dispatch with explicit validity intervals and
piecewise-continuous H/S/G evaluation across coefficient boundaries. Reject
states outside all supported intervals.

**Acceptance criterion**

- no Shomate set is evaluated outside its documented interval;
- H, S, G and Cp are continuous within expected NIST rounding at interval
  boundaries;
- SMR K(T) at representative 700-1000 C points is independently checked against
  an external thermochemical calculation/reference;
- reformer equilibrium tests include 1173-1223 K;
- invalid-temperature tests fail explicitly rather than extrapolate.

**Recommended falsification test**

Evaluate immediately below/above 1000 K for H2 and 1200 K for CO2, compare
piecewise results and K(T), and assert explicit failure outside supported
ranges.

## Major findings

### R2-M01 — CCS source penalties are mixed with a materially different recycle flowsheet without state-based rescaling

**Finding**

Case-2A source electricity/steam values are correctly decomposed, but the
candidate reuses full-scale Case-2A capture and CO2-compression loads while its
fresh feed, tail flow, capture mass, composition and pressure topology have
changed.

**Exact repository evidence**

`converged_capture_and_recycle_electricity_mwe` retains 4.575 MWe capture
consumption and 2.874 MWe CO2 compression/dehydration as constants, then adds
post-capture recycle compression. MDEA heat is carried as a source-bounded
interval from Case 2A rather than calculated from the candidate captured mass
and solvent state.

**Scientific/mathematical reason**

Avoiding double counting of the first compressor is correct, but a source
anchor is not automatically the candidate duty. Absorber circulation,
regeneration steam, CO2 compression and dehydration depend on CO2 flow,
composition, pressure, capture fraction and product pressure.

**Affected outputs**

Electrical parasitics, MDEA heat, total nuclear service, lifecycle/cost screens.

**Required correction**

Retain Case 2A as validation. Build candidate CCS mass/energy functions from the
candidate stream state and scale/compute capture, regeneration, dehydration and
compression consistently.

**Acceptance criterion**

Candidate CCS energy derives from candidate CO2 flow/composition/pressure and
reproduces IEAGHG Case 2A when fed the Case-2A source state within a declared
tolerance. No compressor or expander appears twice.

**Recommended falsification test**

Set candidate inputs exactly equal to Case 2A and recover 4.575, 2.874 and
appropriate steam duties; halve captured CO2 at otherwise matched conditions
and verify physically sensible duty scaling.

---

### R2-M02 — PSA treatment is source reconstruction, not a validated predictive separation model

**Finding**

The ~90% PSA recovery is reconstructed correctly for the once-through source
case but is reused under substantially altered recycle composition.

**Exact repository evidence**

`iterative_tail_recycle_fixed_h2` sets
`psa_recovery=ieaghg_reconstructed_psa_h2_recovery()` and uses it unchanged
through iterations.

**Scientific/mathematical reason**

PSA recovery/purity/tail composition depend on feed composition, pressure,
cycle and impurity loading. Recycle changes these variables. A full adsorption
model is not mandatory, but a composition-insensitive fixed recovery cannot be
presented as predictive without validation/bounds.

**Affected outputs**

H2 product, tail inventory, fresh NG, recycle fixed point and all derivatives.

**Required correction**

Either implement a validated reduced PSA surrogate with composition/pressure
dependence or bound PSA recovery/purity using literature/source cases and
propagate that uncertainty.

**Acceptance criterion**

Once-through IEAGHG purity/recovery are reproduced; recycle predictions retain
>=99.9% product purity on the common basis; recovery sensitivity is propagated
and does not create negative/impossible species balances.

**Recommended falsification test**

Sweep impurity loading and recovery; reject any state whose product/tail
species cannot satisfy component balances and the common H2 specification.

---

### R2-M03 — Test suite is strong on identities/regressions but still weak on independent integrated falsification

**Finding**

The repository has many useful tests, but most current integrated-result tests
lock ranges or compare the numerical recycle solver to an analytical solution
of the same surrogate equations.

**Exact repository evidence**

Examples include numerical-vs-analytical recycle fixed point, broad numeric
windows for heat/economics, and monotonic pressure/cost tests. The analytical
recycle benchmark is explicitly described as "not an independent physical
model."

**Scientific/mathematical reason**

These tests are valuable for software correctness but cannot validate omitted
physics. A wrong surrogate can pass all of them exactly.

**Affected outputs**

Confidence assigned to recycle, heat, lifecycle and economics.

**Required correction**

Add independent external benchmark/integration tests after B01-B03 corrections.

**Acceptance criterion**

The test portfolio includes:
A identity/unit; B regression; C limiting-case; D conservation; E independent
analytical; F external benchmark; G system integration, with at least one F and
one G test for each model chain used in final conclusions.

**Recommended falsification test**

Blindly reproduce an external once-through SMR/CCS case using only its inputs,
then compare withheld output variables.

---

### R2-M04 — Helium loop remains a screening transport model, not a closed nuclear heat-delivery design

**Finding**

Temperature ordering is correct and JAEA precedent is used cautiously, but the
loop uses constant Cp screening, ideal density, IHX pressure-drop multiples and
does not close all component pressure losses/return temperature/loop losses.

**Exact repository evidence**

`helium_mass_flow_kg_s` takes caller-supplied constant Cp.
`loop_dp_from_ihx_multiple_kpa` scales the 58 kPa GTHTR300C IHX loss by an
assumed multiplier. Repository notes explicitly say total loop pumping must be
larger than the IHX-only layer.

**Scientific/mathematical reason**

Nominal reactor outlet temperature above process temperature is necessary but
not sufficient. Delivered heat depends on both terminal approaches, return
temperature, component losses and circulation power.

**Affected outputs**

helium mass flow, circulator MWe, available process heat, cost and feasibility.

**Required correction**

Keep JAEA as precedent/scaling anchor, not validation. Close a bounded secondary
loop using temperature-dependent He properties and component/total pressure-loss
bounds tied to evidence.

**Acceptance criterion**

For every reported nuclear case: positive IHX and reformer terminal approaches,
closed Q=m*integral(cp dT), bounded total loop pressure loss, explicit
circulator efficiency/power, compatible return temperature and no use of the
10 MW HTTR or 170 MW GTHTR300C values as direct scale validation.

**Recommended falsification test**

Sweep minimum approaches and total pressure drop until feasibility disappears;
the model must reject infeasible temperature crossings and excessive parasitics.

---

### R2-M05 — Lifecycle/abatement/economic outputs are correctly dimensioned but inherit unvalidated physical closures

**Finding**

The annualisation arithmetic is sound, and upstream gas factors are mostly
labelled as proxies. However, current ~1.9 kgCO2e/kgH2, ~0.667 Mt/y and S$100/t
economic boundaries consume the reduced recycle fresh-feed fraction, imposed
90% capture and representative thermal duty.

**Exact repository evidence**

`converged_reference_screen` uses 0.90 capture, 11.5 g/MJ upstream gas,
162 MWth, 5.5 g/kWh-e nuclear proxy, 0.504 efficiency and 2.5% CCS transport.
STATUS explicitly calls the recycle/heat model the largest remaining scientific
weakness.

**Scientific/mathematical reason**

Correct downstream arithmetic cannot upgrade uncertain upstream physics.
These values are conditional screening outputs, not verified integrated results.

**Affected outputs**

candidate lifecycle intensity, annual assignment compliance, allowable annual
cost, minimum gas-price and CCS-cost feasibility surfaces.

**Required correction**

After B01-B03/M01-M02, regenerate lifecycle/economics from the verified material,
capture, energy and parasitic states. Preserve literature proxies as sensitivity
ranges.

**Acceptance criterion**

Every final lifecycle/economic point traces to one internally consistent
physical case; capture mass equals modelled available carbon; energy parasitics
are included once; Singapore/global proxy labels remain explicit; annual
abatement recomputes exactly from specific abatement and annual H2.

**Recommended falsification test**

Recalculate the same case independently from exported mass/energy ledgers,
without calling the lifecycle/economic convenience functions, and require
agreement within numerical tolerance.

## Minor findings

### R2-m01 — STATUS contains stale contradictory Review-1 text

Top line says Review 1 CLOSED while an older embedded section still says OPEN.
This does not change model equations but can mislead future gate logic. Clean it
when Main Research next updates STATUS; do not spend a research pass on it.

### R2-m02 — Property objects do not carry validity metadata

Validity is enforced inconsistently by caller assertions rather than by the
`Shomate` object itself. B03 requires the scientific correction; attaching
range metadata would also reduce future misuse.

### R2-m03 — Some function names remain stronger than their evidence class

Names containing `converged` can be read as flowsheet convergence although
they mean convergence of a reduced CHO surrogate. Rename or document at call
sites after B01; not a separate scientific blocker.

## Claim-strength corrections

Current important quantities should be classified as follows:

| Quantity | Correct category at reviewed commit |
|---|---|
| IEAGHG stream values / 96.04 MW radiant duty | SOURCE VALUE |
| stream-4/5 inferred ~154.4 kmol/h water addition | RECONSTRUCTED VALUE |
| HTS extent / PSA recovery from source rows | RECONSTRUCTED VALUE |
| 96.04 MW unit conversion | VERIFIED CALCULATION FROM SOURCE |
| WGS Q/K diagnostic | SCREENING/DIAGNOSTIC RESULT |
| SMR K(T) above 1000 K | UNVALIDATED / PROPERTY-DEFECT AFFECTED |
| ~0.7371 fresh-NG fraction | REDUCED-SURROGATE SCREENING RESULT |
| ~26.3% fresh-NG displacement | REDUCED-SURROGATE SCREENING RESULT |
| recycle compressor work | SENSITIVITY RESULT |
| ~145-179 MWth / representative 162 MWth | BOUNDED ESTIMATE / REPRESENTATIVE MIDPOINT |
| ~80 kg/s secondary helium | SCREENING RESULT |
| ~1.9 kgCO2e/kgH2 candidate | CONDITIONAL SCREENING RESULT |
| ~0.667 MtCO2e/y | CONDITIONAL SCREENING RESULT |
| S$100/t feasibility surfaces | CONDITIONAL ECONOMIC SCREEN |
| JAEA 10 MW / 950 C / 880 C values | SOURCE PRECEDENT, NOT PROJECT VALIDATION |

No recycle, nuclear-duty, lifecycle, annual-abatement or cost result at this
commit qualifies as a VERIFIED INTEGRATED MODEL RESULT.

## Missing falsification tests

Highest-value missing tests:

1. inert feed + zero purge must fail to reach physical steady state;
2. finite purge must close inert, C/H/O and total mass simultaneously;
3. conversion -> 0 must produce physically interpretable limiting behaviour;
4. high recycle must not create negative flows or mole fractions >1;
5. piecewise Shomate boundary tests at 1000 K H2 and 1200 K CO2;
6. external SMR K(T) values at reformer temperatures;
7. candidate CCS model recovering IEAGHG Case 2A from source inputs;
8. full candidate first-law closure with heat recovery enabled/disabled;
9. PSA impurity/recovery sensitivity preserving H2 purity and component balance;
10. independent recomputation of lifecycle and annual abatement from exported
    physical ledgers.

## Test-quality classification

- **A identity/unit:** strong (unit conversions, source arithmetic).
- **B regression:** very strong, arguably overrepresented.
- **C limiting-case:** partial; important recycle/purge limits missing.
- **D conservation:** strong for source baseline/HTS; incomplete for full recycle.
- **E independent analytical:** present for algebraic recycle, but it verifies
  the same surrogate equations rather than physical validity.
- **F external benchmark:** good for selected IEAGHG source quantities; weak for
  predictive recycle/CCS/nuclear integration.
- **G integration/system:** insufficient for a final integrated conclusion.

## Source / parameter traceability sample

- **IEAGHG:** base NG/H2/CO2, PSA streams, 96.04 MW radiant duty and Case-2A
  electricity are traceable and generally used in context.
- **NIST/JANAF:** coefficients are traceable, but interval enforcement is
  defective for high-temperature SMR H2 and slightly for high-T CO2 usage.
- **IAPWS:** Region-4 saturation implementation is traceable; several steam
  enthalpy/latent values remain encoded as source/property anchors rather than a
  complete IF97 property engine.
- **JAEA:** 950 C HTTR outlet, IHX/secondary-loop precedent and 10 MW steam-
  reforming design are appropriately precedents; larger GTHTR values are only
  plausibility/scaling anchors.
- **CCS:** Case-2A source loads are traceable but not yet transformed into a
  state-based candidate CCS model.
- **Lifecycle:** 11.5/18.6 gCO2e/MJ and nuclear g/kWh values are explicitly
  sensitivity/proxy anchors; candidate result strength is limited by physical
  model closure rather than dimensional arithmetic.

## Dimensional-analysis result

No root conversion-factor error was found in the sampled important equations:
- kmol/h * kJ/mol / 3600 -> MW is handled consistently because kmol*kJ/mol = MJ;
- GJ/h / 3.6 -> MW is correct;
- MMkcal/h * 4.184 / 3.6 -> MW is correct;
- kgCO2e/kgH2 times annual kgH2 / 1000 -> tCO2e/y is correct;
- the CN4252 0.25 Mt/y scale identity is dimensionally correct.

The dominant risks are therefore physical closure and model-class overreach,
not a hidden factor-of-1000 error.

## Review 2 decision

The repository contains a substantially stronger baseline and verification
culture than at Review 1. However, the model currently used to propagate recycle
benefits into lifecycle and economics is not a full physical steady state, the
new high-temperature SMR equilibrium layer has a correlation-range defect, and
the integrated nuclear thermal duty remains a bounded service estimate rather
than a closed candidate energy balance.

Integrated conclusions, optimisation and final-paper quantitative claims should
not rely on these outputs yet.

## Main Research Handoff

1. **R2-B03 first:** repair interval-safe thermodynamic property dispatch and
   independently validate SMR K(T) at reformer temperatures.
   **Acceptance:** no correlation extrapolation; boundary continuity; external
   K(T) checks pass.

2. **R2-B01:** replace the CHO recycle surrogate with a full-species
   recycle/purge steady state coupled to reformer/WGS constraints and
   composition-aware/bounded PSA behavior.
   **Acceptance:** total/element/inert closure, finite purge, physical limiting
   cases, nonnegative species and source-consistent once-through validation.

3. **R2-B02:** close the candidate temperature-resolved first-law heat cascade
   from the corrected material state.
   **Acceptance:** every heat source/sink counted once, internal recovery
   allocated by temperature grade, and external nuclear duty emerges as the
   residual with benchmark closure.

4. **R2-M01 + R2-M02:** derive candidate CCS/PSA penalties from candidate stream
   states while retaining IEAGHG cases as validation anchors.
   **Acceptance:** source cases are reproduced from source inputs and candidate
   duties scale physically with candidate mass/composition/pressure.

5. **R2-M04:** close the bounded secondary-He loop after candidate duty is known.
   **Acceptance:** finite approaches, temperature-dependent He heat balance,
   bounded full-loop pressure loss/circulator power and compatible return state.

6. **R2-M03 + R2-M05:** add independent external/system falsification tests and
   regenerate lifecycle/economic screens only from the corrected physical case.
   **Acceptance:** at least one external predictive benchmark and one full
   integration check pass; lifecycle/annual/economic results independently
   reproduce from exported mass/energy ledgers.

