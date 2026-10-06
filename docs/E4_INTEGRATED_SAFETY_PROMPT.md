# E4 Main Research Prompt — Integrated Nuclear + Chemical Safety Case

## Authority and scope

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- results/E3_JURONG_SITING_COOLING.md
- results/E2B_ARCHITECTURE_MATURATION.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md
- current nuclear-feasibility/safety evidence matrices and manuscript safety/deployment sections.

E3 is CLOSED at commit `9d88a34f4a9d`.

E3 CI:
- Research CI `37439295391` — PASS
- Paper/reproducibility `37439295371` — PASS

This instruction authorises **E4 only — Integrated Nuclear + Chemical Safety Case**.

Do not begin E5, W5, E8, final visual generation or independent review automatically.

## Governing architecture

Evaluate the preferred future E2B/E3 architecture:
- one 600 MWth GTHTR300C-class high-temperature reactor;
- two 130 MMSCFD SMR-H2+CCS trains;
- 353.6 MWth total process heat;
- primary helium → IHX → secondary helium → reformers;
- ~195,892 tH2/y;
- ~1.085 MtCO2/y captured;
- Jurong Island only as a conditional candidate industrial context.

E4 must not merely say “HTGR/TRISO is inherently safe.” It must construct a **project-specific screening safety case** for nuclear + hydrogen + methane + CO + amine CCS + high-temperature helium + Jurong industrial co-location.

This is not a licensing PRA/QRA. E5 will define/attempt the quantitative PRA/source-term/QRA depth. E4 must establish the hazard logic, barriers, propagation paths, safe states and evidence maturity rigorously enough that E5 has a concrete basis.

## 1. Evidence standard

Use current primary/authoritative sources where needed.

Prioritise:
- JAEA/JAERI HTTR and GTHTR300C;
- IAEA safety standards/guides;
- NRC advanced/non-LWR/TRISO/source-term guidance;
- OECD-NEA;
- INL/ORNL/DOE technical reports;
- peer-reviewed HTGR safety literature;
- Singapore NEA/SCDF/MOM/JTC/EMA requirements for hazardous installations/QRA/process safety where applicable.

Distinguish:
DEMONSTRATED / EXPERIMENTAL / SOURCE DESIGN / MODELLED / REGULATORY / PROJECT SCREENING / UNRESOLVED.

Do not transfer HTTR/AGR/other-reactor evidence to the 600 MWth project without stating transferability limitations.

## 2. Safety objectives and boundaries

Define the safety objectives separately for:
- nuclear reactor/core;
- primary helium pressure boundary;
- IHX;
- secondary helium/process-heat loop;
- reformers and chemical train;
- H2 product systems;
- NG systems;
- CO/WGS systems;
- amine CCS;
- CO2 conditioning/compression;
- cooling/ultimate heat sink;
- electrical/control/support systems;
- shared/common utilities;
- external Jurong hazards.

Define what constitutes a safe state for major initiating-event families.

Do not assume that shutting down the chemical plant automatically places the reactor in a safe state, or vice versa.

## 3. Defence-in-depth / barrier map

Construct an explicit barrier/defence-in-depth map.

At minimum cover:
- TRISO fuel particle barriers;
- graphite/fuel compact/core retention functions;
- reactor pressure boundary;
- confinement/building functions as applicable to source design;
- primary helium boundary;
- IHX pressure/heat-transfer boundary;
- secondary helium boundary;
- physical separation to chemical process;
- chemical containment/isolation;
- detection/trip/isolation systems;
- decay/residual heat removal;
- emergency/backup power and ultimate heat sink concept;
- physical separation/security.

For each barrier state:
FUNCTION → HAZARD IT PREVENTS/MITIGATES → EVIDENCE → FAILURE CONSEQUENCE → NEXT BARRIER.

Do not claim a containment function that the selected source architecture does not actually possess.

## 4. Nuclear initiating events

Develop structured event narratives for at least:

### N1 Reactor trip / loss of process heat demand
Two reformers trip or rapidly reduce heat demand while reactor heat generation/decay heat persists.

Analyse:
- immediate thermal mismatch;
- reactor trip/control response;
- primary/secondary helium response;
- where residual heat goes;
- need for bypass/dump/alternate heat sink;
- safe-state requirement.

This event is especially important because E3 showed remaining capacity is not automatically a heat sink.

### N2 Loss of forced cooling / helium circulation
Use HTTR demonstrated evidence where applicable.

Distinguish:
- demonstrated HTTR behaviour;
- scaling/geometry differences;
- project-specific unresolved transient.

### N3 Primary-helium depressurisation
Analyse:
- initiating causes;
- core cooling implications;
- confinement/source-term implications;
- IHX/process-side effects.

### N4 Air ingress
Analyse oxidation/graphite/fuel implications and source evidence.

### N5 Water/steam ingress
Analyse reactivity/graphite/pressure/chemical effects where applicable.

### N6 Reactivity events / control failure
Treat at the appropriate screening level with source evidence.

### N7 Loss of ultimate heat sink / station blackout
Connect explicitly to E3 cooling/siting work.

Do not invent reliability/frequency numbers.

## 5. IHX and nuclear/process interface events

At minimum analyse:

### I1 IHX tube/wall leak
Possible primary→secondary transfer and/or secondary→primary transfer depending on pressure state.

### I2 IHX rupture / major boundary failure
Assess:
- pressure interaction;
- helium inventory loss;
- process-heat loss;
- radiological transfer pathway;
- isolation.

### I3 Loss of secondary-helium circulation
Assess reactor/process thermal response and reformer safe shutdown.

### I4 Secondary-helium depressurisation
Assess loss of heat transfer and interaction with chemical side.

### I5 Excess process heat / control failure
Assess high-temperature chemical equipment consequences.

### I6 Tritium/permeation
Map:
source → transport through primary circuit/IHX → secondary loop → possible process interaction.

Use quantitative literature values only where transferable and locally cited.

Do not claim zero radiological transfer merely because an IHX exists.

## 6. Chemical/process hazards

Construct explicit event narratives for:

### C1 Hydrogen release
- jet fire;
- flash fire;
- explosion/deflagration;
- confined accumulation;
- ignition sources;
- escalation toward nuclear systems.

### C2 Natural-gas/methane release
Same propagation logic.

### C3 CO release
Toxic exposure and process isolation implications.

### C4 Reformer fire / furnace/process-heater event
Even though nuclear heat replaces fired reformer duty, analyse high-temperature process inventory, NG/H2 release and external fire.

### C5 High-pressure steam/process rupture
Assess missiles, thermal effects, common-cause impact.

### C6 PSA/tail-gas event
Use only source-supported process topology; do not invent detailed bed states.

### C7 Amine/CCS hazards
Assess solvent release/degradation, reboiler/process hazards where relevant.

### C8 CO2 release
High-pressure release, dry ice/low temperature if relevant, asphyxiation, dispersion, confined-area hazards.

### C9 CO2 compressor/conditioning event
At screening level, without invented compressor design.

## 7. Nuclear → chemical propagation

Create an explicit propagation matrix.

Examples:
- reactor trip → loss of reformer heat → chemical shutdown;
- secondary-loop failure → process temperature excursion;
- primary depressurisation/IHX leakage → radiological classification of secondary/process systems;
- nuclear-site fire/failure → common utilities lost to chemical trains;
- emergency reactor heat rejection → impact on shared cooling infrastructure.

For each:
INITIATOR → COUPLING PATH → CHEMICAL CONSEQUENCE → BARRIER → SAFE STATE → UNRESOLVED ANALYSIS.

## 8. Chemical → nuclear propagation

This is equally important.

At minimum evaluate:
- H2 explosion blast;
- NG explosion/fire;
- high-pressure vessel/projectile;
- toxic/corrosive release;
- CO2 cloud/asphyxiation effects on personnel/access;
- chemical fire thermal radiation;
- shared utility loss;
- cooling-water/common intake loss;
- electrical disturbance;
- emergency-access obstruction.

For each assess whether physical separation, barriers, independent utilities or isolation are required.

Do not assign a definitive separation distance yet unless supported by an applicable calculation/source.

## 9. Common-cause / shared-system hazards

Identify shared dependencies that could defeat both nuclear and chemical safety functions:
- grid loss;
- seawater intake blockage;
- cooling-system failure;
- flooding;
- firewater/common utilities;
- control/communications;
- access;
- extreme weather;
- port/marine event;
- industrial-area fire/explosion;
- malicious/security event at high level.

Explicitly identify where independence/diversity is required.

## 10. Jurong external-hazard integration

Use E3's Jurong context.

Construct an external-hazard register covering at least:
- neighbouring chemical/refinery fire/explosion;
- hazardous/toxic gas release;
- pipelines/storage;
- shipping/port collision/explosion;
- dangerous-goods transport;
- coastal flooding/storm surge;
- extreme rainfall/compound flooding;
- sea-level rise;
- intake blockage/fouling;
- seismic/geotechnical basis;
- aircraft/transport where applicable;
- malicious/security considerations.

Do not claim these hazards are acceptable merely because Jurong is industrial.

## 11. Safe-state logic

For each major event family define the required safe-state functions.

Examples:
NUCLEAR:
- terminate fission;
- maintain fuel temperature within safety limits;
- remove decay heat;
- retain radionuclides;
- maintain/restore cooling;
- isolate failed boundaries.

CHEMICAL:
- isolate NG/H2/CO inventories;
- terminate feed;
- depressurise safely where appropriate;
- stop reaction/heat input;
- prevent ignition/escalation;
- maintain safe vent/flare/disposal where applicable.

INTEGRATED:
- prevent propagation between nuclear and chemical areas;
- maintain independent emergency power/cooling;
- preserve access/control;
- prevent one plant's emergency action from disabling the other's safe state.

Do not invent detailed control logic unavailable in source design.

## 12. Safety classification / evidence matrix

For every important safety claim classify:
- demonstrated experimentally;
- demonstrated operationally;
- source design/model;
- project-derived screening;
- regulatory requirement;
- unresolved.

Examples that require careful classification:
- HTTR LOFC behaviour;
- TRISO high-temperature retention;
- passive heat removal;
- GTHTR300C scale-up;
- IHX isolation;
- tritium;
- source term;
- EPZ;
- blast separation;
- common-cause safety.

## 13. Quantitative calculations allowed in E4

Perform bounded quantitative screening where inputs are defensible, for example:
- energy/inventory scales;
- thermal mismatch magnitudes;
- source-supported TRISO temperature margins;
- blast/thermal screening only if an accepted method and inventory are available;
- separation/source design evidence;
- helium inventories only if supported.

Do not fabricate:
- core-damage frequency;
- event frequencies;
- source-term release fractions;
- off-site dose;
- EPZ;
- QRA risk contours.

Those require E5-level data/methods.

## 14. Hazard register

Create a machine-readable hazard register with fields such as:

ID
System
Initiating event
Hazard
Immediate consequence
Propagation path
Preventive barrier
Mitigating barrier
Safe-state function
Evidence
Evidence maturity
Quantitative analysis available?
E5 calculation required?
Singapore/Jurong relevance
Status.

This register should become the backbone for E5.

## 15. FMEA / bow-tie / event-sequence representation

Use a structured representation appropriate to the evidence.

At minimum create one or more of:
- FMEA-style table;
- bow-tie logic;
- event-sequence trees at screening level.

Do not assign probabilities unless sourced/applicable.

Focus on causal structure.

## 16. Safety-critical unresolved questions

E4 must finish with a precise list of what prevents a project safety claim.

At minimum assess whether unresolved:
- mechanistic source term;
- PRA;
- QRA;
- project-specific transients;
- IHX qualification;
- tritium;
- nuclear/chemical blast/separation;
- ultimate heat sink;
- site meteorology/dispersion;
- emergency planning;
- security;
- regulatory acceptance.

Replace generic "future work" with:
REQUIRED INPUT → REQUIRED METHOD → REQUIRED OUTPUT → DECISION IT ENABLES.

## 17. CN4252 feasibility answer

Explicitly answer:

What can the project currently claim about safety?

A useful hierarchy is:

SUPPORTED:
source technology has relevant demonstrated/experimental safety evidence.

ENGINEERING-SUPPORTED:
the project architecture contains identifiable barriers and safe-state concepts.

CONDITIONAL:
integration appears plausible only if specified analyses/designs close.

NOT DEMONSTRATED:
licensing-level integrated safety for Jurong.

Do not claim "safe" as a binary conclusion.

Explain how this affects the CN4252 feasibility criterion and implementation roadmap.

## 18. Later visual specification

Do not generate final polished images.

Create a later visual specification for the user's external image-generation/editing workflow.

Likely final safety figure should show:
- nuclear island;
- primary helium;
- IHX;
- secondary helium;
- two chemical trains;
- physical separation;
- barriers;
- nuclear→chemical propagation arrows;
- chemical→nuclear propagation arrows;
- independent cooling/power/safety functions;
- conditional/not-to-scale status.

Also identify which safety visuals should remain deterministic technical diagrams rather than AI artwork.

## 19. Durable outputs

Create at minimum:
- results/E4_INTEGRATED_SAFETY_CASE.md
- results/e4_safety/e4_hazard_register.csv
- results/e4_safety/e4_barrier_matrix.csv
- results/e4_safety/e4_propagation_matrix.csv
- results/e4_safety/e4_safe_state_matrix.csv
- results/e4_safety/e4_evidence_matrix.csv
- results/e4_safety/E4_LATER_VISUAL_SPECIFICATION.md

Add deterministic Rust calculations/tests only where useful quantitative screening is performed.

Do not create fake precision merely to use Rust.

## 20. Acceptance test

E4 passes only if a technically trained reader can answer:

WHAT CAN GO WRONG ON THE NUCLEAR SIDE?

WHAT CAN GO WRONG ON THE CHEMICAL SIDE?

HOW CAN EACH SIDE AFFECT THE OTHER?

WHAT BARRIERS PREVENT PROPAGATION?

WHAT SAFE STATE IS REQUIRED?

WHAT HAS ACTUALLY BEEN DEMONSTRATED?

WHAT IS ONLY MODELLED/DESIGNED?

WHAT PROJECT-SPECIFIC ANALYSIS IS STILL MISSING?

WHAT JURONG-SPECIFIC EXTERNAL HAZARDS MATTER?

WHAT EXACTLY MUST E5 QUANTIFY?

If the answer remains merely “HTGRs are passively safe and separation is required,” E4 is incomplete.

## 21. Verification

At completion:
- run relevant tests;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1/E2/E2B/E3 outputs remain unchanged;
- verify no invented risk frequencies/doses/EPZ;
- verify safety claims have evidence maturity;
- verify every major hazard has a barrier/safe-state disposition or explicit unresolved status.

## 22. Report and STOP

Report:

PHASE: E4 — Integrated Nuclear + Chemical Safety Case

STARTING HEAD:

NUCLEAR INITIATING EVENTS:

IHX/INTERFACE EVENTS:

CHEMICAL HAZARDS:

NUCLEAR→CHEMICAL PROPAGATION:

CHEMICAL→NUCLEAR PROPAGATION:

COMMON-CAUSE HAZARDS:

JURONG EXTERNAL HAZARDS:

DEFENCE-IN-DEPTH/BARRIERS:

SAFE-STATE LOGIC:

DEMONSTRATED SAFETY EVIDENCE:

SOURCE-DESIGN/MODELLED EVIDENCE:

PROJECT-SCREENING EVIDENCE:

QUANTITATIVE SCREENS PERFORMED:

PRA/QRA/SOURCE-TERM ITEMS DELIBERATELY NOT INVENTED:

CRITICAL UNRESOLVED SAFETY QUESTIONS:

CN4252 SAFETY/FEASIBILITY CLASSIFICATION:

E5 REQUIRED CALCULATIONS/DATA:

VISUAL SPECIFICATION CREATED:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E5 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
