# Manuscript Development Map — W0 Current Writing State

## Scope and authority
This map records the **current post-rewrite manuscript state at starting HEAD `0fd357f741994b2ad91ecb657fd334162ef65326`** against `docs/PROGRESSIVE_MANUSCRIPT_WORKFLOW.md`. It is a writing-development map, not a scientific review and not a final-closure record. No scientific model, canonical result, review record, equation, figure, table, citation or manuscript prose is changed by W0.

The active manuscript is assembled by `paper/main.tex` from 12 main-text section files plus abstract, nomenclature and two appendices. Several historical section files remain in `paper/sections/` but are not active inputs; they are preserved as repository/audit history.

## Provenance vocabulary for later phases
Later writing phases must distinguish:
- **literature/source input** — value or claim taken from cited evidence;
- **assumption** — project-selected screening value or boundary;
- **design basis** — literature architecture/scale adopted to define the screened configuration;
- **model parameter** — value used by the computational model;
- **derived quantity** — arithmetic consequence of inputs/assumptions;
- **calculated result** — integrated model output.

The current model is **not wholly first-principles**. W0 does not begin the separate model-redevelopment workflow.

## Target narrative spine
Problem → proposed configuration → system boundary/design basis → model formulation → heat integration → hydrogen/material balance → carbon/CCS → energy/economics → sensitivity → CN4252 assessment → deployment constraints → conclusion.

---

## Front matter

### Abstract — `00_abstract.tex`
- **Current purpose:** State the assignment thresholds, proposed concept, main design basis, headline quantitative result and conditional conclusion.
- **Current content:** INL Case-6 process basis; GTHTR300C-class screen; 176.8 MWth, 130 MMSCFD, 97,946 t/y, 0.917 MtCO2e/y, 1.95 kgCO2e/kgH2 and S$3.725/tCO2e; limitations.
- **Action:** KEEP; SHORTEN/POLISH later (W12/W13), after body stabilises.
- **Proposed destination:** Front matter.
- **Why:** Correct role, but dense with design-selection detail before the reader sees the system.
- **Equations affected:** none.
- **Figures/tables affected:** none.
- **Cross-references affected:** none.
- **Scientific/provenance dependencies:** INL TEV-953/961; Nishihara/JAEA; final lifecycle and economic ledgers.
- **Writing/flow problem:** Too much reactor-screen detail for an abstract; source/design/derived categories are scientifically correct but compressed.

### Nomenclature — `00_nomenclature.tex`
- **Current purpose:** Define symbols/acronyms and units.
- **Current content:** Major process, nuclear, safety, economic and lifecycle terms.
- **Action:** KEEP; audit only after final architecture.
- **Proposed destination:** Front matter.
- **Why:** Supports conceptual accessibility and acronym discipline.
- **Equations affected:** none.
- **Figures/tables affected:** nomenclature longtable.
- **Cross-references affected:** none.
- **Dependencies:** all active sections.
- **Writing/flow problem:** None requiring W1; ensure it does not substitute for first-substantive-use expansion.

---

## 1. Introduction and research question — `01_introduction.tex`

### Section: Introduction and research question
- **Current purpose:** Define CN4252 problem, SMR/CCS heat gap, HTGR opportunity, research question, analytical sequence and model-provenance honesty.
- **Current content:** Concise problem → concept → research question → roadmap; explicit statement that model mixes literature parameters, assumptions, balances and Rust calculations.
- **Action:** KEEP; SHORTEN/POLISH only in W2.
- **Proposed destination:** Target Section 1 Introduction.
- **Why:** Already closely aligned with progressive workflow.
- **Equations affected:** none.
- **Figures/tables affected:** none.
- **Cross-references affected:** none.
- **Scientific/provenance dependencies:** official CN4252 statement; INL process evidence; GTHTR300C design basis.
- **Writing/flow problem:** Singapore industrial context is minimal; research gap/contribution statement could be sharper in W2 without expanding generic background.

---

## 2. Proposed system — `02_background.tex`

### Section: Proposed system
- **Current purpose:** Explain physical solution before equations.
- **Current content:** Central idea; conventional process, reference HTGR pathway and proposed coupled configuration.
- **Action:** KEEP; EXPAND/REFINE in W3 only.
- **Proposed destination:** Target Section 2 Proposed system.
- **Why:** Correct narrative position.
- **Equations affected:** none.
- **Figures affected:** Figures `fig:smr-explainer`, `fig:htgr-power-orientation`, `fig:proposed-orientation`, `fig:ihx-isolation`.
- **Tables affected:** none.
- **Cross-references affected:** figure references throughout later heat/integration sections.
- **Scientific/provenance dependencies:** INL TEV-953/961; GTHTR300C/JAEA; IAEA coupling evidence.
- **Writing/flow problem:** Four figures/illustrations and a 15-step list may make the opening solution section longer than needed; W3 should decide what is explanatory versus repetitive.

### Subsection: From the conventional process to the proposed configuration
- **Current purpose:** Use Figures 1–3 to establish baseline, reference nuclear pathway and proposed system.
- **Current content:** Conventional SMR+CCS, HTGR electricity reference, proposed HTGR-assisted SMR+CCS with detailed caption qualifications.
- **Action:** KEEP; SHORTEN surrounding prose; preserve figure roles.
- **Proposed destination:** Section 2.1–2.3 sequence in W3.
- **Why:** Matches target architecture exactly.
- **Equations affected:** none.
- **Figures affected:** Figures 1–3.
- **Tables affected:** none.
- **Cross-references affected:** later Figure 3 references.
- **Dependencies:** source process values and GTHTR300C architecture.
- **Writing/flow problem:** Figure 3 caption carries many qualifications; body should explain the core physical separation without duplicating the caption.

### Subsection: How the proposed plant works
- **Current purpose:** Step-by-step high-school-accessible physical sequence.
- **Current content:** 15-step fission → helium → IHX → reformer → WGS → capture → PSA → H2 → CO2 boundary.
- **Action:** KEEP but MERGE/SHORTEN in W3.
- **Proposed destination:** Section 2.3 Proposed configuration.
- **Why:** Essential conceptual bridge, but 15 numbered items risk reading as procedural repetition.
- **Equations affected:** none.
- **Figures affected:** Figure 3.
- **Tables affected:** none.
- **Cross-references affected:** none material.
- **Dependencies:** INL source states, annualisation result (which should not be derived here).
- **Writing/flow problem:** Introduces annual H2/CO2 results before their derivations; later W3 should preserve source values but defer derived annual results to W6/W7.

### Subsection: Why the nuclear and chemical circuits remain separate
- **Current purpose:** Explain IHX physical isolation.
- **Current content:** Nuclear primary loop versus secondary process-heat loop; safety/interface rationale.
- **Action:** KEEP; MERGE partly with proposed configuration; MOVE detailed safety evidence to deployment section if duplicated.
- **Proposed destination:** Section 2.3 for physical separation; detailed safety evidence Section 12.
- **Why:** Physical separation belongs in system explanation; accident/safety qualification belongs later.
- **Equations affected:** none.
- **Figures affected:** `fig:ihx-isolation`, Figure 3.
- **Tables affected:** none.
- **Cross-references affected:** safety section.
- **Dependencies:** IAEA coupling/IHX evidence.
- **Writing/flow problem:** Potential duplication with Section 8/12 safety discussion.

---

## 3. Design basis and system boundary — `03_system_boundary.tex`

### Section: Design basis and system boundary
- **Current purpose:** Fix common-output comparison and provenance classes before calculation.
- **Current content:** Candidate process chain, conventional baseline, source process states, 600 MWth design basis, availability/lifecycle/economic boundaries.
- **Action:** KEEP; EXPAND provenance presentation in W4.
- **Proposed destination:** Target Section 3.
- **Why:** Correct role and position.
- **Equations affected:** displayed process-chain expression only.
- **Figures affected:** none.
- **Tables affected:** future design-basis/provenance table candidate.
- **Cross-references affected:** downstream heat, carbon and cost sections.
- **Dependencies:** master provenance register; assumptions; source register.
- **Writing/flow problem:** Important numerical inputs are prose-dense; W4 should make classification/value/unit/role easier to scan without changing numbers.

### Subsection: Process and heat design basis
- **Action:** KEEP/EXPAND in W4.
- **Problem:** Source inputs and design basis are correctly distinguished, but reactor/process evidence layers could be tabulated.

### Subsection: Annualisation and lifecycle boundary
- **Action:** KEEP; possibly MOVE lifecycle detail nearer carbon section while retaining boundary definition here.
- **Problem:** Correct boundary, but proxies need explicit classification beside later equations.

### Subsection: Economic boundary
- **Action:** KEEP; later cross-link to TEA section.
- **Problem:** S$0/y revenue assumption is clear; other cost assumptions are deferred and should be locally classified in W8.

---

## 4. Model formulation and nuclear-to-reformer heat integration — `04_final_design.tex`

### Section: Model formulation and nuclear-to-reformer heat integration
- **Current purpose:** Combine process inputs, temperature logic, reactor selection, thermal allocation, IHX screen, lifecycle/economic formulation.
- **Current content:** Largest technical section; equations for temperature differences, helium consistency, reactor fraction, remaining capacity, branch utilisation, IHX gap/ratio, helium flow, abatement-cost definition.
- **Action:** SPLIT/MOVE in W1/W5. Keep all technical content.
- **Proposed destination:** Target Section 4 Model formulation + Section 5 Nuclear-to-reformer heat integration; lifecycle/economic formulation moves to Sections 7/9.
- **Why:** Current section mixes four analytical layers and weakens the heat→hydrogen→carbon→cost sequence.
- **Equations affected:** `eq:reported-process-dt-general` through `eq:helium-flow-check`; `eq:abatement-cost-definition`.
- **Figures affected:** generated heat-flow figure.
- **Tables affected:** `tab:reactor-screen`.
- **Cross-references affected:** Section 7 ledgers; conclusions; traceability appendix.
- **Dependencies:** INL TEV-953/961; Nishihara; JAERI 170 MWth reference; assumption R-08 cp; lifecycle/economic registers.
- **Writing/flow problem:** Most important current structural problem: heat integration, lifecycle and economics coexist under one heading.

### Subsection: Process model and literature inputs
- **Action:** MOVE/MERGE with Section 3 design basis or target Section 4 formulation.
- **Problem:** Repeats values already introduced in Section 3.

### Subsection: Temperature cascade and reformer heat requirement
- **Action:** KEEP/EXPAND in W5.
- **Problem:** Good equation provenance; needs stronger purpose/result/next-use transitions.

### Subsection: Reactor selection from the process requirement
- **Action:** KEEP; SHORTEN technology-screen prose; retain evidence table.
- **Problem:** Comparator detail can distract from selected solution.

### Subsection: Reactor capacity, IHX scale and helium-loop check
- **Action:** KEEP/EXPAND in W5; central Section 5 content.
- **Problem:** Strong content; sequence can be made more physical: duty → temperature → flow → capacity → component qualification.

### Subsection: Lifecycle-model formulation
- **Action:** MOVE to carbon/CCS Section 7.
- **Problem:** Appears before hydrogen and carbon story.

### Subsection: Economic-model formulation
- **Action:** MOVE to TEA Section 9.
- **Problem:** Appears before carbon denominator is derived.

### Subsection: Model scope
- **Action:** KEEP briefly in Section 4 or limitations.
- **Problem:** Historical-study wording is useful audit context but not central narrative.

---

## 5. Hydrogen-production result — `06_integrated_configuration.tex`

### Section: Hydrogen-production result
- **Current purpose:** Explain SMR/WGS chemistry, PSA recovery and source-vs-derived H2 quantities.
- **Current content:** Reaction equations in unnumbered display form; 130 MMSCFD source result; 29,000 lb/h source rate; points to annualisation ledger.
- **Action:** EXPAND/MERGE in W6.
- **Proposed destination:** Target Section 6 Hydrogen production/material balance.
- **Why:** Correct topic but currently too short relative to importance.
- **Equations affected:** unnumbered SMR and WGS reactions; annual H2 equations currently in `07a_ledgers.tex`.
- **Figures affected:** Figure 3 may be referenced.
- **Tables affected:** stream/parameter table in Appendix B.
- **Cross-references affected:** `sec:ledgers`, `eq:annual-h2`.
- **Dependencies:** INL source model; PSA recovery source; availability assumption.
- **Writing/flow problem:** Material-balance story is thin; annual H2 derivation is physically separated in the carbon/economic ledger.

---

## 6. Carbon balance and techno-economic analysis — `07a_ledgers.tex`

### Section: Carbon balance and techno-economic analysis
- **Current purpose:** Detailed reproducible H2 annualisation, carbon/lifecycle ledger and economic ledger.
- **Current content:** ~40 numbered equations plus generated lifecycle/cost tables and bridge figures.
- **Action:** SPLIT in W1 into target Section 6 annual H2, Section 7 Carbon/CCS, Section 9 TEA. Preserve equations/labels.
- **Proposed destination:** Sections 6, 7 and 9.
- **Why:** Carbon and economics are individually strong but currently joined in one very large section.
- **Equations affected:** `eq:annual-h2-general` through `eq:abatement-cost-final`.
- **Figures affected:** `fig:co2-bridge`, `fig:cost-bridge`.
- **Tables affected:** `tab:co2-ledger`, `tab:cost-ledger`.
- **Cross-references affected:** final results, conclusion, Appendix B.
- **Dependencies:** source CO2/NG/H2 values; assumptions A-01–A-17; IEA/UNECE proxies; Nishihara economics; IEAGHG CCS anchor.
- **Writing/flow problem:** Dense equation sequence; later W7/W8 should add purpose/interpretation/next-use bridges without deleting derivations.

### Subsection: Annual hydrogen production from the source rate
- **Action:** MOVE to Section 6.
- **Problem:** Hydrogen result belongs before carbon.

### Subsection: Carbon balance and lifecycle emissions
- **Action:** KEEP as target Section 7; EXPAND narrative in W7.
- **Problem:** Good arithmetic provenance; physical carbon path could precede equations more clearly.

### Subsection: Techno-economic calculation
- **Action:** MOVE to target Section 9; EXPAND narrative in W8.
- **Problem:** Cost chain is correct but long; source economic burden versus project cost assumptions must remain explicit.

### Subsection: What is not fully priced here?
- **Action:** MOVE/MERGE with TEA limitations and deployment constraints.
- **Problem:** Avoid duplicate limitations across Sections 10/11.

---

## 7. Integrated assessment, sensitivity and CN4252 test — `07_final_results.tex`

### Section: Integrated assessment, sensitivity and CN4252 test
- **Current purpose:** Headline integrated carbon/cost tables, requirement mapping, robustness figures and conditional pass.
- **Current content:** Repeats some ledger values; official CN4252 table; availability/CCS/economic robustness.
- **Action:** SPLIT in W1/W9: sensitivity Section 10 and integrated CN4252 assessment Section 11.
- **Proposed destination:** Target Sections 10–11.
- **Why:** Sensitivity should precede final requirement verdict; integrated tables should summarize rather than re-derive.
- **Equations affected:** none directly.
- **Figures affected:** `fig:abatement-threshold`, `fig:cost-threshold`, `fig:availability-robustness`, `fig:ccs-robustness`.
- **Tables affected:** `tab:final-result` plus carbon/cost summary tables.
- **Cross-references affected:** conclusion, Appendix B.
- **Dependencies:** final Rust results and official CN4252 problem statement.
- **Writing/flow problem:** Some duplication of carbon/cost numbers immediately after detailed ledgers; W9 should make this a concise integration/robustness section.

### Integrated carbon result / Integrated cost result
- **Action:** SHORTEN/MERGE into integrated assessment.
- **Problem:** Duplicate detailed ledgers; useful as summary tables only.

### Assessment against official CN4252 requirements
- **Action:** KEEP; MOVE to end of integrated assessment after sensitivity.
- **Problem:** Correct content, slightly early relative to robustness subsections.

### Result interpretation / Why lifecycle avoidance exceeds direct avoidance
- **Action:** MERGE with Section 7 carbon interpretation or integrated discussion.
- **Problem:** Carbon interpretation belongs adjacent to carbon ledger.

### Economic robustness / Sensitivity and robustness
- **Action:** KEEP/EXPAND as target Section 10 using only supported sensitivities.
- **Problem:** Need clear base case → varied parameter → range → result → interpretation pattern.

### Conditional model pass
- **Action:** KEEP as end of target Section 11.
- **Problem:** None; this is the correct verdict location.

---

## 8. Deployment engineering: nuclear–chemical safety constraints — `05_nuclear_feasibility_safety.tex`

### Section and nine subsections
- **Current purpose:** Establish evidence maturity and unresolved safety/interface gates: TRISO, accidents, IHX, tritium, coupled transients, chemical hazards, EPZ.
- **Current content:** Deep feasibility evidence, primarily literature-backed, with explicit non-transferability.
- **Action:** KEEP but MOVE/MERGE in W1/W10 under target Section 12 Deployment constraints; SHORTEN repeated technology background.
- **Proposed destination:** Section 12.1–12.x.
- **Why:** Safety evidence is important, but it is deployment qualification after the model result, not part of the base-case calculation.
- **Equations affected:** displayed EPZ-method sequence only.
- **Figures/tables affected:** none.
- **Cross-references affected:** limitations/future work.
- **Dependencies:** HTTR/JAEA, NRC PIRTs, IAEA coupling, HTR-PM native evidence.
- **Writing/flow problem:** Long relative to the core process analysis; some material reads as a mini literature review. W10 should organize by decision gate rather than evidence catalogue.

---

## 9. Singapore feasibility and implementation roadmap — `10_singapore.tex`

### Section and eight subsections
- **Current purpose:** Translate model result into Singapore deployment conditions and staged roadmap.
- **Current content:** scale/infrastructure, nuclear readiness, siting/cooling, CCS, gas transition, bankability, roadmap, feasibility matrix.
- **Action:** KEEP; MERGE with safety/limitations/future work into target Section 12 in W1/W10.
- **Proposed destination:** Target Section 12 Deployment constraints and future work.
- **Why:** Strong decision-gate content; currently fragmented across four active sections.
- **Equations affected:** none.
- **Figures affected:** none.
- **Tables affected:** feasibility-status table.
- **Cross-references affected:** conclusion; future-work table.
- **Dependencies:** Singapore MTI/EMA/PUB evidence; CCS agreements; INIR; project model outputs.
- **Writing/flow problem:** Deployment material is distributed across Sections 8, 9, 10, 11/12; consolidation will improve balance.

---

## 10. Limitations — `11_limitations.tex`

### Section: Limitations
- **Current purpose:** State model/data/economic/safety boundaries.
- **Current content:** Aspen dependence, lifecycle proxies, IHX scale, cost proxies, zero electricity credit, unmodelled deployment gates.
- **Action:** MERGE in W1/W10 with deployment constraints; retain a concise model-limitations subsection.
- **Proposed destination:** Section 12, with model limitations separated from deployment gates.
- **Why:** Prevents repeating the same caveats in discussion, future work and conclusion.
- **Equations/figures/tables affected:** none.
- **Cross-references affected:** conclusion.
- **Dependencies:** provenance register and Review-07 boundaries.
- **Writing/flow problem:** High duplication with Sections 8, 9, 11 and conclusion.

---

## 11. Discussion: what the model establishes — `12_discussion.tex`

### Section: Discussion
- **Current purpose:** Interpret process duty, source branch, remaining thermal capacity, economics and lifecycle result.
- **Current content:** Three-question interpretation plus deeper-feasibility implications.
- **Action:** MERGE/DISTRIBUTE in W1: heat interpretation to Section 5, sensitivity interpretation to Section 10, deployment interpretation to Section 12.
- **Proposed destination:** No separate long discussion unless residual synthesis remains necessary.
- **Why:** Much of the content repeats results and limitations already stated locally.
- **Equations affected:** none.
- **Figures/tables affected:** none.
- **Cross-references affected:** none material.
- **Dependencies:** all final results; deep-feasibility evidence.
- **Writing/flow problem:** Repetition rather than advancing the physical story.

### Subsection: From screening result to engineering interpretation
- **Action:** DISTRIBUTE by topic.
- **Problem:** Contains useful interpretation but mixes reactor selection, IHX, tritium/EPZ and sensitivity.

---

## 12. Future Work: What Would Change the Decision? — `12a_future_work.tex`

### Section: Future work
- **Current purpose:** Define project/site/vendor/regulator analyses required to move conditional/unresolved findings.
- **Current content:** Decision-oriented longtable covering IHX, trips, PRA/EPZ, QRA, cooling, tritium, siting, CCS and FOAK economics.
- **Action:** KEEP; MERGE under target Section 12 after deployment constraints.
- **Proposed destination:** Section 12 final subsection.
- **Why:** Excellent decision-gate framing; should remain near deployment discussion.
- **Equations affected:** none.
- **Figures affected:** none.
- **Tables affected:** `tab:future-work`.
- **Cross-references affected:** conclusion.
- **Dependencies:** unresolved Review-07/deep-feasibility questions.
- **Writing/flow problem:** None major; section title/capitalization can be harmonized later.

---

## 13. Conclusions — `13_conclusions.tex`

### Section: Conclusions
- **Current purpose:** Answer research question numerically and state deployment qualification.
- **Current content:** Thermal result, H2, lifecycle abatement, economic result, CN4252 pass and unresolved gates.
- **Action:** KEEP; polish only in W11 after body stabilises.
- **Proposed destination:** Target Section 13.
- **Why:** Already substantially aligned with target conclusion.
- **Equations affected:** none.
- **Figures/tables affected:** none.
- **Cross-references affected:** roadmap reference may need update after W1.
- **Dependencies:** canonical final heat/H2/carbon/economic results.
- **Writing/flow problem:** Slightly long; final version should avoid re-listing every limitation already established in Section 12.

---

## Appendices

### Appendix A — Reproducibility
- **Current purpose:** Give canonical build/test/reproduction path.
- **Action:** KEEP.
- **Proposed destination:** Appendix A.
- **Dependencies:** Rust model, build script, generated data chain.
- **Writing/flow problem:** None material.

### Appendix B — CN4252 requirement traceability
- **Current purpose:** Requirement and stream/parameter provenance traceability.
- **Action:** KEEP but UPDATE cross-reference text after W1 architecture changes.
- **Proposed destination:** Appendix B.
- **Tables affected:** `tab:stream-parameters`.
- **Cross-references affected:** currently cites historical section numbers such as Sections 4, 5, 7–13 and comparator context Section 9.
- **Dependencies:** official problem statement, master provenance register.
- **Writing/flow problem:** Cross-reference prose is already stale relative to current post-rewrite ordering.

---

## Inactive/historical manuscript files retained in repository

These files exist under `paper/sections/` but are **not input by current `paper/main.tex`**:
- `04_methodology.tex` — historical governing-equation/recycle methodology.
- `05_verification.tex` — historical verification section.
- `07_results.tex` — historical Gate-5 results and joint-pass logic.
- `08_uncertainty.tex` — historical 64-case Gate-5 uncertainty/falsification study.
- `09_comparators.tex` — historical comparator context.
- `10a_deployment_extension.tex` — Review-5 deployment-scale extension.

**W0 disposition:** KEEP as audit/research history; do not silently reincorporate their superseded results into the active final-design story. If later phases need any method/evidence from them, provenance and compatibility with the current final design must be checked first.

---

## Current active structure versus target

| Current active section | Target role | W0 disposition |
|---|---|---|
| 1 Introduction | 1 Introduction | KEEP |
| 2 Proposed system | 2 Proposed system | KEEP |
| 3 Design basis/system boundary | 3 Design basis/system boundary | KEEP |
| 4 Model + heat + lifecycle + economics | 4 Model + 5 Heat + parts of 7/9 | SPLIT/MOVE |
| 5 Hydrogen result | 6 Hydrogen/material balance | EXPAND/MERGE |
| 6 Carbon + economics ledger | 6/7/9 | SPLIT |
| 7 Integrated assessment/sensitivity/CN4252 | 10/11 | SPLIT/REORDER |
| 8 Nuclear/chemical safety | 12 Deployment constraints | MOVE/MERGE |
| 9 Singapore feasibility/roadmap | 12 Deployment constraints | MERGE |
| 10 Limitations | 12 Deployment constraints/model limitations | MERGE |
| 11 Discussion | distribute to 5/10/12 | MERGE/DISTRIBUTE |
| 12 Future work | 12 Deployment constraints/future work | MERGE |
| 13 Conclusion | 13 Conclusion | KEEP |

## Major W0 flow findings
1. **Core solution direction is now correct**, but the active body still does not cleanly implement the 13-section target architecture.
2. **Section 4 is overloaded**: heat integration, lifecycle formulation and economics coexist before the hydrogen/carbon story.
3. **Hydrogen/material balance is underdeveloped** relative to heat, carbon and safety; its annualisation is located in the carbon/economic ledger.
4. **Carbon and economics are over-coupled structurally** despite each having strong detailed derivations.
5. **Sensitivity and final CN4252 assessment are combined**; target architecture requires sensitivity first, integrated verdict second.
6. **Deployment material is fragmented** across safety, Singapore feasibility, limitations, discussion and future work.
7. **Discussion repeats rather than synthesises**; most of its useful interpretation belongs beside the corresponding analysis.
8. **Appendix B cross-reference prose is stale** after the previous writing pass.
9. **Inactive historical sections can confuse repository readers** if mistaken for active manuscript content; `main.tex` remains authoritative.
10. **Provenance honesty improved substantially in the previous pass** and must be preserved: source inputs, assumptions, design bases and derived results are explicitly distinguished.

## Content-balance assessment
- **Background/context:** not currently overweight in the active opening; generic background has already been compressed. The safety/deployment literature later in the paper is comparatively long, but it serves constraints rather than introductory background.
- **Solution analysis:** directionally dominant, but **under-structured rather than simply underweight**. Heat analysis is strong; hydrogen/material-balance explanation is underweight; carbon/economics are strong but fused; sensitivity is present but mixed with the verdict.
- **Likely W1 objective:** structural redistribution only—split overloaded analytical sections, reunite hydrogen annualisation with hydrogen, separate carbon from TEA, separate sensitivity from CN4252 verdict, and consolidate deployment constraints—without detailed prose polishing.

## Recent writing-state recovery
The large post-schematic writing pass is represented by commits:
- `07ed7d1` — reordered active manuscript around solution analysis;
- `28fd7d2` — refocused background into proposed-system explanation;
- `111dad4` — clarified design basis/provenance classes;
- `5856a4b` — reframed model around heat integration and non-first-principles honesty;
- `4edef02` — changed repeated integrated-configuration prose into hydrogen-result narrative;
- `f392ecb` — made carbon/economic ledgers the analytical core;
- `6573aa1` — moved CN4252 verdict after derived ledgers;
- `116b208` — moved nuclear safety interpretation toward deployment;
- `c58fa76` — tightened discussion;
- `4fd94bc` — rewrote introduction around the engineering research question;
- `caacb31` — rewrote conclusion to answer the research question numerically;
- `0fd357f` — added the progressive manuscript workflow that now governs further writing.

## W0 acceptance
W0 changes documentation only. The active manuscript text, scientific model, canonical numbers, review records and STATUS closure language are intentionally untouched in this phase.

**Single recommended next phase: W1 — Architecture and narrative spine.**


---

## W1 implementation record — Architecture and narrative spine

W1 started from `a7ec2fe107b259d310bd98138e05a100f4172104` and changed manuscript **structure only**. The scientific model, equations, canonical values, assumptions, source classifications and review outcomes were not redeveloped.

### Active architecture implemented
1. `01_introduction.tex` — Introduction and research question
2. `02_background.tex` — Proposed system
3. `03_system_boundary.tex` — Design basis and system boundary
4. `04_model_formulation.tex` — Model formulation
5. `05_heat_integration.tex` — Nuclear-to-reformer heat integration
6. `06_hydrogen.tex` — Hydrogen production and material balance
7. `07_carbon_ccs.tex` — Carbon balance and CCS
8. `08_energy_performance.tex` — Energy performance, bounded to supported thermal allocation
9. `09_techno_economics.tex` — Techno-economic analysis
10. `10_sensitivity.tex` — Sensitivity and parametric analysis
11. `11_cn4252_assessment.tex` — Integrated assessment against CN4252
12. `12_deployment_constraints.tex` — Deployment constraints and future work
13. `13_conclusions.tex` — Conclusions

### W0 findings resolved structurally
- The overloaded former Section 4 was split into model formulation and heat integration. Its lifecycle formulation now sits with carbon/CCS; its economic formulation sits with TEA.
- Annual hydrogen production equations `eq:annual-h2-general` and `eq:annual-h2` were moved intact beside the hydrogen-production story.
- The former combined carbon/economic ledger was split into dedicated carbon/CCS and TEA sections; equation identities and generated ledger labels were preserved.
- A bounded Energy Performance section now reports only supported thermal allocation and explicitly does not introduce exergy analysis.
- Supported sensitivity material is now an active section before the integrated CN4252 verdict.
- Nuclear/chemical safety, Singapore feasibility/roadmap, model limitations and decision-oriented future work are nested under one Deployment Constraints and Future Work section.
- The old standalone Discussion is no longer active. Its useful interpretation was distributed beside energy/heat, carbon, TEA, sensitivity and deployment content.
- Appendix B requirement traceability now uses semantic section references rather than stale hard-coded section numbers.

### Content-preservation notes
Historical inactive files remain in `paper/sections/` as audit history. They are not silently reincorporated by W1. The previous combined active files also remain in the repository where applicable, but `paper/main.tex` is authoritative for the new active assembly.

### W1 flow test
The active order now supports the plain-language explanation:
idea → plant definition → model → heat → hydrogen → carbon → supported energy allocation → cost → sensitivity → assignment test → deployment barriers → answer.

Detailed prose/accessibility development remains intentionally deferred to W2–W12.

**Recommended next phase after W1 acceptance: W2 — Introduction and research question.**
