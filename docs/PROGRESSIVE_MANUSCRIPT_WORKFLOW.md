# Progressive Manuscript Development Workflow

## Purpose
Further manuscript work MUST proceed through bounded, dependency-aware phases. Do not rewrite or "finish" the paper in one broad pass. Each phase is completed, verified, committed, and stopped before the next begins. The repository is canonical; chat sessions are workers operating on it.

## Core loop
For every pass: (1) recover current repository state; (2) inspect the scientific/writing basis relevant to the phase; (3) identify the single highest-value bounded task; (4) make only those changes; (5) preserve provenance and canonical results; (6) run appropriate checks/CI; (7) report what changed and remains; (8) STOP. Do not automatically continue.

## Writing basis
Ground further writing in the official CN4252 requirements, PROJECT_BRIEF.md, RESEARCH_FRAMING.md, current canonical model/results/provenance, current LaTeX manuscript, resolved reviews through Review 07, the analogous-paper research on HTGR process heat/nuclear hydrogen/SMR heat integration/SMR+CCS/first-law-exergy/TEA-LCA/sensitivity, and the established human-scientific-writing principles.

Target research-paper sequence:
Problem -> Proposed configuration -> System boundary and design basis -> Physical/mathematical model -> Base-case calculation -> Subsystem performance -> Integrated performance -> Sensitivity/robustness -> Deployment constraints -> Conclusion.

Approximately 75-80% of substantive technical discussion should focus on the proposed solution and its analysis. This is directional, not a word-count quota.

## Scientific honesty
Do NOT describe the current model as wholly first-principles. It contains literature-derived/hard-coded values, assumptions, design bases, engineering balances, model parameters, and derived/calculated quantities. Distinguish: literature/source input; assumption; design basis; model parameter; derived quantity; calculated result. Do not invent derivations or relabel literature inputs as first-principles. A later workflow may rederive selected hard values; that is outside this writing workflow.

## Readability and style
The conceptual story should be understandable by an intelligent high-school graduate while retaining engineering depth. Prefer:
physical question -> plain-language mechanism -> technical term -> equation/model -> inputs/provenance -> result -> interpretation -> next use.

Write as an engineer explaining a model to another engineer. Prefer plain language, specific physical nouns, direct verbs, numbers, and calibrated claims. Avoid generic AI-academic filler and inflated language such as "delve", "underscore", "pivotal", "transformative", "multifaceted", "intricate interplay", and repeated "it is important to note" constructions. Do not mechanically replace them with synonyms; rewrite around the engineering fact. Limited stylistic flair is acceptable when it clarifies the story.

## Progressive phases

### W0 - Recover current writing state
Inspect the latest repository and writing commits already made. Create/update a manuscript-development map: current section/subsection; purpose; keep/move/merge/shorten/expand; proposed destination; affected equations/figures/tables/cross-references; scientific/provenance dependencies. Do not rewrite the whole paper.

### W1 - Architecture and narrative spine
Make only structural changes needed for the solution-centred sequence. Target architecture:
1. Introduction
2. Proposed system
3. Design basis and system boundary
4. Model formulation
5. Nuclear-to-reformer heat integration
6. Hydrogen production/material balance
7. Carbon balance and CCS
8. Energy performance (only where supported)
9. Techno-economic analysis
10. Sensitivity/parametric analysis
11. Integrated assessment against CN4252
12. Deployment constraints and future work
13. Conclusion
Preserve technical content while moving/merging it. Avoid detailed prose polishing here.

### W2 - Introduction and research question
Develop only: Singapore context -> conventional hydrogen/SMR problem -> CCS role -> remaining heat problem -> HTGR process-heat opportunity -> research gap -> research question -> contributions/approach. Keep generic background short.

### W3 - Proposed system and physical explanation
Develop the solution before equations: conventional SMR+CCS -> conventional/reference HTGR pathway -> proposed HTGR-assisted SMR+CCS. The reader should be able to explain: fission -> reactor heat -> primary He -> IHX -> secondary He -> reformer heat -> SMR -> WGS -> CO2 capture -> PSA -> H2, with captured CO2 proceeding to the conditional transport/storage boundary.

### W4 - Design basis and provenance
Audit every important design number. State value, unit, meaning, why used, provenance class, and local citation where source-derived. Do not redevelop the model.

### W5 - Model formulation and heat integration
Improve explanation/ordering of the current model and nuclear-to-reformer heat integration. For important equations: purpose -> numbered equation -> definitions/units -> source/derivation -> inputs -> substitution where useful -> result -> physical meaning -> next use. Do not claim unsupported first-principles status.

### W6 - Hydrogen/material-balance story
Develop SMR -> WGS -> material balance -> PSA recovery -> H2 product -> annual production. Explain terms before acronyms. Distinguish PSA recovery from purity.

### W7 - Carbon/CCS story
Build one carbon ledger: feed carbon -> process CO2 -> capture -> residual direct emissions -> upstream contributions -> nuclear lifecycle contribution -> transport/storage contribution -> lifecycle intensity -> avoided emissions. Avoid disconnected result dumping.

### W8 - Energy and economic story
Explain supported thermal allocation, then follow the money: baseline -> represented natural-gas/process-heat costs -> nuclear heat -> CCS -> conditioning/transport/storage -> incremental annual cost -> avoided emissions -> abatement cost. Do not invent exergy analysis or unsupported costs.

### W9 - Sensitivity and integrated assessment
Use only supported sensitivities. For each: base case -> parameter varied -> range/case -> why it matters -> result -> interpretation. Then answer CN4252 requirements and explain the CONDITIONAL MODEL PASS.

### W10 - Deployment constraints
Separate model closure from deployability. Organize unresolved work including as applicable: project-scale IHX qualification; reactor-trip/reformer safe state; tritium; PRA/source term; EPZ; nuclear/chemical QRA; siting; Singapore regulatory framework; cooling; cross-border CCS; FOAK economics.

### W11 - Conclusion
Only after the body is stable. Answer the research question numerically and state principal qualifications. Avoid generic sustainability conclusions.

### W12 - Human scientific-writing pass
After technical structure is stable, perform a whole-paper prose pass for plain language, paragraph rhythm, transitions, acronym first-use expansion, repetition, unnecessary headings, AI-style filler, vague nouns, excessive nominalization, fake certainty, and unsupported novelty language. Do not alter technical meaning.

### W13 - Whole-manuscript coherence and artifact QA
Check that every section creates a reason for the next; the reader understands why each major calculation is performed; solution analysis dominates; literature inputs are distinguished from derived results; heat -> hydrogen -> carbon -> cost is continuous; conclusions do not exceed the model; undefined citations/references are zero; exact generated PDF is visually readable.

### W14 - Independent writing/technical review
Only after W13 passes. A fresh reviewer evaluates rather than rewrites: narrative flow, solution emphasis, high-school-level conceptual accessibility, engineering depth, technical fidelity, provenance honesty, human scientific-writing quality, overclaiming, and internal consistency. Main Research resolves only substantiated findings.

## Separate future workflow: model redevelopment
Do NOT mix this into W0-W14. Future scientific redevelopment begins with an inventory of literature/hard-coded values and identifies what can genuinely be replaced by more fundamental derivations/models. Improve one subsystem at a time: inventory -> derive/model -> validate against literature/reference cases -> regression test -> propagate results -> update manuscript.

## Phase acceptance rule
At the end of every phase report:
PHASE:
STARTING HEAD:
FILES CHANGED:
WHAT WAS IMPROVED:
SCIENTIFIC MODEL CHANGED?:
CANONICAL NUMBERS CHANGED?:
CITATION/REFERENCE STATUS:
RESEARCH CI:
PAPER CI (when manuscript-affecting):
REMAINING ISSUES:
SINGLE RECOMMENDED NEXT PHASE:
COMMIT:

Then STOP. Do not automatically begin the recommended next phase.

## Current-state warning
STATUS.md may contain an earlier "STOP MAIN RESEARCH" or final-artifact closure from a prior manuscript state. Subsequent user-directed manuscript-development work supersedes that instruction for this bounded writing workflow only. Do not rewrite historical review outcomes or mark a new final submission closure until W13/W14 and the exact current artifact genuinely pass.
