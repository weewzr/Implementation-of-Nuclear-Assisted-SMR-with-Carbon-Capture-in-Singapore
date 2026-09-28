# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation; Independent Review 1 gate OPEN

## Current scientific question/task
Decompose the now-verified total SMR/CCS energy ledger into temperature-resolved process services before any detailed nuclear heat substitution.

## Independent Review 1
- Review-response pass started from the substantive review available in the
  CN4252 Project conversation.
- The requested repository file `reviews/review_01_research_foundation.md`
  was absent when checked; this provenance discrepancy is recorded rather than
  hidden.
- Finding dispositions and acceptance criteria are recorded in
  `reviews/review_01_resolution.md`.
- Review gate remains OPEN because valid scientific blockers remain.

## Newly resolved scientific issue
The previous stream-4 -> stream-5 elemental non-closure has been diagnosed from
the full IEAGHG 2017-02 process description.

Stream 4 is feed to the pre-reformer. Between stream 4 and the published HTS
inlet (stream 5), IEAGHG explicitly describes a second HP-superheated-steam
addition and BFW desuperheating before the primary reformer.

Using the rounded published stream compositions:
- carbon residual is only about -0.020 kmol-C/h;
- hydrogen residual implies about 155.2 kmol/h H2O;
- oxygen residual implies about 153.7 kmol/h H2O;
- least-squares reconciliation gives about 154.4 kmol/h aggregate H2O addition.

Including that source-described but unnumbered water/steam addition closes C/H/O
to <0.1% of the stream-4 elemental inventories without tuning published carbon
species. The material-boundary component of Review blocker B1 is therefore
resolved.

## New energy result
IEAGHG's fixed-output comparison is now encoded and CI-verified:
- Base: 394.77 MW NG input, 9.918 MWe export, 0.8091 kgCO2/Nm3 H2.
- Shifted-syngas MDEA (1A): 407.68 MW NG, 1.492 MWe export, 0.3704 kgCO2/Nm3.
- Flue-gas MEA (3): 433.72 MW NG, 0.426 MWe export, 0.0888 kgCO2/Nm3.

Thus Case 1A adds 12.91 MW NG and loses 8.426 MWe export while avoiding 43.87 tCO2/h plant-gate; Case 3 adds 38.95 MW NG and loses 9.492 MWe while avoiding 72.03 tCO2/h. Capture topology therefore materially changes the energy burden and cannot be selected independently of nuclear integration.

## Preserved findings
- PSA tail gas remains a first-order nuclear-integration constraint.
- Replacing make-up furnace NG alone does not remove feedstock carbon.
- Existing HTS C/H/O closure and reaction-extent verification remain valid.
- The 298 K reaction-duty calculation is a lower thermochemical layer, NOT
  reformer furnace duty.
- Captured CO2 and avoided CO2 remain separate metrics.
- No final nuclear reactor or CCS topology has been selected.

## Valid blockers still open
1. **Conventional energy balance:** temperature-dependent enthalpy, steam
   generation/superheat, reformer duty, heat recovery and furnace losses are not
   yet closed against an authoritative benchmark.
2. **PSA-tail-gas disposition:** every species needs a defined destination in
   each furnace-free/nuclear candidate.
3. **Capture topology:** cannot be frozen until tail-gas/carbon architecture is
   selected; stream-specific pressure/composition must drive solvent choice and
   regeneration duty.
4. **Common comparison specification:** 1 kg H2 is the canonical mass basis,
   but final product pressure and full nested boundary matrix still require
   authoritative selection.

## Major items coupled to blockers
- Add stream-specific CCS literature/parameters.
- Build thermodynamic property layer.
- Preserve nuclear integration as a heat-temperature envelope before mapping
  reactor concepts.
- Freeze transparent incremental economic assumptions before assignment-level
  S$/tCO2e results.
- Add independent predictive validation beyond source reconstruction.

## Verification status
- Rust regression for the corrected stream-4/5 interpretation and inferred interstage water addition passed GitHub Actions.
- IEAGHG baseline energy-invariant tests also passed GitHub Actions.
- A later auxiliary reformer diagnostic is still running; it is not required for the already-passed reconciled closure test.
- No production simulation is accepted as a scientific result.
- No detailed integrated nuclear model is permitted while Review 1 blockers
  remain.

## Next highest-priority task
Decompose the conventional SMR energy ledger into reformer radiant/process duty, convection/feed preheat, steam generation/superheat, syngas heat recovery, furnace losses and PSA-tail-gas chemical energy. Attach temperature levels and verify the sum against the authoritative 394.77 MW NG input plus reported power/steam outputs.

Acceptance criterion: conventional reference energy demand and major duties
close and reproduce an authoritative published metric within a declared
tolerance before nuclear heat is substituted.
