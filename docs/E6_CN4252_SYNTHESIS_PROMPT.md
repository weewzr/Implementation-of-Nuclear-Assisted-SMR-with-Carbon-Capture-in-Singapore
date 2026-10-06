# E6 Main Research Prompt — CN4252 Feasibility, Effectiveness and Requirement Synthesis

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md
- docs/ASSIGNMENT_REQUIREMENTS.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- results/E2B_ARCHITECTURE_MATURATION.md
- results/E3_JURONG_SITING_COOLING.md
- results/E4_INTEGRATED_SAFETY_CASE.md
- results/E5_QUANTITATIVE_SAFETY_DEPTH.md
- results/FINAL_ORIGINAL_PLAN_TRACEABILITY.md
- paper/REQUIREMENTS.md
- current manuscript Abstract, Introduction, Results, Discussion/Deployment, Limitations and Conclusions for context only.

E5 is CLOSED at commit `3e305702d375`.

E5 CI:
- Research CI `37498922543` — PASS
- Paper/reproducibility `37498922120` — PASS

This instruction authorises **E6 only — CN4252 Feasibility, Effectiveness and Requirement Synthesis**.

Do not begin E7, W5, E8 manuscript reintegration, final visual generation or independent review automatically.

## Purpose

E0–E5 have generated deeper evidence than the active manuscript currently contains.

E6 must now answer the actual CN4252 assignment as one coherent engineering judgement.

This is NOT another broad research phase.

Do not create new modelling merely to make the answer more favourable.

Use the strongest current evidence and identify contradictions/gaps honestly.

## 1. Preserve the two deployment states

The final project must not collapse current/FOAK and future/mature deployment into one claim.

### Current / FOAK evidence state

Preserve:
- E1 stronger central one-module economics approximately S$154/tCO2e — FAIL;
- E2B two-train FOAK approximately S$137.74/tCO2e — FAIL;
- Singapore has not decided to deploy nuclear;
- Jurong is not an approved nuclear site;
- integrated licensing-level safety is not demonstrated;
- CCS contracts/storage chain are not secured.

Therefore do NOT say the solution is currently commercially/deployment feasible.

### Preferred mature / future deployment state

Preserve the E2B forward-designed configuration:

- one 600 MWth GTHTR300C-class high-temperature reactor;
- two 130 MMSCFD SMR-H2+CCS trains;
- 353.6 MWth useful reformer/process heat;
- ~195,892 tH2/y;
- ~1.834 MtCO2e/y lifecycle abatement on the E2B scaling basis;
- no electricity revenue;
- no heat-sharing/co-product credit;
- medium INL heat-only maturation method;
- early-commercial/BOAK ~S$74.14/tCO2e;
- preferred mature 10-OAK ~S$42.84/tCO2e;
- projected/modelled future economics, NOT observed commercial cost.

This future numerical PASS is conditional on the implementation/feasibility gates established in E3–E5.

## 2. Broad CN4252 assignment spine

Explicitly synthesize the project using the broad assignment structure:

### A. Decarbonisation problem and context
Answer concisely:
- what Singapore problem is being addressed;
- why conventional SMR-H2 emits;
- why CCS helps;
- why high-temperature process heat remains relevant;
- why Singapore/Jurong industrial context matters.

### B. Proposed solution
State the preferred future architecture clearly and simply.

### C. Abatement potential
Show the final preferred future annual abatement and how it compares with >0.25 MtCO2e/y.

### D. Cost estimate
Show:
- FOAK/current adverse value(s);
- preferred mature forward-calculated value;
- evidence maturity;
- why the old S$3.725/t screening bridge is no longer controlling.

### E. Key questions / further work
Use E3–E5 to state precise unresolved deployment questions rather than generic future work.

## 3. Final official requirement matrix

Create a definitive matrix with columns:

Requirement
Official criterion
Current/FOAK evidence
Preferred mature/future evidence
Status
Key condition/limitation
Canonical evidence

Cover at minimum:

- solution-at-scale within Singapore;
- >0.25 MtCO2e/y;
- <S$100/tCO2e;
- clear emissions-abatement mechanism;
- implementation roadmap;
- originality;
- feasibility;
- potential effectiveness;
- content accuracy/evidence quality;
- presentation/readability.

Use statuses only where justified:
PASS
CONDITIONAL PASS
FAIL
SUPPORTED
CONDITIONAL
UNRESOLVED
NOT DEMONSTRATED.

Do not hide a FAIL by calling it conditional.

## 4. Numerical threshold synthesis

State the controlling numerical story cleanly.

### Abatement
Explain:
- E2 conventional SMR+CCS alone already gives ~0.461 MtCO2e/y on its matched-service screen;
- nuclear is not necessary merely to exceed 0.25 Mt/y;
- preferred E2B two-train nuclear-assisted architecture gives ~1.834 MtCO2e/y on its project scaling basis.

Do not mix E2's cross-source comparator denominator and E2B's canonical INL-family scaling without explaining the boundary difference.

### Economics
Explain:
- historical S$3.725/t = superseded differential screening bridge;
- E1 dedicated one-module stronger central = ~S$154/t FAIL;
- E2B two-train FOAK = ~S$137.74/t FAIL;
- E2B early-commercial/BOAK = ~S$74.14/t projected/modelled PASS;
- E2B mature 10-OAK = ~S$42.84/t projected/modelled PASS.

Do not imply S$42.84/t is a current Singapore quotation.

## 5. Why nuclear?

This must be answered honestly because E2 showed CCS alone can meet the minimum abatement target.

Synthesize:
- what CCS alone achieves;
- what nuclear heat changes physically;
- what additional scale/abatement architecture E2B enables;
- how better utilization of the 600 MWth source changes economics;
- what nuclear does NOT uniquely provide;
- whether nuclear is necessary for the assignment threshold;
- why the project still studies it.

Do not claim nuclear is the preferred Singapore pathway unless evidence actually establishes that.

A defensible conclusion may be:
nuclear is not necessary for minimum CN4252 compliance, but a mature high-utilization nuclear-assisted architecture is a conditional larger-scale decarbonisation option if its deployment gates close.

Use evidence, not advocacy.

## 6. Effectiveness synthesis

Potential effectiveness must go beyond threshold arithmetic.

Evaluate:
- H2 output scale;
- lifecycle abatement scale;
- NG dependence;
- fired-heat displacement;
- CCS dependence;
- two-train infrastructure scale;
- robustness to economic maturation;
- sensitivity to unresolved costs;
- H2 offtake requirement;
- CO2 storage requirement;
- availability dependence.

State what would cause real-world effectiveness to fall below modelled effectiveness.

## 7. Singapore / Jurong feasibility synthesis

Use E3.

Explicitly classify:
- industrial integration;
- land/footprint;
- NG;
- H2 offtake;
- CCS logistics;
- cooling;
- water;
- nuclear/chemical separation;
- external hazards;
- security/emergency planning;
- environmental permitting;
- nuclear licensing/site approval.

Preserve:
**Jurong Island = conditional candidate industrial context, not demonstrated nuclear site.**

Explain what "within Singapore" means:
- H2 production and principal industrial integration occur in Singapore;
- emissions are avoided relative to a Singapore industrial counterfactual;
- CO2 is captured/conditioned in Singapore;
- permanent geological storage may be cross-border;
- cross-border storage therefore remains a deployment dependency.

## 8. Safety feasibility synthesis

Use E4/E5.

Do not reduce safety to “TRISO is safe.”

State:

SUPPORTED:
source-technology experimental/operational evidence.

ENGINEERING-SUPPORTED:
barriers, safe states, hazard/propagation architecture.

CONDITIONAL/PARTIAL:
quantitative integrated safety.

NOT DEMONSTRATED:
licensing-level project/site safety.

Summarize the decision-critical unresolved analyses:
- 600 MWth coupled transients;
- PRA/reliability;
- mechanistic source term;
- IHX leak/rupture;
- tritium;
- chemical QRA;
- blast/fire separation;
- Jurong external hazards;
- UHS/SBO;
- site dispersion/dose;
- EPZ/EPR;
- security/regulatory acceptance.

Explain which could still falsify the project.

## 9. Originality

Explicitly test the originality requirement.

The project must not claim originality merely because it uses CCS or hydrogen.

State the project-specific originality:
- high-temperature nuclear process heat replacing fired reformer heat;
- integration with SMR-H2 + amine CCS;
- two-train utilization architecture;
- Singapore/Jurong deployment feasibility;
- integrated nuclear/chemical safety and economics;
- falsifiable FOAK→mature deployment pathway.

Compare conceptually with existing Singapore reforming/CCS initiatives without overstating novelty.

## 10. Accuracy/evidence quality

Summarize why the final result is scientifically bounded:
- source/assumption/project-derived classifications;
- deterministic Rust outputs;
- CI;
- provenance registers;
- explicit model limitations;
- no invented site approval;
- no invented project electricity;
- no invented PRA/QRA/dose;
- historical screening results preserved but superseded where stronger evidence exists.

Identify any important remaining evidence-quality weakness.

## 11. Comparator / decision logic

Create a concise decision comparison:

Conventional SMR
vs
SMR+CCS
vs
preferred mature nuclear-assisted SMR+CCS.

Do not pretend all cost/LCA boundaries are perfectly matched where E2 showed they are not.

The purpose is to show:
- minimum CN4252 compliance;
- additional nuclear ambition;
- additional complexity/conditions.

Do not rank unrelated hydrogen pathways without matched evidence.

## 12. Overall feasibility classification

Produce one final E6 classification for each deployment stage.

For example:

### FOAK/current
NUMERICAL ABATEMENT: ...
ECONOMICS: FAIL
TECHNICAL: ...
SAFETY: ...
SITING: ...
CCS: ...
OVERALL: ...

### Early-commercial/BOAK
NUMERICAL: ...
ECONOMICS: projected PASS
OTHER FEASIBILITY: conditional
OVERALL: ...

### Preferred mature/10-OAK
NUMERICAL: PASS
ECONOMICS: projected PASS
TECHNICAL: conditional
SAFETY: conditional/not demonstrated at licensing level
SITING: conditional
CCS: conditional
OVERALL: CONDITIONAL FUTURE FEASIBILITY / whatever evidence supports.

Do not label overall PASS if major deployment requirements remain unresolved.

## 13. Falsification statement

Explicitly state what the research has falsified.

At minimum consider:
- the original idea that a one-train/under-utilized 600 MWth nuclear-assisted architecture is robustly economical;
- the historical S$3.725/t controlling interpretation;
- the assumption that nuclear is necessary to exceed 0.25 Mt/y;
- any claim that Jurong is already a feasible nuclear site;
- any claim that literature safety evidence proves project safety.

Then state what remains viable:
the preferred future architecture, under measurable maturation and feasibility conditions.

This is scientifically stronger than pretending every original hypothesis survived.

## 14. Implementation-roadmap handoff

E6 must identify the conditions E7 must turn into a staged roadmap.

Create a consolidated gate list from E2B–E5:

- FOAK process-heat demonstration;
- two-train 353.6 MWth architecture proof;
- standardized replication/cost-learning validation;
- early-commercial/mature economic gate;
- 85% availability;
- H2 offtake;
- ≥~1.085 Mt/y CO2 transport/storage;
- Jurong/candidate parcel;
- cooling/heat-rejection closure;
- water/NG infrastructure;
- external hazards;
- nuclear/chemical QRA/separation;
- transient/PRA/source-term/tritium;
- dose/EPR/EPZ;
- security;
- regulatory/licensing framework.

Do not build the full E7 roadmap yet.

## 15. Final-report controlling claims register

Create a register of claims that E8 should use.

For each:
CLAIM
VALUE/STATUS
DEPLOYMENT STAGE
EVIDENCE CLASS
CANONICAL SOURCE
QUALIFICATION.

At minimum include:
- preferred architecture;
- H2 output;
- abatement;
- FOAK cost;
- BOAK cost;
- mature cost;
- CCS-alone finding;
- Jurong status;
- safety status;
- implementation status.

Also create a **superseded claims list** so E8 knows what to remove/demote:
- S$3.725/t as controlling final economics;
- one-train architecture as preferred final design;
- generic qualitative safety language where E4/E5 supersede it;
- generic Jurong feasibility language where E3 supersedes it.

## 16. Do not rewrite the manuscript yet

E6 is the synthesis record.

Do not broadly modify paper sections.

E8 will reintegrate the manuscript after E7.

You may update status/traceability documents needed to preserve E6.

## 17. Durable outputs

Create at minimum:
- results/E6_CN4252_SYNTHESIS.md
- results/e6_synthesis/e6_requirement_matrix.csv
- results/e6_synthesis/e6_deployment_stage_matrix.csv
- results/e6_synthesis/e6_controlling_claims.csv
- results/e6_synthesis/e6_superseded_claims.csv
- results/e6_synthesis/e6_roadmap_handoff.csv

Use Rust only if new project-derived arithmetic is required. Do not create code for qualitative classification merely for appearance.

## 18. Acceptance test

E6 passes only if a marker could read the E6 synthesis and answer:

WHAT PROBLEM IS BEING SOLVED?

WHAT IS THE PROPOSED FINAL ARCHITECTURE?

HOW MUCH CO2e DOES IT ABATE?

WHAT DOES IT COST AT FOAK AND MATURE DEPLOYMENT?

DOES IT MEET >0.25 MtCO2e/y?

DOES IT MEET <S$100/tCO2e?

IS THAT TRUE NOW OR ONLY AT A FUTURE DEPLOYMENT STAGE?

WHY USE NUCLEAR IF CCS ALONE CAN PASS THE ABATEMENT THRESHOLD?

IS JURONG ISLAND ACTUALLY A FEASIBLE NUCLEAR SITE?

WHAT IS KNOWN ABOUT SAFETY?

WHAT IS NOT YET DEMONSTRATED?

WHAT COULD STILL FALSIFY THE PROJECT?

WHAT MUST THE IMPLEMENTATION ROADMAP ACHIEVE?

WHAT ORIGINAL CN4252 REQUIREMENTS ARE MET, CONDITIONAL OR FAILED?

If any answer is obscured by repository complexity, E6 is incomplete.

## 19. Verification

At completion:
- run relevant tests;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1–E5 canonical results remain unchanged;
- verify no old S$3.725 controlling claim is promoted;
- verify S$42.84 is labelled projected mature/10-OAK;
- verify Jurong is not called approved/selected;
- verify safety is not called demonstrated/licensed;
- verify requirement statuses match evidence.

## 20. Report and STOP

Report:

PHASE: E6 — CN4252 Feasibility and Effectiveness Synthesis

STARTING HEAD:

FINAL PROPOSED FUTURE ARCHITECTURE:

DECARBONISATION PROBLEM:

ABATEMENT RESULT:

FOAK ECONOMIC RESULT:

EARLY-COMMERCIAL ECONOMIC RESULT:

MATURE ECONOMIC RESULT:

CCS-ALONE FINDING:

WHY NUCLEAR?:

SINGAPORE/JURONG FEASIBILITY:

SAFETY FEASIBILITY:

ORIGINALITY:

POTENTIAL EFFECTIVENESS:

CURRENT/FOAK OVERALL CLASSIFICATION:

EARLY-COMMERCIAL OVERALL CLASSIFICATION:

MATURE/FUTURE OVERALL CLASSIFICATION:

WHAT HAS BEEN FALSIFIED:

WHAT REMAINS VIABLE:

DECISION-CRITICAL UNRESOLVED ITEMS:

E7 ROADMAP HANDOFF:

CONTROLLING CLAIMS REGISTER CREATED:

SUPERSEDED CLAIMS REGISTER CREATED:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E7 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
