# Targeted Scientific Extension Evidence

## Scope
Reopened after the prior Final Submission QA specifically for nuclear-literature depth, full CN4252 feasibility/safety/implementation coverage, and reproducible Rust visualisation. The independently verified quantitative foundation was preserved.

## Claim-level manuscript audit summary

| Active section | Material claims audited | Evidence status after extension |
|---|---|---|
| Introduction/background | SMR/CCS mechanism; HTGR high-temperature relevance; IHX isolation | Primary/authoritative citations present |
| Final design/method | INL 871/925/900 C state; 176.8 MWth; Nishihara 600/370/230/88 source architecture | Primary source-backed; project-derived quantities labelled |
| Nuclear feasibility/safety | HTTR operation; TRISO; LOFC feedback; IHX construction/materials; integration hazards | Added primary JAEA/INL/NRC/IAEA evidence; demonstrated/designed/modelled distinctions explicit |
| Final results | H2, direct/lifecycle abatement, intensity, zero-credit economics | Preserved independently verified model result |
| Singapore feasibility | nuclear policy/readiness; CCS storage dependency; economic interpretation; roadmap | Updated authoritative Singapore evidence and staged decision gates |
| Limitations | project-scale IHX, lifecycle proxy, no project power claim, CCS | Corrected stale 170-MW statement and aligned with verified architecture |
| Conclusion | thresholds versus real-world feasibility | Explicit supported/conditional/unresolved/not-demonstrated interpretation |

## Nuclear evidence added
- JAEA HTTR operating evidence: demonstrated 950 C outlet, 30 MWth scale, safety-test history.
- HTTR LOFC analysis: negative reactivity feedback and graphite thermal inertia under circulator trip.
- INL AGR TRISO programme: irradiation, engineering-scale fabrication and 1600-1800 C safety-test evidence.
- JAEA HTTR IHX development/structural design: constructed 10 MW He-He IHX, Hastelloy XR, creep/fatigue/inspection constraints.
- IAEA nuclear-hydrogen coupling: intermediate-loop isolation, return-temperature/control, tritium and explosion concerns.
- NRC NGNP PIRT: chemical/nuclear co-location, inventories, layout and separation-distance safety questions.
- Singapore MTI/EMA 2026: no nuclear deployment decision, INIR readiness assessment, safety/reliability/affordability/sustainability criteria.
- Singapore MTI CCS: no suitable domestic geological storage; cross-border chain and cost remain under development.

## Feasibility conclusion
- Thermodynamic/process: SUPPORTED at screening/model level.
- Heat-source compatibility: SUPPORTED at screening level.
- High-temperature IHX/materials: CONDITIONAL.
- Nuclear safety: CONDITIONAL; project/site case not demonstrated.
- Chemical/process safety: CONDITIONAL; integrated PHA/QRA absent.
- Singapore regulatory/siting: UNRESOLVED.
- Cross-border CCS: CONDITIONAL/UNRESOLVED.
- Economic threshold: SUPPORTED within verified model; bankability NOT DEMONSTRATED.
- Deployment readiness: NOT DEMONSTRATED.

## Rust visualisation architecture
Methodological lesson adopted from OUTRAM PARK: explicit V&V/evidence status and deterministic generated outputs rather than decorative simulation.

Canonical chain:
`model/src/final_design.rs`
-> `model/src/bin/final_design_visual_data_csv.rs`
-> `results/final_design/generated/visual_data.csv`
and
`model/src/bin/final_design_heat_figure_tex.rs`
-> `results/final_design/generated/heat_flow_figure.tex`
-> manuscript.

The Rust-generated heat-flow figure uses only canonical model constants and explicitly labels 423.2 MWth as remaining thermal capacity with no electricity claim.

## Quantitative foundation invalidation check
No new primary evidence found in this pass invalidates the verified 97,946 tH2/y, 862,094 t/y direct avoided, 917,139 tCO2e/y lifecycle avoided, 1.95 kgCO2e/kgH2 intensity, 176.8 MWth duty or S$3.725/tCO2e zero-credit screening result.

One manuscript inconsistency was corrected: an old limitations sentence referenced a 170 MWth IHX as though it were the selected economic architecture. The verified selected Nishihara source heat branch is 370 MWth; detailed project-scale IHX engineering nevertheless remains unresolved.

## Remaining scientific uncertainties
- project-specific 176.8 MWth IHX mechanical design and materials qualification;
- coupled reactor/process transient response and control;
- site-specific nuclear/process hazard propagation and separation distance;
- Singapore licensing/siting/emergency-planning acceptability;
- bankable FOAK nuclear project economics;
- contracted cross-border CCS transport/storage tariff, capacity, monitoring and liability;
- Singapore-specific natural-gas upstream lifecycle inventory;
- project-specific nuclear process-heat LCA.


## Verification closure

Final targeted-extension scientific/layout commit:
`86d7e617ef69d3e84b241b8dda9eb611d5efe3ee`.

Verification:
- Research CI `36724044125`: **PASS**.
- Paper/reproducibility CI `36724043876`: **PASS**.
- Canonical manuscript artifact `11101183481`: **19 pages**.
- BibTeX/latexmk convergence: **PASS**.
- Undefined citations/references after completed build: **0**.
- Exact-artifact page-by-page visual inspection: **PASS**.

Specific visual acceptance:
- Rust-generated thermal-capacity figure: PASS; no overlapping labels and no unsupported electricity inference.
- Rust-generated lifecycle/cost ledger tables: PASS; readable and model-reconciled.
- Nuclear engineering/safety section: PASS.
- Singapore feasibility/implementation table and roadmap: PASS; no cell collision.
- Bibliography: PASS.
- No clipping, accidental blank pages or obsolete architecture presented as current.

## Targeted-extension milestone

The targeted scientific extension was independently reviewed in Review 06. Decision: **TARGETED SCIENTIFIC EXTENSION VERIFIED — MINOR CORRECTIONS ONLY**. The bounded corrections TE-R01--TE-R04 are recorded in `reviews/review_06_resolution.md`.

No further broad Main Research expansion is warranted. Final closure depends only on fresh CI/PDF verification of the Review-06 corrections.
