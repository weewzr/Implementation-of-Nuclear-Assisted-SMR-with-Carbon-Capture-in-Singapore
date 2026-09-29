# STATUS

## Current research gate
Gate 6 — Paper/reproducibility: IN PROGRESS.

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
Focused Review-3 closure state
`c384e053e8ea1ec42f1d471fe7f7733dda022b75`:
GitHub Actions run `36545720485` PASS (`cargo test --all-targets`).

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
No unresolved Review-3 BLOCKER or required MAJOR corrective action prevents
using the model for Gate-5 experiments/results.

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
- Canonical modular LaTeX manuscript under `paper/`: FIRST COMPLETE TEXT DRAFT.
- Abstract through conclusions: populated from verified evidence.
- CN4252 requirement traceability appendix: created.
- Reproducibility appendix and `scripts/reproduce.sh`: created.
- Starter bibliography: created; citation coverage still requires strengthening.
- Paper/reproducibility GitHub Actions workflow: created.
- Manuscript build CI: currently being verified.

## Gate-6 progress
- Single canonical LaTeX manuscript under `paper/`: ESTABLISHED.
- Abstract through conclusions: FIRST-PASS CONTENT ESTABLISHED.
- Governing equations / verification / adverse Gate-5 results: INTEGRATED.
- Canonical bibliography: ESTABLISHED; requires progressive source enrichment.
- CN4252 requirement traceability: ESTABLISHED.
- Reproducibility build script: ESTABLISHED.
- CI LaTeX build: ADDED; first build validation in progress.
- Historical `manuscript/main.tex`: SUPERSEDED provenance pointer.

## Next step
Resolve the first LaTeX CI build if necessary, then deepen the manuscript with
canonical generated tables/figures and fuller primary-source citations. Do not
request Independent Review 4 until the complete first manuscript has all major
results, figures, discussion, limitations and conclusions.

Canonical review records:
- `reviews/review_01_resolution.md`
- `reviews/review_02_resolution.md`
- `reviews/review_03_integrated_model_results.md`
- `reviews/review_03_resolution.md`
