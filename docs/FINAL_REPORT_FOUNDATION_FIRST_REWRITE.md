# Final report first - foundational rewrite and de-assignment pass

## Objective
Produce the strongest possible independent engineering research paper on nuclear-assisted steam methane reforming with carbon capture in Singapore. Prioritize the FINAL COMPILED REPORT above historical workflow documentation, repository cosmetics, additional models, or artwork. The user wants a substantial substantive rewrite, not another superficial polish.

## Required source recovery
Read PROJECT_BRIEF.md, RESEARCH_FRAMING.md, original foundational material in attachments/, data/, literature/, and source/provenance registers. Identify original hypothesis, intended comparisons, equations and research scope. Compare with the current active paper/main.tex inclusion list, canonical E2B and post-E8 sizing results, and the current manuscript. Do not substitute outdated original assumptions for verified newer results. Record which original ideas are supported, superseded or still unresolved.

## Editorial mandate
- Rewrite the active manuscript as a self-contained engineering research paper for an intelligent non-specialist, with top-down explanation, whole-system equations before component derivations, symbols/units and worked examples.
- Remove **CN4252** from the title, author, abstract, introduction narrative, headings, figure captions, conclusion and other general scientific exposition. Do not present the paper as a course submission.
- Retain the originating requirements **annual avoided emissions >0.25 MtCO2e/y and abatement cost <S$100/tCO2e** as explicit externally specified screening/acceptance criteria in one appropriate methodology/assessment location, without repeated course branding. It is acceptable to identify the original assignment source in a discreet provenance/acknowledgment note or appendix if academically necessary; do not erase or misattribute the origin of the thresholds.
- Rename the active 'Integrated assessment against CN4252' section and any current-facing figure/table labels into scientific assessment terminology, maintaining LaTeX reference consistency.
- Open with established commercial SMR, distinguish reformer combustion vs process CO2, discuss CCS limitations, and motivate high-temperature nuclear heat. Avoid unverified claims of 'highest efficiency', 'carbon free', or 'commercial nuclear-SMR demonstration'.
- Make a clear, vivid and technically restrained storyline: industrial problem -> physical mechanism -> source-backed design -> balances -> emissions -> economics -> sensitivity -> safety/Singapore deployment -> conditional verdict.
- Remove redundant prose, repeated numerical summaries, stale one-train controlling values, duplicate explanation, and repetitive caveats where one strong caveat with appropriate cross-reference suffices. Keep necessary qualifications near each major result.
- Preserve rigorous equations, derivations, citations, references, safety and uncertainty. Do not suppress adverse FOAK S$137.74/t, projected BOAK S$74.14/t and mature 10-OAK S$42.84/t, nor imply two trains is a proven global optimum.
- Preserve correct source distinctions: 600 MWth reactor, ~370 MWth process-heat branch, 353.6 MWth two-train duty, physical IHX qualification, no invented electricity or EPZ, Jurong only a conditional context.
- Where material is valuable for traceability but too detailed for the narrative, move to appendices or link to existing repository audit/results rather than silently deleting it.
- Apply existing equation-first economics and high-school-readable specifications without bloating the report.

## Required deliverables
1. Actual edits to paper/main.tex and ALL active included sections as needed, not only a plan.
2. README.md updated to foreground final research paper, current results and reproduction instructions; move assignment/workflow details below the primary research narrative.
3. results/FINAL_REPORT_FOUNDATION_RECONCILIATION.md: original foundation vs current evidence and decisions.
4. results/FINAL_REPORT_DEDUPLICATION_AUDIT.md: section-by-section cuts, moved content, claims/units/equations preserved and any unresolved issues.
5. New final PDF artifact from current LaTeX after edits, with page-by-page visual inspection.
6. Run all Rust/reproducibility and paper CI, validate citations, equation references, numerical ledger consistency, and report exact pass/fail.
7. Report a concise before/after outline and word/page count, file list, SHA, final PDF location and outstanding risks.

## Sequencing and safety
Check current GitHub HEAD and open work before editing. Work safely and avoid concurrent edits. Make editorial changes on a branch/PR if feasible. Do not rewrite or delete historical scientific records, raw source documents, provenance, reviews or Rust model outputs merely for stylistic reasons. If an actual numerical error is discovered, isolate and document it, then reconcile model/manuscript rather than quietly changing the number. Do not start independent review automatically.

## Completion message
FINAL REPORT REWRITE:
FOUNDATIONAL MATERIAL INSPECTED:
COURSE BRANDING REMOVED FROM MAIN SCIENTIFIC NARRATIVE:
THRESHOLDS PRESERVED:
REDUNDANCY REMOVED:
EQUATIONS AND CITATIONS VERIFIED:
PDF BUILT AND VISUALLY INSPECTED:
CI RESULTS:
FILES:
COMMIT/PR:
REMAINING ISSUES:
Then STOP.
