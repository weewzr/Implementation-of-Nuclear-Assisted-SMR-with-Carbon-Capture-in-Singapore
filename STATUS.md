# STATUS

## Current research gate
Gate 6 — Paper/reproducibility: COMPLETE FIRST MANUSCRIPT; READY FOR INDEPENDENT REVIEW 4.

Gate 5 — Experiments/results: COMPLETE.

Gate 4 — Verified computational model: COMPLETE.

Independent Review 1 gate: CLOSED.
Independent Review 2 gate: CLOSED.
Independent Review 3 / Gate 4 gate: CLOSED.

## Verified computational state
- Canonical R3 flowsheet starts from the IEAGHG NG/steam-derived external feed,
  then reforming/WGS, explicit CO2 removal, PSA, purge and recycle.
- Plant-boundary C/H/O/N and total mass closure are CI-tested.
- Captured CO2 is physically removed before downstream PSA/recycle.
- Candidate heat integration uses positive process/IHX approaches:
  900 C process, 920 C secondary-He hot end, 950 C primary outlet in the
  reference screening hierarchy.
- Candidate CCS and energy ledgers use the same scaled MDEA duty; recovered heat
  is allocated once and the first-law ledger is tested.
- Lifecycle emissions include residual carbon, upstream NG, nuclear heat,
  CCS/tail-compression and helium-circulator electricity, plus CCS transport.
- Economics are forward calculations; S$100/t is a test threshold, not an input
  to the cost equation.
- Singapore deployment scale is derived from the corrected R3 state and nuclear
  deployment / cross-border storage remain explicit scenario conditions.
- Gate-4 F/G/H tests cover external benchmark, integrated-system and adversarial
  failure modes.

## Canonical results and adverse findings
Canonical contract: `results/r3_canonical_results.md`.

The verified model does NOT establish CN4252 feasibility:
- conservative credible case fails both assignment thresholds;
- coupled uncertainty design contains 64 points and **0 joint passes**;
- IEAGHG Case 1A conventional SMR+CCS exceeds 0.25 Mt/y direct avoided CO2 at
  the common H2 scale;
- nuclear-assisted SMR+CCS is not established as a preferred solution.

These adverse results are retained as scientific findings, not treated as model
failures.

## Current CI state
Current canonical paper/reproducibility workflow is verified:
commit `b9ca2896c442071f7694a811b4a4383c71fee7fe`,
GitHub Actions Paper run `36566879465`: PASS.

Acceptance evidence:
- canonical Rust tests completed successfully;
- canonical Gate-5/Gate-6 manuscript datasets regenerated;
- clean LaTeX/BibTeX build converged;
- final unresolved citation/reference integrity check passed;
- `paper/main.pdf` generated successfully (13 pages in the converged CI build);
- `cn4252-manuscript` artifact uploaded successfully, artifact ID
  `11032795799`, SHA-256 digest
  `4ccb56f742cbd55c8b3f139ec01612c23ed4db52d697a2b1630cc364fc32781b`.
Research CI run `36566879451`: PASS.

First-pass LaTeX warnings for citations/references occurred before BibTeX and
subsequent LaTeX passes, then resolved during the converged build. They are not
remaining unresolved citations/references.

## Retained limitations
- Reformer/prereformer treatment remains a screening model, not catalyst
  kinetics.
- PSA remains a bounded recovery model, not a bed-resolved adsorption cycle.
- Detailed exchanger area/pinch-network and piping design are outside scope.
- Economic prices/CAPEX/T&S remain scenario assumptions unless source-labelled.
- eSMR/electrolysis do not yet have a full matched Singapore forward-cost model.
- Singapore has not been assumed to have deployed nuclear or domestic CO2
  storage.

## Blockers
No unresolved scientific, manuscript-build or reproducibility blocker prevents
Independent Review 4. Review 4 has NOT been conducted by Main Research.

## Gate-5 progress
- Experiment 01: threshold failure topology — COMPLETE.
- Experiment 02: local threshold-driver attribution — COMPLETE.
- Experiment 03: common-scale IEAGHG Case-1A decomposition — COMPLETE.
- Experiment 04: binding-constraint map — COMPLETE.
- Canonical CSV/Markdown renderers and reproduction commands — COMPLETE.
- Figure-ready 64-case threshold dataset and claim-strength synthesis — COMPLETE.
- Deterministic figure-data pipeline and manifest — COMPLETE.

Gate-5 result contract: `results/GATE5_RESULTS.md`.

The experimental evidence currently remains adverse to a robust nuclear case:
0/64 coupled cases pass both CN4252 thresholds. Do not optimize this result away.

## Gate-5 closure
Gate 5 is CLOSED. Closure record: `results/gate5_closure.md`.

The experimental conclusion remains adverse/conditional:
- 0/64 tested nuclear cases pass both CN4252 thresholds;
- the conservative case fails both;
- Case 1A exceeds the annual direct-abatement scale;
- no technology is established as a definitive Singapore economic winner on a
  fully matched basis.

## Gate-6 progress
- Single canonical modular LaTeX manuscript under `paper/`: ESTABLISHED.
- Abstract through conclusions: FIRST COMPLETE TEXT DRAFT.
- Governing equations / verification / adverse Gate-5 results: INTEGRATED.
- Baseline LaTeX/PDF CI build: VERIFIED PASS at `eeab77d`.
- Reproducible Gate-5 threshold map and binding table: INTEGRATED; latest render
  CI pending.
- Reproducible local driver/sensitivity figure: INTEGRATED; latest render CI
  pending.
- Original integrated nuclear-SMR-CCS system schematic: INTEGRATED; latest
  render CI pending.
- IEAGHG Case-1A claim-labelled comparator table: INTEGRATED.
- Singapore deployment-scale table: INTEGRATED.
- Lifecycle-emissions decomposition: INTEGRATED with screening limitations
  explicit.
- Primary-source coverage strengthened for IEAGHG, NIST, JAERI/JAEA and official
  Singapore nuclear/CCS context; further citation completeness review remains.
- Reproducibility script and CN4252 traceability appendix: ESTABLISHED.

## Next step
STOP Main Research manuscript expansion. The complete first manuscript now
satisfies the Review-4 trigger and is ready for Independent Review 4. Main
Research must not conduct Review 4 itself.

Canonical review records:
- `reviews/review_01_resolution.md`
- `reviews/review_02_resolution.md`
- `reviews/review_03_integrated_model_results.md`
- `reviews/review_03_resolution.md`
