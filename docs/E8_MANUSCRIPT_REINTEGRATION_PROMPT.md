# E8 Main Research Prompt — Full Manuscript Reintegration and Report Closure

## Authority

Continue as Main Research for the existing CN4252 project.

Read first:
- docs/POST_SCREENING_ENGINEERING_CLOSURE_WORKFLOW.md
- docs/CN4252_BROAD_ASSIGNMENT_REQUIREMENTS.md
- docs/CN4252_PROBLEM_STATEMENT.md
- paper/REQUIREMENTS.md
- results/E1_ENGINEERING_ECONOMIC_MODEL.md
- results/E2_INCREMENTAL_NUCLEAR_BENEFIT.md
- results/E2B_ARCHITECTURE_MATURATION.md
- results/E3_JURONG_SITING_COOLING.md
- results/E4_INTEGRATED_SAFETY_CASE.md
- results/E5_QUANTITATIVE_SAFETY_DEPTH.md
- results/E6_CN4252_SYNTHESIS.md
- results/e6_synthesis/e6_controlling_claims.csv
- results/e6_synthesis/e6_superseded_claims.csv
- results/E7_IMPLEMENTATION_ROADMAP.md
- results/e7_roadmap/e7_gate_matrix.csv
- current paper/main.tex and every paper/sections/*.tex file
- paper/references.bib
- current figure/table provenance and reproducibility records.

E7 is CLOSED at commit `75a27ebda7da`.

E7 CI:
- Research CI `37502836662` — PASS
- Paper/reproducibility `37502836392` — PASS

This instruction authorises **E8 only — Full Manuscript Reintegration and Report Closure**.

Do not begin final external visual generation or independent review automatically.

## Purpose

The active manuscript predates much of E1–E7 and may still contain superseded architecture, economics, feasibility and safety framing.

E8 must make the actual report reflect the strongest current project evidence coherently from Abstract through Conclusions.

This is not permission for another broad research programme.

Use E1–E7 as canonical unless reintegration reveals a genuine contradiction.

## 1. Project identity — non-negotiable

The proposed CN4252 solution is:

**NUCLEAR-ASSISTED SMR-H2 + CCS.**

Preferred future architecture:

**1 x 600 MWth GTHTR300C-class high-temperature reactor
→ primary helium
→ IHX
→ secondary helium
→ 353.6 MWth total reformer heat
→ 2 x 130 MMSCFD SMR-H2+CCS trains.**

Conventional SMR+CCS remains the non-nuclear comparator/counterfactual.

Do not rewrite the project into a conventional CCS proposal.

## 2. Controlling deployment-stage results

The manuscript must distinguish deployment stages.

### Current / FOAK
- two-train FOAK ~S$137.74/tCO2e — FAIL cost threshold;
- current Singapore deployment/site/licensing safety not demonstrated.

### Early-commercial / BOAK
- projected/modelled ~S$74.14/tCO2e — numerical cost PASS subject to unresolved project costs/gates.

### Preferred mature / 10-OAK
- ~195,892 tH2/y;
- ~1.834 MtCO2e/y lifecycle abatement on E2B basis;
- projected/modelled ~S$42.84/tCO2e;
- numerical PASS for both CN4252 thresholds;
- conditional future feasibility, not current commercial proof.

Every occurrence of these values must carry the correct deployment/evidence qualification.

## 3. Superseded claims

Use `results/e6_synthesis/e6_superseded_claims.csv`.

At minimum remove/demote from submission-facing controlling narrative:
- S$3.725/tCO2e as final/control economics;
- one-train 600 MWth architecture as preferred final solution;
- ~97,946 tH2/y as the preferred final project scale where the mature two-train architecture is being described;
- ~0.917 MtCO2e/y as the preferred final architecture's total abatement where E2B two-train result controls;
- generic Jurong feasibility claims superseded by E3;
- generic “HTGR is safe/passively safe” language superseded by E4/E5;
- generic implementation prose superseded by E7.

Historical values may remain where needed to explain model evolution or comparator/sensitivity evidence, but must be labelled historical/superseded and must not confuse the reader.

## 4. Numerical consistency audit during reintegration

Search the entire manuscript for all important old/new values and verify context.

At minimum audit:
- 600 MWth;
- 176.8 MWth;
- 353.6 MWth;
- 370/371 MWth;
- 170 MWth;
- 423.2 MWth;
- 130 MMSCFD;
- 260 MMSCFD;
- 97,946 tH2/y;
- 195,892 tH2/y;
- 34 MMSCFD;
- 68 MMSCFD;
- 1.085 MtCO2/y captured;
- 0.25 MtCO2e/y threshold;
- S$100/tCO2e threshold;
- S$3.725/t;
- S$154/t;
- S$137.74/t;
- S$74.14/t;
- S$42.84/t;
- ~0.461 MtCO2e/y comparator abatement;
- ~1.834 MtCO2e/y preferred mature abatement.

Do not globally replace values blindly. Some old values remain valid for one-train/source/comparator cases.

## 5. Abstract

Rewrite/refine the Abstract so it answers the actual final project.

It should contain:
- Singapore decarbonisation problem;
- nuclear-assisted proposed solution;
- method scope;
- preferred two-train architecture;
- key mature projected numerical result;
- FOAK adverse result;
- major feasibility qualification;
- final conclusion.

Do not lead with the superseded S$3.725/t result.

Do not imply the mature result is observed/commercial.

## 6. Introduction / problem / originality

Ensure the Introduction clearly answers:
- problem/context;
- why conventional SMR-H2 emits;
- role of CCS;
- proposed nuclear intervention;
- why Singapore;
- originality.

Originality is the Singapore-focused integration and falsifiable engineering assessment of HTGR process heat + two-train SMR-H2 + amine CCS + maturation + siting/cooling + integrated safety.

Do not claim the component technologies are individually novel.

## 7. Proposed-system section

Figures 1–3 currently use the user's uploaded project artwork.

Preserve them unless scientific reintegration requires caption/text changes.

Update surrounding text/captions so the preferred architecture is not confused with the earlier one-train screening case.

If Figure 3 depicts one train for conceptual clarity, explicitly state whether it is a per-train schematic and that the preferred mature architecture uses two parallel trains.

Do not replace the user's artwork with old generated artwork.

## 8. Design basis / provenance

Preserve W4's strict provenance discipline.

Distinguish:
SOURCE VALUE
SOURCE MODEL
PROJECT DERIVATION
PROJECT ASSUMPTION
MODEL OUTPUT
PROJECTED DEPLOYMENT CASE.

Where two-train values are simple duplication/scaling, state that explicitly.

Do not pretend the two-train plant is a published JAEA/INL design.

## 9. Governing equations/model

Do not rewrite verified physical equations unnecessarily.

Ensure the model section makes clear which equations operate per train and how E2B constructs the two-train architecture.

Preserve:
- physical flow ordering;
- material closure;
- equilibrium scope;
- CCS topology;
- PSA recovery meaning;
- thermodynamics;
- nuclear/process heat integration;
- lifecycle calculation.

Do not call the model wholly first-principles.

## 10. Results architecture

Restructure Results so the reader can follow the scientific progression without repository history.

A strong order may be:

1. verified one-train physical/process model;
2. threshold result / original architecture limitation;
3. stronger economics;
4. non-nuclear comparator;
5. architecture innovation;
6. preferred two-train mature result;
7. deployment-stage sensitivity.

Do not make the reader reconstruct E1/E2/E2B from separate audit narratives.

## 11. Economics section

This section requires substantial update.

Present clearly:
- historical screening bridge only if useful and explicitly superseded;
- E1 stronger dedicated case ~S$154/t;
- two-train FOAK ~S$137.74/t;
- early-commercial/BOAK ~S$74.14/t;
- mature 10-OAK ~S$42.84/t.

Explain why the mature pass occurs:
**real higher utilization of the source-supported process-heat branch + evidence-backed maturation**, not threshold tuning or unsupported electricity revenue.

State:
- no electricity revenue;
- no fictional heat-sharing customer;
- unresolved project costs;
- projected/modelled evidence maturity.

Do not present S$42.84 as a quotation.

## 12. Abatement / comparator section

Explain:
- conventional SMR+CCS comparator can already exceed 0.25 MtCO2e/y (~0.461 Mt/y on E2 matched screen);
- nuclear is not necessary merely for the minimum abatement threshold;
- proposed solution remains nuclear-assisted because nuclear process heat is the chosen novel intervention;
- preferred two-train architecture gives ~1.834 MtCO2e/y on E2B basis.

Explain boundary differences between E2 and E2B rather than combining denominators carelessly.

## 13. Jurong / Singapore feasibility

Integrate E3 substantively.

Include:
- why Jurong is considered;
- LCT3 relevance with explicit non-nuclear qualification;
- industrial infrastructure;
- two-train NG/H2/CO2 scales;
- cooling calculation/screen;
- water;
- land/footprint limitations;
- CCS logistics;
- external hazards;
- nuclear siting/regulatory status.

Required classification:
**Jurong = conditional candidate industrial context; nuclear-site feasibility not demonstrated.**

Do not imply LCT3 is a nuclear site.

## 14. Safety section

This must be materially deeper than the old manuscript.

Integrate E4:
- nuclear initiating events;
- IHX/interface events;
- chemical hazards;
- bidirectional propagation;
- barriers;
- safe-state logic;
- Jurong external hazards.

Integrate E5:
- what can be quantified now;
- event-tree/PRA structure;
- mechanistic source-term chain;
- tritium;
- chemical QRA;
- separation methodology;
- UHS/SBO;
- dose/EPZ;
- exact data gaps.

Use the hierarchy:

SOURCE TECHNOLOGY: SUPPORTED
PROJECT SAFETY ARCHITECTURE: ENGINEERING-SUPPORTED
QUANTITATIVE INTEGRATED SAFETY: CONDITIONAL/PARTIAL
LICENSING/SITE SAFETY: NOT DEMONSTRATED.

Never simply say “the plant is safe.”

## 15. Implementation roadmap

Replace generic roadmap prose with E7's decision-gated structure.

Include a concise submission-facing roadmap:

CURRENT RESEARCH
→ COMPONENT/INTEGRATION QUALIFICATION
→ FOAK PROCESS-HEAT DEMONSTRATION
→ TWO-TRAIN 353.6 MWth DEMONSTRATION
→ REPLICATION / BOAK
→ MATURE / 10-OAK COST GATE
→ SINGAPORE SITE + LICENSING + CONTRACT CLOSURE
→ CONSTRUCTION / COMMISSIONING
→ COMMERCIAL OPERATION / MEASURED VERIFICATION.

Include key STOP/REDESIGN conditions.

Do not invent dates.

## 16. CN4252 requirement answer

The report must make the assignment answer easy to find.

Explicitly answer:
- decarbonisation problem/context;
- proposed solution;
- abatement potential;
- cost;
- key further questions;
- >0.25 MtCO2e/y;
- <S$100/tCO2e;
- solution-at-scale within Singapore;
- abatement mechanism;
- implementation roadmap;
- originality;
- feasibility;
- potential effectiveness;
- accuracy.

Use a compact final requirement table if useful.

Distinguish:
CURRENT/FOAK
versus
PREFERRED MATURE/FUTURE.

## 17. Discussion

The Discussion must interpret rather than repeat results.

Address:
- why the original under-utilized architecture failed economically;
- why two-train utilization matters;
- what maturation contributes;
- why CCS comparator matters;
- why nuclear remains the proposed novel intervention;
- what Jurong enables and does not prove;
- why safety remains conditional;
- what could still falsify the project;
- what the results mean for Singapore.

Do not advocate beyond evidence.

## 18. Limitations

Update limitations to include current true limitations, not stale ones.

At minimum:
- equilibrium screening rather than catalyst kinetics;
- source-based process reconstruction;
- two-train replication is project-derived;
- projected FOAK/BOAK/NOAK economics;
- unresolved project-specific CAPEX/OPEX;
- no validated project power export;
- incomplete full process water/cooling balance;
- no selected nuclear site;
- no licensing PRA/QRA/source term/dose;
- IHX/tritium qualification;
- cross-border CCS dependency;
- H2 offtake;
- regulatory framework;
- comparator-boundary differences.

Do not bury decision-critical limitations.

## 19. Conclusions

Conclusions should directly answer CN4252.

A defensible structure:

1. Proposed nuclear-assisted solution and preferred architecture.
2. Numerical mature result and threshold comparison.
3. FOAK adverse result.
4. Nuclear/comparator interpretation.
5. Feasibility status.
6. Implementation condition.
7. Final overall judgement.

Do not conclude “preferred Singapore solution” unless evidence supports that stronger claim.

Do not conclude failure merely because current FOAK fails if the report's proposed solution is explicitly a conditional mature deployment pathway.

## 20. Tables

Integrate canonical generated tables where they improve clarity.

At minimum consider:
- design basis;
- deployment-stage economics;
- comparator/abatement;
- Jurong feasibility matrix;
- safety evidence/status;
- CN4252 requirement matrix;
- implementation gates.

Avoid giant unreadable audit tables in the main body.

Move detailed provenance to appendices/repository where appropriate.

## 21. Figures

Do not perform the final external-image-generation pass yet.

Preserve user attachment Figures 1–3.

Integrate existing deterministic quantitative figures if still correct.

Identify placeholders/needs for the final visual pass:
- preferred two-train architecture if current Figure 3 is only per-train;
- Jurong conceptual integration;
- integrated safety/barrier visual;
- implementation roadmap;
- any quantitative economics/threshold figure.

Do not create polished AI visuals in E8.

Create a final visual-needs register for the coordinator/user.

## 22. Equations and number justification

Maintain the user's strict requirement:

Every material number must have:
- meaning;
- units;
- calculation/equation where project-derived;
- source/assumption;
- local citation where externally sourced;
- correct deployment-stage context.

Audit especially:
COST
CO2
REACTOR
PROCESS STREAM
SAFETY SCREEN
SITE/COOLING
ROADMAP THRESHOLD values.

Do not leave unexplained headline numbers.

## 23. Citations

Perform a manuscript-level citation audit.

Prioritise primary/authoritative sources:
- JAEA/JAERI;
- IAEA;
- NRC;
- INL/DOE;
- IEAGHG;
- Singapore Government/NEA/EMA/MTI/JTC/PUB/MOM/SCDF;
- peer-reviewed literature where primary institutional evidence is unavailable.

Every external quantitative/factual claim should have a local citation.

Do not cite internal project Markdown as if it were external literature.

Project-derived results should point to equations/tables/reproducibility records, not fake literature citations.

## 24. Reproducibility appendix

Update reproducibility instructions so a reader can reproduce:
- canonical physical model;
- E1 economics;
- E2 attribution;
- E2B maturation;
- E3 quantitative siting/cooling screens.

E4/E5 qualitative/register outputs should be reproducibly traceable even where no numerical Rust model is justified.

Ensure commands/files are current.

## 25. Requirement-traceability appendix

Update the CN4252 traceability appendix against:
- broad assignment framing;
- final problem statement;
- E6 requirement matrix.

Every requirement should point to a final manuscript location.

No stale references to superseded results.

## 26. Internal consistency audit

Search the complete manuscript for contradictions involving:
- one vs two trains;
- H2 output;
- NG flow;
- process heat;
- captured CO2;
- abatement;
- economics;
- Jurong status;
- nuclear deployment status;
- safety status;
- implementation stage.

Resolve presentation contradictions without changing canonical science.

If a genuine scientific contradiction is discovered, STOP and report it.

## 27. PDF and layout quality

Build and inspect the exact final E8 PDF.

Check:
- title/abstract;
- equations;
- citations;
- bibliography;
- tables;
- figures;
- captions;
- page breaks;
- appendix;
- cross-references;
- no clipping/overlap;
- readability at normal scale.

Do not accept a technically compiling PDF that is visibly poor.

## 28. E8 final visual-needs register

Create:
`results/E8_FINAL_VISUAL_NEEDS.md`

For each potential final visual state:
- purpose;
- manuscript location;
- whether existing figure is sufficient;
- whether deterministic Rust/TikZ is preferable;
- whether user external image generation/editing is preferable;
- required scientific content;
- numbers/labels;
- prohibited misleading implications;
- base/source image needed if applicable.

Do NOT write the actual external generation prompts yet unless needed to define the specification. The coordinator will prepare them with the user after report-content closure.

## 29. Verification

After manuscript reintegration:
- build canonical LaTeX;
- run all Rust tests;
- run Research CI;
- run Paper/reproducibility CI;
- require both PASS;
- verify zero undefined citations;
- verify zero undefined references;
- verify bibliography convergence;
- inspect exact PDF;
- verify controlling/superseded claims;
- verify all E1–E7 canonical results remain reproducible.

## 30. Acceptance test

E8 passes only if the report itself—not repository archaeology—allows a marker to answer:

WHAT IS THE NUCLEAR-ASSISTED SOLUTION?

WHAT IS THE FINAL PREFERRED ARCHITECTURE?

WHY IS IT NOVEL?

HOW MUCH H2 DOES IT PRODUCE?

HOW MUCH CO2e DOES IT ABATE?

WHAT DOES IT COST AT FOAK/BOAK/MATURE DEPLOYMENT?

WHEN DOES IT PASS CN4252?

WHY DOES NUCLEAR REMAIN IN THE SOLUTION IF CCS ALONE CAN PASS THE MINIMUM ABATEMENT THRESHOLD?

IS JURONG ACTUALLY APPROVED/FEASIBLE?

WHAT DOES THE SAFETY EVIDENCE SHOW?

WHAT IS NOT YET DEMONSTRATED?

WHAT IS THE IMPLEMENTATION ROADMAP?

WHAT COULD STILL STOP THE PROJECT?

If these answers require reading E1–E7 repository files rather than the paper, E8 is incomplete.

## 31. Report and STOP

Report:

PHASE: E8 — Full Manuscript Reintegration

STARTING HEAD:

FILES CHANGED:

ABSTRACT UPDATED:

PROJECT IDENTITY CONSISTENT:

PREFERRED TWO-TRAIN ARCHITECTURE INTEGRATED:

SUPERSEDED S$3.725 CLAIM REMOVED/DEMOTED:

FOAK/BOAK/10-OAK ECONOMICS INTEGRATED:

E2 COMPARATOR FINDING INTEGRATED:

JURONG/COOLING INTEGRATED:

E4/E5 SAFETY INTEGRATED:

E7 ROADMAP INTEGRATED:

CN4252 REQUIREMENTS DIRECTLY ANSWERED:

ORIGINALITY CLEAR:

LIMITATIONS UPDATED:

CONCLUSIONS UPDATED:

NUMBER/CITATION AUDIT:

TABLES INTEGRATED:

FIGURES STATUS:

FINAL VISUAL-NEEDS REGISTER CREATED:

REPRODUCIBILITY APPENDIX UPDATED:

TRACEABILITY APPENDIX UPDATED:

LATEX BUILD:

RESEARCH CI:

PAPER CI:

PDF INSPECTION:

SCIENTIFIC CONTRADICTIONS FOUND:

REMAINING CONTENT ISSUES:

COMMIT SHA:

RECOMMENDED NEXT ACTION:

Then STOP.

Do not begin external image generation.
Do not begin independent review automatically.
