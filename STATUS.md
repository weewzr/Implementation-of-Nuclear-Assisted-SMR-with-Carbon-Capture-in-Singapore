# STATUS

## E6 — CN4252 FEASIBILITY, EFFECTIVENESS AND REQUIREMENT SYNTHESIS — COMPLETE

E6 is complete. E0-E5 evidence has been synthesized against the official CN4252 requirements without changing the underlying engineering models.

Controlling judgement:
- **CURRENT/FOAK = FAIL / NOT CURRENTLY DEPLOYMENT FEASIBLE AS DEMONSTRATED.**
- **EARLY-COMMERCIAL/BOAK = CONDITIONAL FUTURE FEASIBILITY; projected/modelled economics ~S$74.14/tCO2e.**
- **PREFERRED MATURE/10-OAK = CONDITIONAL FUTURE FEASIBILITY; ~1.834 MtCO2e/y and projected/modelled ~S$42.84/tCO2e, subject to E3-E5 deployment gates.**

Critical interpretation:
- CCS alone already clears the 0.25 MtCO2e/y minimum on E2's matched-service screen (~0.461 Mt/y); nuclear is not necessary for minimum CN4252 abatement compliance.
- Nuclear remains a conditional larger-scale high-temperature process-heat decarbonisation option, not an established preferred Singapore pathway.
- Jurong remains a conditional industrial context, not a demonstrated/approved nuclear site.
- Licensing-level integrated safety is not demonstrated.
- Historical S$3.725/tCO2e is superseded as controlling economics and retained only as audit provenance.

Durable outputs:
- `results/E6_CN4252_SYNTHESIS.md`
- `results/e6_synthesis/e6_requirement_matrix.csv`
- `results/e6_synthesis/e6_deployment_stage_matrix.csv`
- `results/e6_synthesis/e6_controlling_claims.csv`
- `results/e6_synthesis/e6_superseded_claims.csv`
- `results/e6_synthesis/e6_roadmap_handoff.csv`

**Current research gate: E6 CLOSED. STOP.** Do not begin E7, W5, E8, final visual generation or independent review without explicit instruction.

---

## E5 — QUANTITATIVE SAFETY DEPTH — COMPLETE

E5 is complete. E4's causal safety case has been converted into screening event-tree logic, reliability-data requirements, a mechanistic source-term calculation chain, chemical-QRA scenario logic, a strict quantifiable-now boundary and an executable quantitative safety-analysis programme.

E5 classification: **QUANTITATIVE SAFETY SCREEN PARTIAL / INTEGRATED SAFETY CONDITIONAL / LICENSING-LEVEL SAFETY NOT DEMONSTRATED.**

Quantified/reproduced now:
- 176.8 MWth one-train and 353.6 MWth simultaneous two-train process-heat demand steps;
- E3 246.4 MWth residual-disposition identity and illustrative cooling envelopes, explicitly not decay heat/UHS duty;
- 68 MMSCFD NG, 260 MMSCFD H2 and ~1.085 Mt/y captured-CO2 throughput scales;
- HTTR secondary-loop tritium values retained only as source measurements, not project predictions.

Deliberately not calculated:
- PRA/event frequencies or CDF;
- project core radionuclide inventory without fuel/burnup/history inputs;
- project TRISO accident release without 600 MWth transient temperatures;
- mechanistic environmental source term;
- IHX rupture/leak transfer;
- project tritium concentration;
- chemical risk/blast/fire/toxic/asphyxiant contours;
- nuclear/chemical separation distance;
- safety UHS/decay-heat duty or SBO autonomy;
- off-site dose or Singapore EPZ.

Durable outputs:
- `results/E5_QUANTITATIVE_SAFETY_DEPTH.md`
- `results/e5_safety/e5_event_tree_register.csv`
- `results/e5_safety/e5_reliability_data_gaps.csv`
- `results/e5_safety/e5_source_term_chain.csv`
- `results/e5_safety/e5_qra_scenario_register.csv`
- `results/e5_safety/e5_quantifiable_now.csv`
- `results/e5_safety/e5_analysis_program.csv`
- `results/e5_safety/E5_LATER_TECHNICAL_VISUAL_SPECIFICATION.md`

**Current research gate: E5 CLOSED. STOP.** Do not begin E6, W5, E8, final visual generation or independent review without explicit instruction.

---

## E4 — INTEGRATED NUCLEAR + CHEMICAL SAFETY CASE — COMPLETE

E4 is complete. The preferred E2B/E3 architecture has been converted into a project-specific screening safety case covering nuclear initiating events, IHX/interface failures, chemical hazards, bidirectional propagation, common-cause dependencies, Jurong external hazards, defence-in-depth barriers, safe-state functions and evidence maturity.

E4 classification: **ENGINEERING-SUPPORTED SAFETY ARCHITECTURE / INTEGRATED SAFETY CONDITIONAL / LICENSING-LEVEL SAFETY NOT DEMONSTRATED.**

Key boundaries:
- HTTR/AGR/JAEA evidence is retained with explicit scale/transferability limits;
- a simultaneous two-train trip is a 353.6 MWth process-heat-sink step, not an invented decay-heat or dump-cooler duty;
- the IHX provides physical separation but not perfect radiological isolation; project tritium remains unresolved;
- no event frequency, core-damage frequency, source-term release fraction, off-site dose, EPZ, chemical risk contour or separation distance is invented;
- Jurong external hazards remain site-specific and unresolved;
- E5 is required for PRA/source-term/QRA quantitative depth.

Durable outputs:
- `results/E4_INTEGRATED_SAFETY_CASE.md`
- `results/e4_safety/e4_hazard_register.csv`
- `results/e4_safety/e4_barrier_matrix.csv`
- `results/e4_safety/e4_propagation_matrix.csv`
- `results/e4_safety/e4_safe_state_matrix.csv`
- `results/e4_safety/e4_evidence_matrix.csv`
- `results/e4_safety/E4_LATER_VISUAL_SPECIFICATION.md`

**Current research gate: E4 CLOSED. STOP.** Do not begin E5, W5, E8, final visual generation or independent review without explicit instruction.

---

## E3 — JURONG ISLAND SITING/COOLING FEASIBILITY — COMPLETE

E3 is complete at scientific HEAD `e09be311950e5aec1eadaacce706778b3f6c42a9`.

Current E3 conclusion: **Jurong Island is a credible candidate industrial-integration context, but nuclear-site feasibility is NOT DEMONSTRATED.** The E2B two-train architecture is retained: one 600 MWth GTHTR300C-class source, two 130 MMSCFD SMR-H2+CCS trains, 353.6 MWth useful process heat, approximately 195,892 tH2/y and approximately 1.834 MtCO2e/y lifecycle abatement on the E2B scaling basis.

E3 closes the requested screening-level siting/infrastructure questions:
- Jurong/LCT3 industrial context researched without treating LCT3 as a nuclear project;
- land/footprint evidence screened without inventing a total nuclear-site area;
- 246.4 MWth identified only as the conditional full-power residual-disposition envelope, not canonical cooling duty;
- illustrative seawater flow screens implemented reproducibly in Rust;
- two-train NG = 68 MMSCFD, H2 = 260 MMSCFD, captured CO2 = approximately 1.085 Mt/y;
- cooling, water, H2 offtake, CCS chain, nuclear/chemical separation, external hazards, security, emergency planning, environmental permitting and nuclear licensing retained as conditional/unresolved where evidence requires;
- later conceptual visual specification created as data specification only; no fictional site plan/artwork.

Verification at E3 scientific HEAD:
- Research CI `37438179971` — **PASS**;
- Paper/reproducibility CI `37438180028` — **PASS**;
- E1/E2/E2B files/results were not modified by E3;
- no Jurong nuclear-site approval, environmental permit, universal separation distance or operating cross-border CCS chain is claimed.

Durable E3 outputs:
- `results/E3_JURONG_SITING_COOLING.md`;
- `results/e3_siting/e3_source_evidence_matrix.csv`;
- `results/e3_siting/e3_cooling_screen.csv`;
- `results/e3_siting/e3_infrastructure_demands.csv`;
- `results/e3_siting/e3_jurong_decision_matrix.csv`;
- `results/e3_siting/E3_LATER_VISUAL_SPECIFICATION.md`;
- `model/src/e3_siting.rs`;
- `model/src/bin/e3_jurong_siting.rs`.

**Current research gate: E3 CLOSED. STOP.** Do not begin E4, W5, E8 or independent review without explicit instruction.

---

## FINAL READABILITY / THREE-FIGURE / REPOSITORY PACKAGING PASS — COMPLETE

**FINAL TECHNICAL-REPORT / NUMERICAL-PROVENANCE PRESENTATION PASS — COMPLETE.**

**STRICT EQUATION / CITATION COMPLIANCE — COMPLETE.**  
**REVIEW 07 — RESOLVED.**  
**CONDITIONAL MODEL PASS — RETAINED.**

Deep Feasibility / Professor-Feedback research and Independent Review 07 are complete. The one bounded Review-07 correction pass is complete and verified. Do not reopen broad literature research or begin Review 08 unless a genuinely new requirement, contradictory evidence, or submission feedback appears.

## Current scientific result

**CONDITIONAL MODEL PASS.**

Canonical screening result:
- H2 production: approximately 97,946 t/y at the 85% design-study availability basis;
- direct CO2 avoided: approximately 862,094 t/y;
- lifecycle CO2e avoided: approximately 917,139 tCO2e/y;
- candidate lifecycle intensity: approximately 1.95 kgCO2e/kgH2;
- process heat: 176.8 MWth;
- selected architecture: 600 MWth GTHTR300C-class design basis with the source 370 MWth heat branch;
- remaining reactor thermal capacity: 423.2 MWth, which is capacity and not cooling duty or an exact electricity-output claim;
- project electricity revenue: S$0/MWh;
- central screening abatement cost: approximately S$3.725/tCO2e.

These results satisfy the CN4252 numerical screening thresholds under the declared model boundary. They do **not** demonstrate commercial Singapore deployment, licensing, a selected Jurong site, a Singapore EPZ, project PRA/mechanistic source term, qualified commercial 176.8-MWth IHX, nuclear/chemical QRA, guaranteed cross-border CCS or bankable FOAK economics.

## Review 07 closure

Authoritative review: `reviews/review_07_deep_feasibility.md`  
Resolution: `reviews/review_07_resolution.md`

Review 07 originally reported 0 BLOCKER, 3 MAJOR, 2 MINOR and 1 PRESENTATION findings. All are now resolved:
- no-backup availability sensitivity is throughput/emissions only; unsupported downtime economics were removed;
- CCS delivered-storage sensitivity is emissions-only; unsupported resizing/outage economics were removed;
- decision-oriented Future Work is active in the manuscript;
- DF register and transcript-waiver status were reconciled;
- availability figure definitions distinguish effective process availability from nuclear-source availability with backup;
- exact-PDF table wrapping defects discovered during closure QA were corrected.

The unavailable second professor/research transcript was explicitly waived by the user on 2026-10-01 and is not treated as evidence or an active blocker.

## Final verified artifact

Final packaging/scientific manuscript HEAD before this status-only closure commit:
`15b81f182f5dd51e0bf38441c48a28b659bd5326`

Verification:
- Research CI `36897131627` — **PASS**;
- Paper/reproducibility CI `36897131406` — **PASS**;
- exact workflow manuscript artifact `11179719163`;
- artifact ZIP digest `sha256:365a7ac588d016fd8fb11a597b76cf7677e257036cefaceba2dfe82d1865d160`;
- exact `main.pdf` SHA256: `sha256:28c3b3db36db3335f9a8d41ccf1ecfbc4e378b76952d28c3cc788c55f63eae0d`;
- `main.pdf` — **38 pages**;
- undefined citations in final `main.log`: **0**;
- undefined references in final `main.log`: **0**;
- obsolete `fig:final-system` reference: **resolved**;
- the exact workflow artifact was downloaded and all 38 pages were rendered and visually inspected.

### Submission-facing visual QA
- **Figure 1 — PASS:** conventional SMR + CCS pathway is readable at normal page scale; furnace-to-reformer heat relationship and arrows are clear; caption expands the short forms.
- **Figure 2 — PASS:** conventional HTGR-to-power pathway is readable with no clipping/overlap and does not imply a numerical project electricity output.
- **Figure 3 — PASS:** the previously observed annotation overlap is corrected in the replacement artifact. Primary and secondary helium are visually distinct; primary reactor coolant terminates at the IHX rather than entering the chemical plant. Visible/captioned canonical values and qualifications remain intact: 600 MWth reactor basis; 925 C reactor outlet; 900 C secondary-He supply; 78.49 kg/s source flow; approximately 466 C return; 871 C reformer outlet; 176.8 MWth process heat; 423.2 MWth remaining thermal capacity; approximately 170 MWth reference physical-IHX qualification caveat; approximately 370/371 MWth source process-heat branch distinction; 88% PSA H2 recovery; 130 MMSCFD H2; and conditional CO2 transport/storage boundary. The 423.2 MWth value is not presented as electricity output; the approximately 370/371 MWth branch is not presented as one physical IHX; and 88% PSA recovery is not presented as hydrogen purity.
- **Process explanation — PASS:** the report visibly follows fission/heat generation -> primary helium -> IHX isolation -> secondary helium -> reformer heat -> reforming -> WGS -> CO2 capture -> PSA -> H2 product -> conditional CO2 conditioning/transport/storage.
- **CO2 / cost / CN4252 presentation — PASS:** the CO2 comparison, represented annual-cost comparison and official CN4252 requirement mapping are present and readable; **CONDITIONAL MODEL PASS** is clearly stated after the requirement mapping.
- **Acronym/organisation readability — PASS:** major short forms are expanded at substantive first use, in nomenclature, or in the relevant figure captions.
- **Full-PDF QA — PASS:** all 38 pages inspected. No observed clipping, content outside margins, overlapping text/equations/arrows, malformed tables, broken figures, broken glyphs, accidental blank pages or broken page boundaries. Bibliography and appendices render correctly. The logged 19.0227-pt overfull alignment warning was inspected in the exact rendered appendix and is visually acceptable; no corrective edit is warranted.

**STRICT EQUATION / CITATION COMPLIANCE — COMPLETE.**  
**REVIEW 07 — RESOLVED.**  
**CONDITIONAL MODEL PASS — RETAINED.**

The status commit following this verified manuscript HEAD changes only this closure record; it does not change the scientific model, equations, figures, generated results or manuscript.

## Remaining scientific questions

These are future deployment analyses, not blockers to the CN4252 screening-paper result:
1. project-scale 176.8-MWth IHX design, materials, creep-fatigue, inspection and lifetime qualification;
2. reactor-trip/loss-of-nuclear-heat reformer dynamic safe-state model;
3. selected-design PRA and mechanistic source term followed by Singapore meteorology, dispersion, dose and EPZ analysis;
4. site-specific nuclear/chemical QRA and required separation/barriers;
5. coherent off-design heat-rejection balance and candidate-site cooling design;
6. project tritium transport and process-side radiological-classification assessment;
7. evidence-based comparison of real Singapore candidate sites; no site is selected;
8. contracted cross-border CCS capacity, outage/buffer, liability and tariff evidence;
9. bankable FOAK project economics including financing, EPC, licensing, security, waste, decommissioning, contingency, insurance, schedule and site costs.

## Next action

**STOP MAIN RESEARCH.**

The research/review critical path requested through Review 07 is complete. Preserve the repository as the final submission candidate. The next work should be submission-specific only: extracting the required deliverable, preparing presentation material, or responding to actual instructor/submission feedback. Do not initiate another generic audit, literature pass or independent review merely because another turn is available.
