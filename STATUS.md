# STATUS

## Current research gate
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

## Next step
Gate 5 — experiments/results.

Gate 5 must use the corrected R3 canonical model and must preserve falsification:
experiments should map and explain the no-pass region, comparator behaviour and
threshold drivers rather than optimize the nuclear case toward a desired answer.

Canonical review records:
- `reviews/review_01_resolution.md`
- `reviews/review_02_resolution.md`
- `reviews/review_03_integrated_model_results.md`
- `reviews/review_03_resolution.md`
