# Final Design Independent Verification 02

## Independence and frozen state

This verification was performed independently from scratch against frozen
scientific/design commit:

`12625aed52d52670b4f87de7511e8a3dd1f103d5`.

Subsequent commits were inspected only to confirm scope:
- `d553a3214927bdbb2ef499ed9872d76370b29aa7` changes only STATUS.md;
- `51c94986e9cb7e6f64949a3000319d91d217beb9` adds only the previous
  verification document.

They do not supersede the frozen scientific design.

The pre-existing
`reviews/final_design_independent_verification.md` was deliberately not read
until the primary-source, code and headline-calculation checks below were
completed. It is discussed only in the final comparison section.

No implementation, manuscript, result or assumption was modified.

## Sources independently returned to

Primary/authoritative evidence checked independently:

1. INL TEV-953 Rev. 0, *HTGR-Integrated Hydrogen Production via Steam Methane
   Reforming (SMR) Process Analysis*.
2. INL TEV-961 Rev. 0, *Sensitivity of Hydrogen Production via Steam Methane
   Reforming to High Temperature Gas-Cooled Reactor Outlet Temperature Process
   Analysis*.
3. Nishihara, Mouri and Kunitomi, ICONE15 (2007), *Potential of the HTGR
   Hydrogen Cogeneration System in Japan*.
4. JAEA/JAERI GTHTR300C design literature for the 170 MWth / 202 MWe variant.
5. EMA Singapore electricity-market context.

Repository evidence independently inspected:
- `model/src/final_design.rs`;
- current deployment economic helpers;
- `results/final_design/ASSUMPTIONS.md`;
- `FINAL_DESIGN_RESULTS.md`;
- `SANITY_CHECKS.md`;
- `SOURCE_REGISTER.md`;
- final-design manuscript sections.

## 1. INL process-state verification

PASS for the core source process.

TEV-953 independently confirms:
- reformer outlet 871 C;
- S/C = 3.0;
- methane conversion 78.1% at the specified reformer conditions;
- PSA H2 recovery 88%;
- conventional no-CC common-output basis 130 MMSCFD H2;
- conventional no-CC NG feed 52.5 MMSCFD;
- conventional no-CC direct CO2 = 3,205 ton/day.

TEV-961 independently confirms for its 925 C ROT / 900 C process-heat Case 6:
- 130 MMSCFD H2;
- 34.0 MMSCFD NG;
- 1,927 ton/day CO2 captured;
- 142 ton/day CO2 emitted;
- 17.3 MWe process electricity;
- 176.8 MWth process heat.

TEV-961 explicitly states that Case 6 supplies 900 C heat from a 925 C reactor
outlet and supplies the entire reforming duty at the selected 871 C reformer
target.

The final design therefore has a genuine primary-source process basis rather
than an internally invented process state.

## 2. Important INL integration details omitted/misrepresented by the final screen

TEV-961 also reports for Case 6:
- helium supply temperature = 900 C;
- helium return temperature = 466 C;
- helium flow = 78.49 kg/s;
- a 25 C primary-to-secondary temperature approach;
- at 925 C ROT only ~2 MWt remains available for power generation in the INL
  process-integration interpretation;
- the Case-6 flowsheet requires nitrogen management and uses a small cryogenic
  separation unit in the source model.

The frozen final design instead calculates helium flow using a project
assumption of 585 C return and constant Cp, giving ~107.94 kg/s.

That is not a source reproduction. It is an integration screen. It should not
be described as the INL Case-6 helium-flow result.

More importantly, TEV-961's statement that only ~2 MWt is available for power
generation in its Case-6 integration is not reconciled with the final economic
model's assumption of 202 MWe gross cogeneration from the separate JAEA
architecture. Combining the INL chemical-process heat balance with a different
JAEA cogeneration power split requires an explicit integrated reactor/heat/power
balance, not just common reactor thermal capacity.

## 3. Independent annualization

At 85% availability:
- operating hours = 7446 h/y.

H2:
29,000 lb/h * 0.45359237 kg/lb * 7446 h/y /1000
= **97,946.015 t/y**.

Captured CO2:
1,927 short t/d *0.90718474 *365 *0.85
= **542,361.984 t/y**.

Candidate emitted CO2:
142 short t/d *0.90718474 *365 *0.85
= **39,966.477 t/y**.

Baseline emitted CO2:
3,205 short t/d *0.90718474 *365 *0.85
= **902,060.280 t/y**.

Direct avoided:
= **862,093.803 t/y**.

These headline annualization calculations are correct.

## 4. Independent lifecycle reconstruction

Using the frozen implementation's declared lifecycle assumptions:

- upstream baseline NG = 206,321.689 tCO2e/y;
- upstream candidate NG = 133,617.855 tCO2e/y;
- upstream reduction = 72,703.833 tCO2e/y;
- nuclear heat proxy burden = 3,649.207 tCO2e/y;
- incremental process-electricity burden = 450.483 tCO2e/y;
- CCS transport proxy = 13,559.050 tCO2e/y.

Therefore:

862,093.803 + 72,703.833 - 3,649.207 - 450.483 - 13,559.050
= **917,138.896 tCO2e/y**.

The reported ~0.917 MtCO2e/y lifecycle-abatement headline is arithmetically
reproduced under the declared proxy boundary.

The margin above the CN4252 0.25 Mt/y threshold is ~667,139 t/y, so this
abatement conclusion is not numerically marginal.

## 5. Lifecycle-intensity implementation defect

### MAJOR FDV2-M01 — `lifecycle_ci_kgkg` has a factor-of-1000 unit error

`final_design.rs` computes:

`(base_emitted + upstream_base - lifecycle_avoided) * 1000 / annual_h2_t`.

All numerator terms and `annual_h2_t` are already in tonnes/year. A
tCO2e/tH2 ratio is numerically equal to kgCO2e/kgH2; multiplying by 1000 is
incorrect.

The current function therefore returns roughly **1,953 kgCO2e/kgH2** rather
than approximately **1.95 kgCO2e/kgH2**.

This field is not used to calculate the current 917 kt/y abatement or the
headline abatement cost and was not found propagated into the final manuscript.
It therefore does not reverse the threshold result, but it is a genuine
scientific-code defect.

**Required correction:** remove the erroneous factor 1000 or otherwise make the
units explicit.

**Acceptance criterion:** candidate lifecycle intensity independently
reconstructs from annual candidate lifecycle emissions and annual H2 to the same
kg/kg value, with a dimensional unit test.

## 6. JAEA hardware/economic source audit

### BLOCKER FDV2-B01 — two materially different GTHTR300C variants are conflated

This is the principal independent finding.

The frozen final design states:
- reference IHX = 170 MWth;
- gross electricity = 202 MWe;
- plant economics = 59.7 bn JPY / 0.52 JPY/MJ / 4.9 JPY/kWh;
- doubled-IHX/secondary-loop sensitivity = 70.9 bn JPY / 0.57 JPY/MJ /
  5.5 JPY/kWh.

Those values do not come from one internally consistent JAEA source
configuration.

The JAEA/GTHTR300C design variant with:
- 170 MWth hydrogen heat;
- 430 MWth power branch;
- ~202 MWe electricity
is a real JAEA design variant.

However, Nishihara et al. 2007, which is the source of the 59.7/70.9 bn JPY and
0.52/0.57 JPY/MJ economics, describes a different GTHTR300C configuration:
- 600 MWth reactor;
- **370 MWth IHX / hydrogen heat branch**;
- 900 C secondary helium;
- **88 MWe electricity**.

The paper then states that the 59.7 bn JPY nuclear-plant estimate includes
11.2 bn JPY for the IHX and secondary helium loop. If the **cost** of the IHX
and secondary loop doubles, total plant cost rises to 70.9 bn JPY and heat /
electricity costs rise to 0.57 JPY/MJ and 5.5 JPY/kWh.

Therefore:

1. the source economic reference IHX is 370 MWth, not 170 MWth;
2. 176.8 MWth does **not** exceed that economic source's IHX duty;
3. the 70.9 bn JPY sensitivity is a doubled-*cost* sensitivity, not evidence
   for a doubled-capacity exchanger;
4. the source economic variant produces 88 MWe, not 202 MWe;
5. `final_design.rs` uses 202 MWe while pricing it with the 370-MW/88-MWe
   variant's electricity-cost number.

This invalidates the frozen implementation's exact reactor burden,
"doubled-IHX" rationale, electricity export and headline negative-cost result as
a source-consistent calculation.

**Required correction:** select one coherent JAEA/GTHTR300C variant or build an
explicit, source-supported interpolation/integration model. Hardware duty,
secondary-helium state, gross electric output, plant CAPEX and unit heat/
electricity costs must refer to the same architecture or have a transparent
engineering transformation between architectures.

**Acceptance criterion:** every JAEA economic/hardware quantity in the final
forward calculation traces to one coherent configuration (or an explicitly
derived transformation), and the full reactor heat/electric balance closes
before electricity revenue is credited.

## 7. Independent check of whether FDV2-B01 necessarily destroys the CN4252 pass

It does not, by itself.

As a reviewer sensitivity only, using the Nishihara economic variant
consistently at 88 MWe gross rather than 202 MWe, while retaining the project's
other frozen assumptions, gives approximately:
- gross electricity burden ~S$33.0m/y;
- net export after 17.3 MWe process demand ~70.7 MWe;
- S$150/MWh export value ~S$79.0m/y;
- total incremental annual cost ~-S$94.0m/y;
- abatement cost ~**-S$102/tCO2e**.

With zero export value in that same simplified reconciliation, the calculation
still remains below S$100/t because the source NG reduction is large.

This sensitivity is NOT a replacement canonical result and does not resolve
FDV2-B01. It shows only that correcting the source variant mismatch is not
obviously fatal to the CN4252 pass. The exact final economics must be
recalculated on a coherent architecture.

## 8. Reactor/process-heat compatibility

The INL Case-6 process itself is source-supported at 925 C ROT -> 900 C supplied
heat -> 871 C reformer outlet. This is a 25 C primary-secondary approach and
29 K process-side hot-end difference in the source study.

The frozen manuscript's additional 950 C JAEA "primary envelope" is not needed
to establish the INL Case-6 temperature ladder; it is a hardware precedent.

One 600 MWth reactor has ample aggregate thermal capacity for 176.8 MWth.
But aggregate capacity is not the full integration proof.

The final design must reconcile:
- INL Case-6 helium supply/return/flow and ~2 MWt power-availability statement;
- the selected JAEA secondary loop;
- the selected JAEA electricity-producing variant.

### MAJOR FDV2-M02 — helium-flow and cogeneration power are not derived from one integrated thermal state

The final model uses:
- INL process heat = 176.8 MWth;
- project/JAEA-anchored 585 C return;
- constant Cp -> ~107.94 kg/s;
- JAEA gross electricity = 202 MWe.

INL Case 6 reports 900 C supply, 466 C return and 78.49 kg/s and states that at
925 C ROT only ~2 MWt is available for power generation in its integration.

These can be different design choices, but the frozen final design has not
shown the thermodynamic transformation that makes them one plant.

**Required correction:** establish one explicit primary/secondary helium and
power-cycle energy balance for the final configuration.

**Acceptance criterion:** reactor thermal input = process-heat branch +
power-cycle branch + declared losses/returns on a consistent temperature/flow
basis, with gross electricity generated from the residual branch rather than
inserted independently.

## 9. Independent economics reconstruction of the frozen implementation

The frozen arithmetic itself reproduces.

Using its normalized doubled-cost sensitivity:
- heat price ~S$5.216/GJ;
- electricity cost ~S$50.328/MWh;
- plant cost ~S$648.77m;
- nuclear heat burden ~S$24.72m/y;
- 202-MWe electricity burden ~S$75.70m/y;
- S$150/MWh net-export value (184.7 MWe) ~S$206.29m/y;
- baseline-to-candidate NG saving ~S$94.83m/y;
- scaled CCS CAPEX ~S$114.04m;
- annual CCS capital ~S$10.68m/y;
- integration annualization ~S$3.30m/y;
- T&S ~S$8.14m/y.

These reproduce:
- annual incremental cost ~**-S$178.59m/y**;
- abatement cost ~**-S$194.72/tCO2e**.

The calculation is internally arithmetically consistent but not source-
architecturally valid because of FDV2-B01/M02.

## 10. Electricity-value context

S$150/MWh is a plausible Singapore screening value, not a guaranteed offtake.
EMA reports 2025 USEP largely in the S$100-200/MWh range, and actual monthly
prices are volatile.

The manuscript appropriately labels S$100/150/200 per MWh as a sensitivity
rather than guaranteed revenue.

Because the frozen model's zero-value case is below S$100/t under its current
mixed architecture, the headline cost pass is not solely created by the
S$150/MWh credit. Nevertheless, the zero-value result must be recalculated
after B01/M02 are corrected.

## 11. Source/process claim strength

PASS with corrections required.

The INL process design is genuinely source-backed.

The lifecycle result is a project extension using global proxies and screening
assumptions, correctly described as conditional.

The economics are design-study/scenario economics, not observed Singapore
commercial costs.

The current manuscript's broad phrase "CONDITIONAL MODEL RESULT" is the correct
claim class, but the exact final cost result cannot be called independently
verified until the JAEA architecture/economic mixing is repaired.

## 12. Comparison with the previous verification (performed only after independent work)

The previous
`reviews/final_design_independent_verification.md` independently reproduced
the same annualization/lifecycle/economic arithmetic, but it failed to detect
the GTHTR300C source-variant mismatch.

It states that the Nishihara source supports a 170 MWth reference IHX and 202
MWe together with the 59.7/70.9 bn JPY economics. The primary Nishihara paper
instead uses a 370 MWth IHX and 88 MWe for that economic case.

The previous verification also did not detect the factor-of-1000
`lifecycle_ci_kgkg` implementation defect.

Accordingly, the previous file must not be treated as independent evidence of
final-design correctness.

## Findings summary

### BLOCKER
- **FDV2-B01:** JAEA hardware/economic source variants are conflated, invalidating
  the exact final reactor/electricity economics and doubled-IHX rationale.

### MAJOR
- **FDV2-M01:** factor-of-1000 error in `lifecycle_ci_kgkg`.
- **FDV2-M02:** helium-loop/process-heat/electricity output are not closed as one
  integrated thermal state.

### MINOR
- The 950 C JAEA primary value should be described as a hardware envelope, not
  part of the actual INL Case-6 925->900 C temperature calculation.
- 585 C secondary return and constant-Cp helium flow should remain explicitly
  project screening assumptions; INL Case 6 itself reports 466 C return and
  78.49 kg/s.
- S$150/MWh remains a market-context scenario, not an offtake guarantee.

## Required corrective handoff

1. **Resolve FDV2-B01 first.** Select a coherent GTHTR300C economic/hardware
   configuration and remove the 170/202 versus 370/88 cross-variant mixing.
   Recompute plant cost, heat/electricity burden, export and zero-value case.
   **Acceptance:** one traceable architecture closes full reactor economics and
   still meets the CN4252 thresholds if a pass is claimed.

2. **Resolve FDV2-M02.** Close the final primary/secondary helium and power-cycle
   thermal balance against the selected architecture.
   **Acceptance:** 176.8 MW process duty, helium temperatures/flow, residual
   power-cycle heat and gross MWe are mutually energy-consistent.

3. **Resolve FDV2-M01.** Correct the lifecycle-intensity unit conversion.
   **Acceptance:** candidate CI is ~1.95 kgCO2e/kgH2 under the current proxy
   boundary (subject to any upstream corrections), verified by dimensional test.

4. Re-run final-design results, Research CI, Paper CI and exact PDF inspection.
   Preserve the INL source process and the independently reproduced
   ~917 kt/y lifecycle-abatement arithmetic unless a corrected integrated
   architecture materially changes the lifecycle boundary.

## Independent conclusion

The **INL process design and ~0.917 MtCO2e/y lifecycle-abatement arithmetic are
independently reproduced under the declared proxy boundary**.

The **exact final techno-economic result is not independently verified** because
the frozen implementation combines hardware/electricity quantities from one
GTHTR300C variant with economics from another. The source economic variant's
reference IHX is 370 MWth, not 170 MWth, and its gross electricity is 88 MWe,
not 202 MWe. The current doubled-IHX rationale and -S$194.7/t headline therefore
cannot stand as a source-consistent final result without correction.

A reviewer sensitivity indicates that a corrected coherent JAEA variant may
still satisfy the CN4252 cost threshold, so this finding does not establish that
the proposed concept fails. It establishes that the current frozen final-design
verification is incomplete and the exact economic claim must be recomputed.

**FINAL DESIGN NOT YET INDEPENDENTLY VERIFIED — CORRECTIONS REQUIRED**
