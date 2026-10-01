# Review 07 Resolution — Deep Feasibility

Review: `reviews/review_07_deep_feasibility.md`  
Review commit: `f5d3bcf022c10b773a0bbe6d57328206f2fb4a0d`  
Decision entering resolution: **DEEP FEASIBILITY VERIFIED WITH BOUNDED CORRECTIONS REQUIRED**

## DFR-M01 — Availability economics

**Severity:** MAJOR

**Review finding:** the no-backup availability sensitivity scaled installed/fixed reactor, CCS-capital and integration burdens with operating hours/throughput, so its economic result was not a physically consistent downtime sensitivity.

**Action taken:** removed all economic fields/pass criteria from `AvailabilitySensitivity`. The function now exposes only effective process availability, annual H2 throughput, lifecycle avoided emissions and the CN4252 annual-abatement pass. The CSV/summary were reduced accordingly. The unsupported no-backup availability-cost figure is no longer generated. No fixed/variable cost fractions were invented.

**Files changed:** `model/src/final_design.rs`, reliability summary/generator and figure code, manuscript results section.

**Test/evidence:** base H2/lifecycle reconciliation retained; monotonic emissions and explicit ~23.2% threshold regression added. Gas-backup sensitivity remains a separate lower-bound operating screen with installed central burden retained and omitted backup CAPEX/fixed O&M explicitly stated.

**Scientific conclusion changed?** No central-case result changed. The interpretation of the no-backup sensitivity is narrowed.

**Final status:** **RESOLVED**.

## DFR-M02 — CCS sensitivity boundary

**Severity:** MAJOR

**Review finding:** the CCS sensitivity mixed reduced delivered capture/storage with plant resizing by shrinking CCS CAPEX/integration costs while retaining canonical process electricity.

**Action taken:** converted the sensitivity to an **emissions-only algebraic delivered-storage screen**. The source process operating point remains fixed; a non-delivered portion of the canonical captured stream is returned to direct emissions; T&S lifecycle burden scales only with delivered/stored tonnes. All CAPEX, cost, turndown and economic pass fields were removed.

**Files changed:** `model/src/final_design.rs`, CCS CSV/summary, CCS evidence file, figure labels/captions and manuscript results section.

**Test/evidence:** f=1 lifecycle reconciliation; monotonic delivered tonnes/lifecycle abatement; f=0 regression near 0.388 MtCO2e/y with >0.58 Mt/y candidate direct emissions. No installed CAPEX disappears in this sensitivity because economics are not exposed.

**Scientific conclusion changed?** No central case changed. The robustness interpretation is narrowed.

**Final status:** **RESOLVED**.

## DFR-M03 — Consolidated Future Work

**Severity:** MAJOR

**Review finding:** strong repository future-work detail existed but the active manuscript lacked a consolidated decision-oriented Future Work section.

**Action taken:** added `paper/sections/12a_future_work.tex` before Conclusions with a compact question / next analysis / required input / expected output / decision-enabled table.

Covered: 176.8 MWth IHX qualification; reactor-trip/reformer safe state; mechanistic source term/PRA/Singapore EPZ; nuclear/chemical separation; heat rejection/cooling; tritium; site comparison without selection; CCS infrastructure; bankable FOAK economics.

**Final status:** **RESOLVED** subject to final PDF visual QA.

## DFR-m01 — Question-register status

**Severity:** MINOR

**Action taken:** DF-07 now records the implemented emissions/throughput threshold screen separately from unresolved physical reliability. DF-09 now records the implemented lower-bound gas-backup operating screen separately from unresolved physical backup design. DF-28 is explicitly emissions-only. DF-31 records the unavailable source as waived rather than unresolved source recovery.

**Final status:** **RESOLVED**.

## DFR-m02 — STATUS consistency

**Severity:** MINOR

**Action taken:** removed stale wording describing the second professor transcript as an active blocker. Current-facing text states that it was unavailable and explicitly waived; no claims are attributed to it.

**Final status:** **RESOLVED**.

## DFR-P01 — Availability figure label

**Severity:** PRESENTATION

**Action taken:** the availability figure now states that definitions differ by series. Legend/caption distinguish:
- no backup = **effective process availability**, emissions/throughput only;
- gas backup = **nuclear-source availability**, while process service is held at 85%.

**Final status:** **RESOLVED** subject to final PDF visual QA.

## Central-result regression requirement

The correction pass is not allowed to change the verified central case. Final closure requires tests/CI to reconfirm approximately:
- 97,946 tH2/y;
- 862,094 t/y direct CO2 avoided;
- 917,139 tCO2e/y lifecycle avoided;
- 1.95 kgCO2e/kgH2;
- 176.8 MWth process heat;
- S$0/MWh project electricity revenue;
- S$3.725/tCO2e central screening abatement cost.

## Closure gate

This resolution record is **provisional until the corrected HEAD passes**:
- Rust tests and deterministic generation;
- Research CI;
- Paper/reproducibility CI;
- bibliography/citation/reference integrity;
- exact corrected PDF page-by-page visual QA.

Only then may STATUS be marked **REVIEW 07 RESOLVED — FINAL SUBMISSION CANDIDATE READY FOR CLOSURE**.
