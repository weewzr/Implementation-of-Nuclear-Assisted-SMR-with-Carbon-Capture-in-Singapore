# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation, early baseline construction

## Current scientific question/task
Extend the validated species-resolved PSA reconstruction upstream into reformer/WGS reaction extents, water/steam balance and energy duty before introducing nuclear heat.

## Completed
- Read the complete Project Brief and official CN4252 Problem Statement.
- Established the authoritative assignment requirements matrix.
- Formalised the proposed HTGR-assisted SMR + amine CCS architecture as a hypothesis, not a conclusion.
- Conducted the initial external landscape review covering SMR, CCS, amine regeneration, methane leakage, HTGR process heat, eSMR, electrolysis, Singapore nuclear/hydrogen/CCS context, TEA/LCA methods and relevant open-source software.
- Verified the existing public project repository and inspected the supervisor-reported OUTRAM PARK Rust repository for rendering/software provenance.
- Established the initial comparison set: fired SMR; fired SMR + CCS; eSMR + CCS; nuclear-electric eSMR + CCS; direct HTGR-heated SMR + CCS; and a low-carbon electrolysis benchmark.
- Added the official problem statement and working project brief to the repository as provenance records.
- Added first-principles SMR stoichiometric equations, lifecycle/abatement definitions and benchmark parameter provenance.
- Established the CN4252 scale identity: required annual H2 production depends on the marginal CO2e reduction per kg H2, not on capture percentage alone.

## Most important scientific finding so far
The published IEAGHG heat/material balance permits an independent carbon-atom closure.

For its natural-gas composition, the base case contains about 1578.6 kmol-C/h in NG feedstock and 260.7 kmol-C/h in make-up furnace fuel. Published flue gas contains about 1838.4 kmol-C/h versus 1839.3 kmol-C/h entering: 99.95% closure using rounded source values.

More importantly, the PSA tail gas contains about 1578.5 kmol-C/h — essentially 100% of the feedstock carbon before firing. The separately supplied furnace NG is only about 14.2% of total incoming carbon.

Therefore replacing furnace heat with nuclear heat does NOT remove the feedstock-carbon problem. The nuclear flowsheet must capture, convert, recycle or otherwise treat the PSA/tail-gas carbon.

Even at the PSA inlet, removing all existing CO2 would leave about 505 kmol-C/h as CO + CH4 in the reference stream. This residual carbon is now a first-order design constraint.

## Key assignment implication
If the candidate system reduces emissions by Delta-e kgCO2e/kgH2 relative to the counterfactual, the production scale needed to exceed 0.25 MtCO2e/y is:

H2_required [kt/y] > 250 / Delta-e.

The scale therefore becomes very large when the candidate's marginal advantage over the baseline is small.

## Current model boundary
Initial baseline:
natural gas + water/steam -> reforming -> WGS -> H2 separation -> CO2 capture -> solvent regeneration -> CO2 compression.

Lifecycle extension:
natural-gas upstream emissions + electricity + nuclear lifecycle + CO2 transport/storage.

## Work in progress
- Rust screening model implements the IEAGHG 2017-02 base case, shifted-syngas MDEA Case 1A, full carbon closure, and explicit H2/CO2/CO/CH4 PSA inlet/tail streams.
- Source stream reconstruction gives approximately 89.9% PSA H2 recovery from rounded IEAGHG values.
- The reference PSA tail gas is approximately 47.7 mol% combustible H2+CO+CH4; approximately 32% of its carbon is in CO+CH4 rather than CO2.
- CI for the newest baseline tests is currently queued/in progress; do not treat it as passed until GitHub reports success.
- Determine a consistent natural-gas composition and steam-to-carbon basis.
- Separate process CO2, furnace CO2, PSA/tail-gas carbon and upstream methane emissions.
- Establish a heat-duty model with stream temperature levels.
- Choose capture topology only after baseline stream concentrations and duties are quantified.

## Major unresolved scientific questions
1. What happens to PSA tail gas when the fired reformer is replaced by nuclear heat? This is now a first-order integration constraint because conventional SMR burns PSA tail gas as primary furnace fuel.
2. How much of total CO2 is process carbon versus combustion carbon for that configuration?
3. How much reformer duty remains after internal heat recovery?
4. What fraction of the CCS regeneration duty should be allocated to each energy source?
5. What reactor outlet temperature and heat-transfer approach are actually feasible?
6. What is the fair counterfactual for direct HTGR heat versus nuclear-electric eSMR?
7. Do upstream methane emissions erase much of the apparent plant-level CCS benefit?
8. What production scale is compatible with a Singapore deployment scenario?
9. Can the integrated system achieve the assignment's <S$100/tCO2e threshold on a transparent incremental-cost basis?

## Blockers
- Local Rust is unavailable, but GitHub Actions is now being used for actual build/test verification; the newest run is pending.
- The nuclear flowsheet cannot be closed until PSA tail-gas disposition is selected/modelled.
- Exact Singapore-relevant natural-gas and cost data still need to be selected with provenance.
- Existing Singapore SMR/CCS work needs deeper review to define the originality boundary.

## Verification/build status
- No production simulation has been accepted as a scientific result.
- Baseline equations are analytical definitions, not numerical simulation outputs.
- Literature benchmark values are clearly marked as benchmark/source values.
- No final reactor or CCS topology has been selected.
- GitHub repository is public and being used as the canonical research workspace.
- Earlier Rust CI runs have completed successfully; CI for the newest baseline/tail-gas tests is pending.

## Next highest-priority task
Reconstruct reformer + WGS reaction extents and the water/steam balance that produce the validated PSA inlet. Then calculate the associated reaction/sensible heat duties with source-backed thermochemistry. Acceptance criterion: C/H/O closure plus reproduction of the published PSA inlet and key heat/material balance values within declared tolerances.

Required checks:
1. elemental mass conservation;
2. reaction stoichiometry;
3. H2 recovery/purity consistency;
4. total carbon balance;
5. process-versus-combustion CO2 accounting;
6. energy-balance closure;
7. benchmark comparison with stated tolerances;
8. annual abatement scaling calculation.
