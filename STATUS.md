# STATUS

## Current research gate
Gate 1 — Research framing and landscape; ready to progress into mathematical/model foundation after baseline data selection.

## Current scientific question/task
Determine the minimum defensible reference system against which the proposed HTGR-assisted SMR + CCS concept must be compared.

## Completed
- Read the complete Project Brief.
- Read the complete official CN4252 Problem Statement.
- Confirmed the Problem Statement is authoritative for assignment requirements.
- Formalised the assignment requirements matrix.
- Checked whether HTGR-assisted SMR + amine CCS can satisfy the assignment conceptually.
- Identified missing proof obligations: annual scale, <S$100/tCO2e cost, counterfactual, lifecycle emissions, roadmap, originality, and alternatives.
- Searched conventional SMR, process/combustion CO2, CCS, amine regeneration, methane leakage, HTGR process heat, nuclear-integrated hydrogen, eSMR, electrolysis, Singapore energy/hydrogen/nuclear/CCS context, TEA/LCA methods, and open-source implementations.
- Inspected the supervisor-reported OUTRAM PARK Rust repository and documented its relationship to the original process figure.
- Updated the research matrix with primary/authoritative sources and explicit gaps.
- Preserved the student's initial architecture as a hypothesis rather than a conclusion.
- Initial public GitHub repository structure established.

## Most important finding
The broad concept is technically legitimate but is not novel by itself. Direct HTGR-heated SMR has prior modelling and, by 2026, direct literature comparison against HTGR-electric eSMR already exists. Singapore also already has SMR/CCS studies and a 2025 Aster/Air Liquide low-carbon hydrogen + integrated carbon-capture initiative.

Therefore the project should not claim invention of nuclear-assisted SMR. Its defensible contribution is a Singapore-specific, consistently bounded, quantitatively scaled comparison of direct nuclear heat, nuclear electricity/eSMR, conventional SMR+CCS, and other relevant pathways against the CN4252 thresholds.

## Key assignment risk
A generic "SMR + CCS in Singapore" proposal could be interpreted as a regurgitation of existing work. Nuclear-heat integration and the comparative systems analysis must therefore be central.

## Major unresolved scientific questions
1. What baseline hydrogen plant capacity and operating basis are appropriate for Singapore?
2. What exact process/combustion CO2 split results from the selected SMR configuration?
3. What heat duty remains after heat recovery, and at what temperature level?
4. Can an HTGR supply that duty through a physically credible heat-exchanger/interface without violating reactor/process constraints?
5. How should CCS regeneration heat be supplied in each comparator?
6. What is the fair counterfactual for annual avoided CO2e?
7. How sensitive is lifecycle performance to methane leakage, gas source, capture rate and CO2 storage assumptions?
8. What production scale is required to exceed 0.25 MtCO2e/y?
9. What CAPEX/OPEX and financing assumptions are needed for <S$100/tCO2e?
10. How does the nuclear case compare with direct electricity/eSMR and electrolysis when the same heat/electricity accounting boundary is used?

## Work in progress
- Converting the literature landscape into a parameter provenance table.
- Selecting baseline model equations.
- Determining the smallest model capable of falsifying the initial hypothesis.

## Blockers
- Need validated SMR operating conditions and energy duties before quantitative integration.
- Need consistent Singapore-relevant natural-gas, electricity, CO2 transport/storage and cost assumptions.
- Need deeper review of exact Singapore existing-project documentation to establish originality boundary.

## Verification/build status
- No research simulation has been run or accepted yet.
- No numerical model has been validated.
- No final reactor selection has been made.
- Rust implementation has not yet begun.
- GitHub repository files have been created/updated through the connected GitHub account.
- OUTRAM PARK source code was inspected for provenance/reference purposes only.

## Next highest-priority task
Build the literature-backed conventional SMR + CCS baseline:
1. define functional unit and plant capacity;
2. derive CH4/H2/H2O/CO/CO2 material balances;
3. derive reformer, WGS and heat-recovery duties;
4. separate process and combustion CO2;
5. define candidate capture boundaries;
6. establish a transparent CO2 abatement equation;
7. benchmark the resulting model against authoritative and peer-reviewed values.
