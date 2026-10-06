# Post-Screening Engineering Closure Workflow

## Purpose

This workflow supersedes any assumption that the current differential screening economics or qualitative feasibility discussion is sufficient for the final CN4252 report.

The existing verified screening model and S$3.725/tCO2e result remain preserved as historical/audit evidence. They are **not to remain the controlling final-report economic conclusion** if the stronger engineering-economic workflow produces a better-supported result.

The objective is to replace avoidable screening bridges with the strongest defensible engineering analysis supported by available data, while retaining literature values where the user has explicitly chosen to defer fundamental simulation/model redevelopment.

## Governing principle

Do not try to make the proposal pass or fail.

Try to falsify every favourable project-specific bridge.

For each bridge:
1. identify what is currently assumed/screened;
2. determine what engineering calculation/evidence would replace it;
3. calculate/model it where data are sufficient;
4. refuse to invent missing inputs;
5. propagate any changed result;
6. state what remains unresolved.

## Phase E0 — Cost-method reconciliation
Complete the active cost-method reconciliation using the uploaded CAPCOST/CCEP/DFP/CN4119 materials.

Output: a decision on which costing methods are presentation-only, valid replacements, valid additions, blocked by missing data, not applicable, or double-counting.

Do not preserve S$3.725/tCO2e merely because it is canonical.

## Phase E1 — Engineering-economic model replacement
Only after E0.

Construct the strongest applicable economic model supported by available design data.

Priorities:
- equipment/process CAPEX where calculable;
- installed/fixed/total capital distinctions;
- manufacturing/OPEX where calculable;
- nuclear cost treatment without misusing generic chemical correlations;
- CCS cost treatment without double counting IEAGHG aggregate costs;
- cost-year/currency discipline;
- annualisation;
- FOAK/site/integration categories where defensibly quantifiable;
- uncertainty/ranges where point precision is unjustified.

The old differential screening bridge remains repository history but should be removed as the controlling final-report result once superseded.

## Phase E2 — Incremental nuclear-benefit attribution
Quantify what nuclear heat adds relative to the strongest matched non-nuclear comparator.

At minimum separate:
- conventional unabated SMR-H2;
- SMR-H2 + CCS without nuclear heat;
- HTGR-assisted SMR-H2 + CCS.

Calculate, on matched H2 service and consistent boundaries:
- direct emissions;
- lifecycle emissions;
- annual abatement;
- represented cost;
- incremental abatement attributable to CCS;
- incremental abatement attributable to nuclear heat;
- incremental cost attributable to nuclear integration;
- incremental S$/tCO2e of the nuclear step where defensible.

Do not claim nuclear is necessary if CCS supplies most of the benefit.

## Phase E3 — Jurong Island candidate-context feasibility
Treat Jurong Island as a candidate industrial integration context, not an approved nuclear site.

Research current authoritative evidence for:
- LCT3 / low-carbon technology testbed development;
- hydrogen-ready/low-carbon industrial infrastructure;
- land/industrial zoning context;
- hazardous-industry co-location;
- grid/utility integration;
- natural-gas access;
- CO2 export/logistics;
- port/shipping implications;
- candidate cooling-water/heat-rejection options;
- seawater/environmental constraints;
- footprint/separation requirements;
- emergency planning implications.

Where possible construct engineering screens for:
- plant footprint;
- heat rejection;
- cooling-water duty/flow;
- cooling architecture;
- separation/buffer needs.

Do not claim a nuclear site has been selected or approved.

## Phase E4 — Integrated nuclear/chemical safety case
This is a major CN4252 feasibility workstream.

Build a project-specific screening safety case, not merely a literature narrative.

Create:
- hazard inventory;
- initiating-event register;
- barrier/defence-in-depth map;
- nuclear-to-chemical propagation analysis;
- chemical-to-nuclear propagation analysis;
- common-cause/external-hazard analysis;
- safe-state requirements;
- evidence-maturity classification.

At minimum assess:
- reactor trip;
- loss of forced cooling;
- loss of process heat sink / sudden reformer trip;
- helium depressurisation;
- air ingress;
- water/steam ingress;
- IHX leakage/failure;
- tritium transfer/permeation;
- loss of secondary-He circulation;
- reformer fire;
- methane release;
- hydrogen release/explosion;
- CO release;
- high-pressure steam/process rupture;
- amine hazards where material;
- CO2 release/asphyxiation;
- nuclear/chemical co-location hazards;
- external hazards relevant to Jurong Island.

Use HTTR/HTGR experimental evidence where applicable but distinguish demonstration from project-specific proof.

## Phase E5 — PRA/source-term/QRA depth boundary
Determine how far the project can legitimately go toward quantitative risk.

Attempt quantitative screening only where inputs are supportable.

Define explicitly what would be required for:
- event frequencies;
- event trees/fault trees;
- mechanistic source term;
- radionuclide transport;
- site meteorology;
- dispersion/dose;
- EPZ;
- nuclear/chemical QRA;
- individual/societal risk.

Do not fabricate frequencies or dose results.

Where full PRA/QRA cannot be performed, produce a rigorous requirements/data-gap specification rather than generic 'future work'.

## Phase E6 — CN4252 feasibility/effectiveness synthesis
Re-evaluate the proposal against the exact official requirements:

- >0.25 MtCO2e/y;
- <S$100/tCO2e using the strongest current economic model;
- solution-at-scale within Singapore;
- clear abatement mechanism;
- implementation roadmap;
- originality;
- feasibility;
- potential effectiveness;
- accuracy;
- presentation.

Classify each dimension:
SUPPORTED / CONDITIONAL / UNRESOLVED / NOT DEMONSTRATED / FAILS.

Explicitly address what 'within Singapore' means when CO2 is captured in Singapore and transported cross-border for storage.

## Phase E7 — Implementation roadmap
Update the roadmap from a generic staged list into decision gates tied to E0–E6.

For each stage:
- prerequisite;
- engineering evidence required;
- regulatory/infrastructure dependency;
- decision criterion;
- failure condition;
- next action.

Jurong Island may be treated as a candidate industrial context, not a committed nuclear site.

## Phase E8 — Manuscript reintegration
Only after E0–E7 are stable.

Revise the active manuscript so the strongest analysis controls the final report.

Requirements:
- remove the old S$3.725/tCO2e differential screening bridge as the controlling final-report economic result if superseded;
- retain historical screening provenance in repository records;
- replace final-report economics with the strongest defensible model;
- integrate incremental nuclear-benefit attribution;
- integrate Jurong Island candidate-context analysis;
- integrate engineered cooling/siting analysis;
- deepen safety from literature discussion to project-specific screening case;
- update implementation roadmap;
- update CN4252 traceability;
- propagate changed canonical results into abstract/results/discussion/conclusion/figures/tables.

Do not hide adverse findings.

## Phase E9 — Verification and independent review
After manuscript reintegration:
- regenerate deterministic outputs;
- run Rust tests;
- run Research CI;
- run Paper/reproducibility CI;
- build exact PDF;
- inspect every changed quantitative figure/table/equation;
- verify citations and references;
- verify old superseded economics is not presented as controlling;
- verify CN4252 requirements are directly answered.

Then conduct a fresh independent review focused on the new economics, incremental nuclear value, Jurong feasibility/cooling, integrated safety, implementation and final claim strength.

## Stop rule

Each phase is bounded. Main Research must stop at the requested phase boundary unless explicitly instructed to continue.

Historical evidence is preserved; final-report controlling claims must reflect the strongest current evidence.
