# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation, early baseline construction

## Current scientific question/task
What is the minimum defensible conventional SMR + CCS reference model needed to quantify the marginal value of nuclear process heat?

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
The ideal combined SMR/WGS reaction gives:
CH4 + 2 H2O -> CO2 + 4 H2.

This imposes a stoichiometric lower-bound relationship of about 1.99 kg CH4/kg H2 and 5.46 kg CO2/kg H2 for complete methane-to-hydrogen conversion.

A published Aspen-based 500 t/d SMR case reports 3.16 kg NG/kg H2 and 8.47 kg CO2/kg H2. The difference from the ideal bound shows that a realistic plant-level model must capture feed composition, conversion/recovery losses, fuel use and heat integration rather than using stoichiometry alone.

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
- Convert the baseline equations into a verified Rust implementation.
- Determine a consistent natural-gas composition and steam-to-carbon basis.
- Separate process CO2, furnace CO2, PSA/tail-gas carbon and upstream methane emissions.
- Establish a heat-duty model with stream temperature levels.
- Choose capture topology only after baseline stream concentrations and duties are quantified.

## Major unresolved scientific questions
1. What baseline configuration best represents a Singapore industrial SMR without biasing the nuclear comparison?
2. How much of total CO2 is process carbon versus combustion carbon for that configuration?
3. How much reformer duty remains after internal heat recovery?
4. What fraction of the CCS regeneration duty should be allocated to each energy source?
5. What reactor outlet temperature and heat-transfer approach are actually feasible?
6. What is the fair counterfactual for direct HTGR heat versus nuclear-electric eSMR?
7. Do upstream methane emissions erase much of the apparent plant-level CCS benefit?
8. What production scale is compatible with a Singapore deployment scenario?
9. Can the integrated system achieve the assignment's <S$100/tCO2e threshold on a transparent incremental-cost basis?

## Blockers
- Rust toolchain is not installed in the current execution environment, so the new baseline code scaffold has not been build-tested here.
- Exact Singapore-relevant natural-gas and cost data still need to be selected with provenance.
- Existing Singapore SMR/CCS work needs deeper review to define the originality boundary.

## Verification/build status
- No production simulation has been accepted as a scientific result.
- Baseline equations are analytical definitions, not numerical simulation outputs.
- Literature benchmark values are clearly marked as benchmark/source values.
- No final reactor or CCS topology has been selected.
- GitHub repository is public and being used as the canonical research workspace.
- CI configuration exists but has not been observed running successfully from this environment.

## Next highest-priority task
Build the conventional SMR baseline in Rust and verify it against at least two independent published/authoritative benchmark cases before introducing nuclear heat.

Required checks:
1. elemental mass conservation;
2. reaction stoichiometry;
3. H2 recovery/purity consistency;
4. total carbon balance;
5. process-versus-combustion CO2 accounting;
6. energy-balance closure;
7. benchmark comparison with stated tolerances;
8. annual abatement scaling calculation.
