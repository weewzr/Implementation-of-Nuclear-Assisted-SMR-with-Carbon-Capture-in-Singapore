# E7 Main Research Prompt — Decision-Gated Implementation Roadmap

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- results/E6_CN4252_SYNTHESIS.md
- results/e6_synthesis/e6_requirement_matrix.csv
- results/e6_synthesis/e6_deployment_stage_matrix.csv
- results/e6_synthesis/e6_controlling_claims.csv
- results/e6_synthesis/e6_superseded_claims.csv
- results/e6_synthesis/e6_roadmap_handoff.csv
- results/E5_QUANTITATIVE_SAFETY_DEPTH.md
- results/E4_INTEGRATED_SAFETY_CASE.md
- results/E3_JURONG_SITING_COOLING.md
- results/E2B_ARCHITECTURE_MATURATION.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md

The bounded E6 project-identity correction is complete at commit `427da32bfc87`.

Verification:
- Research CI `37500913168` — PASS
- Paper/reproducibility `37500913377` — PASS

This instruction authorises **E7 only — Decision-Gated Implementation Roadmap**.

Do not begin E8 manuscript reintegration, W5, final visual generation or independent review automatically.

## Governing project identity

The proposed CN4252 solution remains:

**NUCLEAR-ASSISTED SMR-H2 + CCS.**

Preferred future architecture:

**1 x 600 MWth GTHTR300C-class high-temperature reactor
→ 353.6 MWth process heat
→ 2 x 130 MMSCFD SMR-H2+CCS trains.**

Conventional SMR+CCS remains the non-nuclear comparator/counterfactual.

Do not redesign the project into conventional CCS.

## Purpose

E7 must answer the official CN4252 requirement:

**What would implementation actually look like?**

The roadmap must not be a generic:

research → pilot → build → operate.

It must be a sequence of **measurable decision gates** derived from E1–E6.

For each stage identify:

PREREQUISITE
→ WORK TO PERFORM
→ MEASURABLE OUTPUT
→ ACCEPTANCE / GO-NO-GO CRITERION
→ FAILURE CONSEQUENCE
→ NEXT STAGE.

The roadmap must distinguish:
- technology maturation;
- economic maturation;
- Singapore national/regulatory readiness;
- candidate-site/Jurong feasibility;
- nuclear/process safety;
- chemical safety;
- infrastructure/offtake;
- CCS;
- licensing;
- construction;
- operation.

Do not assign arbitrary dates merely to make a timeline look complete.

## 1. Deployment-stage hierarchy

Build the roadmap around evidence-supported stages such as:

### Stage 0 — Current research / pre-deployment
Current state:
- nuclear deployment decision absent;
- project is a screening/research architecture;
- FOAK economics fail;
- licensing-level safety not demonstrated.

### Stage 1 — Component / integration R&D
Close:
- high-temperature IHX/process-heat qualification;
- secondary-helium system;
- reformer integration;
- tritium;
- controls/load-following;
- process-heat transient behaviour.

### Stage 2 — FOAK demonstration
Demonstrate a real nuclear high-temperature process-heat architecture at relevant scale/maturity.

FOAK may remain >S$100/tCO2e.

### Stage 3 — Two-train architecture demonstration / first commercial integration
Demonstrate:
- 353.6 MWth useful process heat;
- two H2 trains;
- required temperatures;
- availability;
- CCS;
- safe load rejection/trips.

### Stage 4 — Replication / early-commercial BOAK
Test whether real cost/construction/O&M performance approaches the E2B medium BOAK basis.

### Stage 5 — Mature / 10-OAK threshold gate
Test whether actual replicated performance supports the projected mature basis and remains <S$100/tCO2e after unresolved site/integration costs.

### Stage 6 — Singapore project development/licensing
Only after national/regulatory/site/infrastructure gates are sufficiently mature.

### Stage 7 — Construction/commissioning
Only after licensing, contracts, financing and site safety closure.

### Stage 8 — Commercial operation / verification
Measure actual H2, capture, availability, lifecycle emissions and cost.

Modify stage structure if evidence supports a better sequence, but preserve the decision-gated logic.

## 2. Singapore national nuclear-readiness gate

Use current Singapore evidence.

Define what must exist before project development:
- national decision to consider/deploy nuclear;
- competent independent regulator;
- power-reactor legal/licensing framework;
- IAEA milestone/INIR progression as applicable;
- safeguards/security framework;
- radioactive waste/spent-fuel policy;
- emergency-preparedness framework;
- liability/insurance arrangements;
- qualified workforce/supply chain.

Do not imply current radiation regulation is already a complete reactor licensing regime.

No arbitrary date.

## 3. Technology qualification gate

Translate E4/E5 into technology requirements.

At minimum:
- selected reactor/fuel basis;
- 600 MWth coupled transient model;
- process-load-loss response;
- decay-heat/UHS closure;
- IHX structural/thermal qualification;
- secondary-helium loop;
- tritium/permeation;
- controls/isolation;
- reformer heat-transfer qualification;
- material compatibility;
- maintenance/inspection.

For each specify measurable evidence required.

## 4. Safety gate

Build explicit go/no-go criteria from E4/E5.

Required analyses include:
- PRA;
- mechanistic source term;
- IHX leak/rupture;
- tritium;
- chemical HAZOP/QRA;
- blast/fire/toxic consequences;
- nuclear↔chemical propagation;
- common-cause hazards;
- UHS/SBO;
- site dispersion/dose;
- EPR/EPZ;
- security.

Do not invent numerical acceptance thresholds where Singapore has not established them.

Use:
**must satisfy applicable future Singapore regulator + IAEA/design-basis requirements**
where a numerical project criterion cannot yet be stated.

Failure can require:
REDESIGN
RELOCATE
REDUCE SCALE
ADD BARRIERS
or
STOP.

## 5. Candidate-site/Jurong gate

Jurong Island remains a candidate industrial context, not the selected site.

Define required site work:
- parcel identification;
- geotechnical/seismic;
- coastal/flood/storm-surge;
- neighbouring industrial hazards;
- marine/shipping;
- land/footprint;
- nuclear/chemical separation;
- security/protected area;
- construction logistics;
- emergency access;
- cooling intake/outfall or alternative;
- environmental/radiological baseline.

Go criterion:
candidate parcel passes regulator/site evaluation and integrated QRA/PRA/environmental requirements.

No-go:
site cannot provide adequate separation, heat sink, hazard protection, security/EPR or environmental acceptability.

If Jurong fails, roadmap should allow evaluation of another Singapore context rather than automatically killing the technology concept.

## 6. Cooling/water gate

Use E3.

Required:
- complete reactor/process/CCS heat balance;
- normal heat-rejection duty;
- decay-heat/UHS duty;
- selected cooling architecture;
- intake/outfall/tower design;
- marine thermal-plume/ecology;
- blockage/fouling;
- corrosion;
- flood/coastal resilience;
- process/makeup water balance.

Do not use 246.4 or 600 MWth as canonical cooling duty.

Go criterion:
normal and safety heat sinks close with acceptable environmental/safety margins.

## 7. H2 market/offtake gate

Preferred architecture produces:
- 260 MMSCFD H2;
- ~195,892 tH2/y.

Roadmap must require:
- identified demand/offtake;
- pressure/purity requirements;
- pipeline/storage/buffer;
- turndown/curtailment strategy;
- contractual/market basis.

Do not assume all produced H2 has a customer.

Go criterion:
credible/contracted offtake sufficient for the planned production scale.

## 8. NG/feedstock gate

Preferred architecture requires:
- 68 MMSCFD operating NG;
- ~21.1 Bscf/y at 85% availability.

Require:
- firm capacity;
- pressure/connection;
- redundancy;
- supply resilience;
- lifecycle/upstream performance.

The nuclear-assisted pathway remains methane-dependent.

## 9. CCS gate

Preferred architecture captures approximately:
- 1.085 MtCO2/y.

Require:
- capture-system qualification;
- conditioning specification;
- buffer/storage;
- transport mode;
- terminal/pipeline/shipping;
- contracted cross-border transport;
- permanent storage capacity;
- monitoring/accounting;
- liability;
- regulatory recognition of abatement.

Go criterion:
credible contracted chain for at least project-scale CO2 with cost/lifecycle burden compatible with the economic case.

Do not treat international MOUs as storage contracts.

## 10. Economic maturation gate

Use E1/E2B.

Preserve:
- one-module E1 ~S$154/t — FAIL;
- two-train FOAK ~S$137.74/t — FAIL;
- BOAK ~S$74.14/t — projected/modelled PASS;
- 10-OAK ~S$42.84/t — projected/modelled PASS.

Roadmap must require measured cost evidence rather than assuming learning.

At each replication stage update:
- OCC/CAPEX;
- O&M;
- construction duration;
- availability;
- integration costs;
- CCS/T&S;
- site/licensing/security costs;
- financing if project-level economics are developed.

Go criterion for CN4252 mature deployment:
forward-calculated total represented cost remains <S$100/tCO2e after incorporating then-known project costs.

The roadmap should identify the E2B remaining margin as a tolerance, not free money.

## 11. Availability/reliability gate

The model uses 85% availability.

Require operational evidence that:
- reactor;
- IHX/secondary loop;
- both H2 trains;
- CCS;
- CO2 transport/storage

can jointly support the required annual service.

Do not equate reactor availability alone with integrated-plant availability.

If integrated availability is lower, recalculate H2, abatement and economics.

## 12. Two-train architecture gate

This is central.

Demonstrate:
- 353.6 MWth process-heat delivery;
- ~370 MW source branch not exceeded;
- required temperature approach;
- load sharing;
- single-train trip;
- simultaneous two-train trip;
- safe reactor response;
- turndown;
- restart;
- CCS interaction;
- H2 quality.

A two-train architecture must be demonstrated physically; it is not merely an economic allocation device.

## 13. FOAK→BOAK→NOAK learning validation

E2B uses projected/modelled maturation.

At each replicated build:
- record actual overnight cost;
- construction time;
- labour/productivity;
- supply-chain maturity;
- O&M;
- availability;
- integration cost;
- learning achieved.

Compare actual learning with INL/DOE source expectations.

If observed maturation is insufficient to reach the economic gate, do not advance merely because the roadmap expected it.

## 14. Financing / commercial gate

Where project development becomes real, require:
- project CAPEX/TCI;
- financing;
- construction schedule;
- interest during construction;
- insurance/liability;
- decommissioning/waste;
- revenue/offtake;
- contractual CCS;
- owner/EPC costs.

Do not pretend the current screening model is bankable finance.

This gate may occur after technical maturity but before final investment decision.

## 15. Environmental / social / emergency gate

Require applicable assessment of:
- marine/ecological effects;
- thermal discharge;
- radiological environmental impact;
- industrial emissions;
- emergency planning;
- public/worker protection;
- transport;
- waste;
- relevant consultation/approval processes.

Do not make claims about public acceptance without evidence.

## 16. Final investment / licensing gate

Before construction require:
- national policy allows project;
- site approved;
- design licensed;
- safety case accepted;
- environmental approvals;
- H2 offtake;
- NG;
- CCS;
- cooling/water;
- security/EPR;
- financing/contracts;
- economics still satisfy the project's decision criterion.

If any binding gate fails, project is redesigned, delayed, relocated or stopped.

## 17. Commissioning / operational verification

After construction define measurable commissioning/operation verification:
- reactor thermal performance;
- IHX;
- two-train process heat;
- H2 production;
- CCS capture;
- CO2 delivery/storage;
- emissions;
- availability;
- safety systems;
- actual operating cost.

Only measured operation can convert projected mature performance into demonstrated performance.

## 18. Roadmap timelines

Do not invent precise calendar years.

Use maturity/order rather than fake schedules unless an authoritative programme date exists.

Where current dates are known (e.g. Singapore INIR work), cite them as context, not guaranteed project milestones.

If useful, give broad phase durations only where supported by analogous evidence and clearly labelled indicative.

## 19. Decision-gate matrix

Create a machine-readable matrix with columns such as:

Gate ID
Stage
Prerequisite
Work
Measurable output
Go criterion
No-go/failure condition
Response if failed
Evidence source
Current status
CN4252 relevance.

Every major unresolved E6 item must appear in the roadmap.

## 20. Critical path / dependencies

Identify logical dependencies.

Examples:
- site selection depends on regulatory framework and hazard survey;
- final separation depends on QRA/PRA;
- EPR/EPZ depends on source term + site dispersion;
- final cooling depends on selected site and plant heat balance;
- project finance depends on licensed design/site/contracts;
- mature economics depend on actual replication learning.

Do not present all workstreams as independent parallel boxes.

## 21. What can proceed in parallel?

Also identify legitimate parallel work:
- reactor/component R&D;
- CCS commercial development;
- H2 market/offtake study;
- regulatory capability building;
- site survey;
- safety modelling;
- supply-chain development.

This makes the roadmap practical without inventing dates.

## 22. Roadmap falsification logic

The roadmap must include explicit STOP/REDESIGN conditions.

Examples:
- no feasible nuclear/chemical separation;
- unacceptable site external hazards;
- no credible UHS;
- mechanistic source term/dose incompatible with site;
- no project-scale CCS chain;
- no H2 offtake;
- integrated availability too low;
- mature economics remain >S$100/t after real project costs;
- regulator rejects design/site.

The implementation roadmap is a falsification pathway, not a promise that every stage succeeds.

## 23. Relationship to originality

Preserve project identity.

The roadmap is for deployment of the **nuclear-assisted** concept.

Conventional SMR+CCS remains the comparator.

Do not turn the implementation roadmap into a conventional CCS project because that comparator is easier to deploy.

However, the roadmap may include a decision point where nuclear is rejected if its incremental value cannot justify its complexity.

## 24. Final E7 roadmap summary

Produce a concise submission-facing roadmap skeleton that E8 can later insert into the manuscript.

It should be understandable in one page/table/figure and show:

CURRENT RESEARCH
→ COMPONENT/INTEGRATION QUALIFICATION
→ FOAK
→ TWO-TRAIN DEMONSTRATION
→ REPLICATION / BOAK
→ MATURE COST GATE
→ SINGAPORE SITE/LICENSING/CONTRACT CLOSURE
→ CONSTRUCTION/COMMISSIONING
→ OPERATIONAL VERIFICATION.

This is a logical roadmap, not a calendar promise.

## 25. Later visual specification

Do not create final polished artwork yet.

Create:
- results/e7_roadmap/E7_LATER_VISUAL_SPECIFICATION.md

Specify a final roadmap visual for the user's external/design workflow.

Quantitative gate labels and statuses should come from deterministic data/text, not be hallucinated by image generation.

## 26. Durable outputs

Create at minimum:
- results/E7_IMPLEMENTATION_ROADMAP.md
- results/e7_roadmap/e7_gate_matrix.csv
- results/e7_roadmap/e7_dependencies.csv
- results/e7_roadmap/e7_parallel_workstreams.csv
- results/e7_roadmap/e7_stop_conditions.csv
- results/e7_roadmap/E7_LATER_VISUAL_SPECIFICATION.md

No Rust code is required unless E7 introduces genuine project-derived arithmetic. Do not code qualitative gates for appearance.

## 27. Acceptance test

E7 passes only if a marker can answer:

WHAT HAPPENS FIRST?

WHAT CAN HAPPEN IN PARALLEL?

WHAT MUST BE DEMONSTRATED BEFORE FOAK?

WHAT MUST FOAK DEMONSTRATE?

HOW DOES THE PROJECT MOVE FROM FOAK >S$100/t TO A PROJECTED PASS?

WHAT MUST BE TRUE FOR THE TWO-TRAIN ARCHITECTURE?

WHEN IS JURONG/SITE SELECTION POSSIBLE?

WHEN ARE PRA/QRA/SOURCE TERM REQUIRED?

WHEN MUST H2 AND CCS CONTRACTS EXIST?

WHAT EXACTLY CAUSES THE PROJECT TO STOP OR REDESIGN?

WHEN CAN THE S$42.84/t MATURE CASE BE TREATED AS MORE THAN A MODELLED TARGET?

If the roadmap is merely a list of activities without decision criteria, E7 is incomplete.

## 28. Verification

At completion:
- run relevant checks;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1–E6 numerical results remain unchanged;
- verify nuclear-assisted project identity;
- verify no invented calendar dates;
- verify every major unresolved E6 issue maps to a gate;
- verify STOP/REDESIGN conditions exist.

## 29. Report and STOP

Report:

PHASE: E7 — Decision-Gated Implementation Roadmap

STARTING HEAD:

ROADMAP STAGES:

CURRENT/PRE-DEPLOYMENT GATE:

TECHNOLOGY QUALIFICATION GATE:

FOAK GATE:

TWO-TRAIN ARCHITECTURE GATE:

BOAK/REPLICATION GATE:

MATURE ECONOMIC GATE:

SINGAPORE NATIONAL/REGULATORY GATE:

JURONG/SITE GATE:

SAFETY GATE:

COOLING/WATER GATE:

H2 OFFTAKE GATE:

NG GATE:

CCS GATE:

AVAILABILITY GATE:

FINANCING/COMMERCIAL GATE:

ENVIRONMENT/EPR GATE:

FINAL INVESTMENT/LICENSING GATE:

COMMISSIONING/OPERATION GATE:

CRITICAL PATH:

PARALLEL WORKSTREAMS:

STOP/REDESIGN CONDITIONS:

SUBMISSION-FACING ROADMAP SKELETON:

VISUAL SPECIFICATION CREATED:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E8 automatically.
Do not begin W5.
Do not begin final visual generation.
Do not begin independent review.
