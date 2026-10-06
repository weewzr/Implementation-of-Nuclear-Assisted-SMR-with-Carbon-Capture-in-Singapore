# E5 Main Research Prompt — Quantitative Safety Depth: PRA, Source Term, QRA and Consequence Requirements

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- results/E4_INTEGRATED_SAFETY_CASE.md
- results/e4_safety/e4_hazard_register.csv
- results/e4_safety/e4_barrier_matrix.csv
- results/e4_safety/e4_propagation_matrix.csv
- results/e4_safety/e4_safe_state_matrix.csv
- results/e4_safety/e4_evidence_matrix.csv
- results/E3_JURONG_SITING_COOLING.md
- results/E2B_ARCHITECTURE_MATURATION.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md

E4 is CLOSED at commit `eceac1284456`.

E4 CI:
- Research CI `37489807632` — PASS
- Paper/reproducibility `37489807544` — PASS

Execute **E5 only**.

Do not begin E6, W5, E8, final visual generation or independent review automatically.

## Purpose

E4 established causal safety structure. E5 must determine how far the project can move from qualitative hazard logic toward **quantitative safety evidence** without inventing frequencies, inventories, source terms, meteorology, geometry or dose results.

The goal is not to manufacture a licensing PRA/QRA.

The goal is:

1. perform quantitative screening where inputs and methods are defensible;
2. construct screening event/fault-tree logic where probabilities are unavailable;
3. determine whether existing source evidence can bound any consequences;
4. specify exactly what data/model is required for calculations that cannot yet be performed;
5. convert vague “future safety work” into a quantitative safety-analysis programme and implementation gate.

## 1. Evidence research

Use primary/authoritative current evidence where required.

Prioritise:
- JAEA/JAERI HTTR/GTHTR300C;
- NRC advanced/non-LWR guidance, mechanistic source-term guidance and HTGR/TRISO PIRTs;
- IAEA PRA, source-term, siting, consequence and emergency-preparedness standards;
- INL/ORNL/DOE;
- peer-reviewed HTGR accident/source-term literature;
- Singapore MOM/NEA/SCDF QRA/MHI requirements;
- authoritative Singapore meteorological/population/environmental data only where applicable.

Do not import LWR source terms or frequencies into HTGR calculations without explicit justification.

## 2. PRA screening structure

For the selected project architecture construct screening event trees for the dominant E4 nuclear/interface initiators, at minimum:

- loss of process heat demand / simultaneous two-train trip;
- loss of forced primary circulation;
- primary depressurisation;
- air ingress;
- water/steam ingress;
- loss of ultimate heat sink;
- station blackout/loss of electrical support;
- IHX leak/rupture;
- loss of secondary-helium circulation;
- relevant external/common-cause event.

For each event tree identify:
INITIATOR → protection/trip → shutdown success/failure → decay-heat-removal success/failure → isolation success/failure → barrier state → release/propagation endpoint.

Do not assign branch probabilities unless applicable data exist.

## 3. Fault-tree/data requirement

For safety functions such as:
- reactor trip;
- shutdown;
- decay-heat removal;
- emergency/backup power;
- ultimate heat sink;
- IHX isolation;
- secondary-loop isolation;
- chemical feed isolation;

identify the component/system reliability data required to quantify failure probability.

Where applicable generic data exist, determine whether they are transferable. Do not use generic values merely to produce a number.

Create a reliability-data gap matrix.

## 4. Mechanistic source-term pathway

Build the project-specific mechanistic source-term calculation chain:

core radionuclide inventory
→ TRISO release by temperature/time/failure mode
→ graphite/core retention
→ primary-circuit transport/deposition
→ pressure-boundary/confinement behaviour
→ leakage/release pathway
→ environmental release
→ atmospheric dispersion
→ dose.

For each stage identify:
- required input;
- available source evidence;
- project-specific missing input;
- model/method required;
- whether any bounded calculation is currently defensible.

Do not assign a project release fraction from AGR particle tests alone.

Do not use an LWR source term.

## 5. Core inventory / radionuclide screen

Determine whether a defensible order-of-magnitude radionuclide inventory can be calculated from:
- 600 MWth rating;
- fuel/burnup/source-design data;
- operating history assumptions;
- established inventory methods.

Only perform this if sufficient source inputs exist.

If not, state the exact missing fuel/burnup/power-history data and the appropriate depletion/inventory method (e.g. ORIGEN-class analysis) required.

Do not fabricate inventory.

## 6. TRISO/fuel temperature and release

Use HTTR/AGR/HTGR evidence to define what is demonstrated about:
- fuel temperature behaviour;
- TRISO retention;
- failure mechanisms;
- high-temperature safety-test response.

Determine whether any project transient temperature can be bounded.

If the project lacks a coupled 600 MWth thermal-hydraulic transient, do not infer project TRISO release from source experiments.

Specify the required transient model and outputs.

## 7. IHX radiological transfer / tritium

Attempt the strongest defensible quantitative treatment of:
- primary-to-secondary leak pathway;
- tritium generation/transport/permeation;
- purification/removal;
- transfer toward process systems/product.

Use actual source values only where transferable.

If project geometry/material/pressure/purification data are insufficient, build the governing mass-transfer/transport equations and identify missing parameters rather than inserting guesses.

Output the conditions required to classify the secondary/process side radiologically.

## 8. Chemical QRA structure

For the two-train plant construct QRA event logic for at least:
- H2 release;
- NG release;
- syngas/CO release;
- high-pressure process rupture;
- reformer/hot-process release;
- PSA/tail-gas release;
- CO2 release;
- CO2 conditioning/compressor release;
- amine/CCS event where material.

For each identify:
release source → isolation → release rate/duration → dispersion → ignition/no ignition → jet/flash fire/explosion/toxic/asphyxiant consequence → escalation → receptors/nuclear structures.

Do not assign frequencies without a valid equipment inventory/failure database.

## 9. Inventory/release calculations

Where process source data allow, calculate or bound:
- H2 inventory/flow available to a release;
- NG flow;
- CO2 flow;
- process pressure/temperature where known;
- maximum credible flow-based release envelopes.

Distinguish continuous source flow from vessel inventory.

Do not invent vessel volumes, pipe diameters or hole sizes.

Where those are required, create a parameterised equation/data requirement instead.

## 10. Blast/fire/separation methodology

Determine the correct future calculation chain for nuclear/chemical separation:

release scenario
→ dispersion/congestion
→ ignition
→ explosion/fire model
→ overpressure/thermal-radiation contours
→ nuclear SSC fragility/acceptance criteria
→ separation/barrier requirement.

Use accepted methods/guidance where applicable.

If enough inputs exist for a bounded illustrative screen, perform it and label it clearly.

Do not declare a project separation distance from generic rules of thumb.

## 11. CO/CO2 dispersion and personnel/access

Define the quantitative analysis required for:
- CO toxicity;
- CO2 asphyxiation/dense-gas behaviour;
- emergency access/control-room habitability where relevant.

Use Singapore/Jurong meteorology only if authoritative site-relevant data are available.

Do not generate fake risk contours.

## 12. External-hazard quantification plan

For Jurong Island identify the quantitative inputs needed for:
- neighbouring industrial fire/explosion;
- hazardous gas;
- pipelines/storage;
- shipping/port events;
- flooding/storm surge/sea-level rise;
- extreme rainfall;
- intake blockage;
- seismic/geotechnical;
- aircraft/transport where applicable.

For each:
hazard → data source → frequency/intensity model → load at site → SSC acceptance criterion → decision.

Perform bounded quantitative screens where authoritative data permit.

## 13. Ultimate heat sink / SBO quantification

Use E3 cooling work and E4 safe-state requirements.

Determine:
- what normal heat-rejection quantities are known;
- what decay-heat duty requires transient analysis;
- independence/diversity requirements;
- duration/autonomy data required;
- common-cause vulnerabilities of seawater intake/grid/shared utilities.

Do not use 600 MWth or 246.4 MWth as decay heat.

## 14. Site dispersion / dose / EPZ

Define the project calculation chain:

mechanistic source term
+ release height/energy
+ site meteorology
+ terrain/buildings
+ population/receptors
→ atmospheric dispersion
→ dose
→ protective-action basis
→ emergency planning.

Research applicable IAEA/Singapore requirements and current framework.

Do not invent an EPZ radius.

If Singapore has no current project-specific nuclear EPZ rule, state that explicitly.

## 15. Quantitative-risk acceptance criteria

Research what acceptance criteria would apply or inform:
- nuclear risk;
- chemical QRA/MHI risk;
- individual/societal risk;
- dose/protective action;
- external hazards.

Do not merge chemical QRA criteria and nuclear safety criteria into one unsupported metric.

Explain how an integrated site would need to satisfy both regimes.

## 16. What can be quantified NOW?

Create a strict table:

Quantity | Can calculate now? | Inputs available | Method | Result/range | Evidence maturity | Missing data

The table must prevent the report from implying more quantitative safety closure than actually exists.

Potential current calculations may include:
- two-train thermal load-loss magnitude = 353.6 MWth;
- known source process flow scales;
- bounded cooling envelopes from E3;
- any transferable source-design tritium/temperature quantities;
- parameterised release equations.

Do not create numbers just to fill the table.

## 17. E5 safety-analysis programme

For every non-calculable decision-critical item create:

REQUIRED INPUT
→ HOW TO OBTAIN IT
→ REQUIRED MODEL/METHOD
→ REQUIRED OUTPUT
→ ACCEPTANCE/DECISION CRITERION
→ ROADMAP STAGE.

This must cover at minimum:
- 600 MWth coupled transient model;
- PRA reliability data;
- mechanistic source term;
- IHX leak/rupture;
- tritium;
- chemical QRA;
- blast/fire separation;
- Jurong external hazards;
- site dispersion/dose;
- EPZ/EPR;
- security interfaces.

## 18. CN4252 feasibility interpretation

At the end answer:

- Does existing quantitative evidence falsify the concept on safety grounds?
- What quantitative evidence supports continued feasibility?
- Which unresolved analyses could still falsify it?
- Is safety currently SUPPORTED / CONDITIONAL / NOT DEMONSTRATED at each level?
- What must be achieved before the mature E2B economic case can be treated as deployable?

Do not convert “not yet falsified” into “safe”.

## 19. Later technical-visual specification

Do not generate final artwork.

Specify later deterministic technical visuals such as:
- screening event tree;
- source-term chain;
- nuclear/chemical QRA chain;
- barrier/propagation diagram;
- safety evidence maturity chart.

AI artwork may later be used only for conceptual site/equipment visuals, not risk contours/event-tree quantitative graphics.

## 20. Durable outputs

Create at minimum:
- results/E5_QUANTITATIVE_SAFETY_DEPTH.md
- results/e5_safety/e5_event_tree_register.csv
- results/e5_safety/e5_reliability_data_gaps.csv
- results/e5_safety/e5_source_term_chain.csv
- results/e5_safety/e5_qra_scenario_register.csv
- results/e5_safety/e5_quantifiable_now.csv
- results/e5_safety/e5_analysis_program.csv
- results/e5_safety/E5_LATER_TECHNICAL_VISUAL_SPECIFICATION.md

Add deterministic Rust calculations/tests only where actual project-derived quantitative screens justify them.

## 21. Acceptance test

E5 passes only if the project can answer:

WHAT SAFETY QUANTITIES CAN ACTUALLY BE CALCULATED NOW?

WHAT CANNOT?

WHY NOT?

WHAT EXACT DATA ARE MISSING?

WHAT EVENT-TREE/PRA STRUCTURE APPLIES?

HOW WOULD A MECHANISTIC SOURCE TERM BE CALCULATED?

HOW WOULD CHEMICAL QRA AND SEPARATION BE CALCULATED?

HOW WOULD JURONG EXTERNAL HAZARDS BE QUANTIFIED?

HOW WOULD DOSE/EPZ BE DETERMINED?

WHAT NUMERICAL/REGULATORY CRITERIA WOULD CONTROL ACCEPTANCE?

WHAT SPECIFIC ANALYSES COULD STILL FALSIFY THE PROJECT?

If E5 merely says “perform PRA/QRA later,” it is incomplete.

## 22. Verification

At completion:
- run relevant tests;
- run Research CI;
- run Paper/reproducibility CI if triggered;
- verify E1-E4 results remain unchanged;
- verify no invented frequencies/source terms/doses/EPZ;
- verify every decision-critical gap has a concrete analysis/data path;
- verify all quantitative screens have provenance.

## 23. Report and STOP

Report:

PHASE: E5 — Quantitative Safety Depth

STARTING HEAD:

PRA EVENT-TREE STRUCTURE:

RELIABILITY DATA AVAILABLE/MISSING:

MECHANISTIC SOURCE-TERM STATUS:

CORE INVENTORY STATUS:

TRISO/TEMPERATURE STATUS:

IHX/TRITIUM QUANTIFICATION STATUS:

CHEMICAL QRA STATUS:

BLAST/FIRE/SEPARATION STATUS:

CO/CO2 DISPERSION STATUS:

EXTERNAL-HAZARD QUANTIFICATION STATUS:

UHS/SBO STATUS:

DOSE/EPZ STATUS:

RISK/ACCEPTANCE CRITERIA:

QUANTITIES CALCULATED NOW:

QUANTITIES DELIBERATELY NOT CALCULATED:

DECISION-CRITICAL DATA GAPS:

ANALYSES THAT COULD STILL FALSIFY THE PROJECT:

CN4252 SAFETY FEASIBILITY CLASSIFICATION:

IMPLEMENTATION-ROADMAP SAFETY GATES:

DURABLE OUTPUTS:

RESEARCH CI:

PAPER CI:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin E6 automatically.
Do not begin W5.
Do not begin E8.
Do not begin independent review.
