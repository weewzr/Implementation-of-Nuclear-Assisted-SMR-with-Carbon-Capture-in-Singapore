# Scientific Paper Benchmark for the CN4252 Targeted Extension

## Benchmark set

The set was chosen for methodological diversity rather than agreement with the project.

1. **Morris et al. (2016), Nuclear Engineering and Design 306, 24-35** — AGR-1 TRISO post-irradiation heating tests. Experimental safety paper with explicit specimen/test conditions, measured release data, failure observations and bounded conclusions.
2. **Demkowicz et al. (2015), Journal of Nuclear Materials 464, 320-330** — first AGR-1 high-temperature safety tests. Strong example of experiment -> measured release -> coating-function interpretation.
3. **Sawa, Suzuki & Shiozawa (2001), Nuclear Engineering and Design 208, 305-313** — HTTR fuel safety criteria/quality control. Shows how nuclear safety claims are tied to a retention function and acceptance criterion rather than generic 'safe' language.
4. **Sawa & Ueta (2004), Nuclear Engineering and Design 233, 163-172** — HTTR fuel R&D. Separates fabrication, irradiation/performance and development extensions.
5. **Mouri, Nishihara & Kunitomi (2007), Transactions AESJ 6, 253-261** — GTHTR300C nuclear/thermal design. Design paper: states design objective, operating period/burnup and explicit safety/design limits, then reports whether the design satisfies them.
6. **Nishihara, Mouri & Kunitomi (2007), ICONE15** — GTHTR300C hydrogen cogeneration. Architecture/economic design study; useful for separating proposed system architecture from demonstrated HTTR evidence.
7. **HTTR long-term/safety-demonstration literature (Nuclear Engineering and Design)** — uses operating/test chronology and measured transient behaviour before interpreting inherent characteristics.
8. **Progress in Nuclear Energy 154 (2022), 104435** — 1-D pseudo-homogeneous helium-heated HTR-10/SMR model. Modelling paper: equations/assumptions precede parametric results; reported conversion/yield ranges are model outputs, not plant observations.
9. **International Journal of Hydrogen Energy 247 (2026), 155890** — comparative HTGR helium-heated vs electrified SMR TEA/environmental study. Demonstrates direct comparison of pathways and keeps cost/GWP outputs conditional on model boundary.
10. **IEAGHG 2017-02** — merchant SMR+CCS technical/economic report. Strong system-boundary, process-flow, stream/cost table and case-comparison practice.

## Structural lessons

### Abstract
Strong papers usually compress five items: problem/context, method/configuration, one or two defining conditions, headline quantitative result, and bounded significance. They do not introduce every caveat or derivation.

**Adopt:** keep CN4252 threshold, final configuration, verified result and conditional-feasibility qualifier.
**Reject:** abstract as a miniature literature review.

### Introduction
Good engineering introductions move from the engineering problem to the specific unresolved gap, then state the exact objective. Nuclear papers distinguish an operating reference facility from the proposed/design system early.

**Adopt:** Singapore problem -> why SMR heat matters -> why HTGR is considered -> evidence/maturity gap -> research question.
**Reject:** textbook-length hydrogen/SMR background before the research gap.

### Configuration before equations
Design/model papers show the system boundary/configuration before dense equations. This makes each later balance equation physically locatable.

**Adopt:** retain beginner process schematic and final nuclear/process architecture before detailed balance interpretation.

### Assumptions and evidence tiers
Strong nuclear design papers identify operating conditions, design limits and source/assumption status before claiming feasibility. Experimental papers state test conditions before interpreting safety performance.

**Adopt:** DEMONSTRATED / DESIGNED / MODELLED / PROJECT ASSUMPTION labels in evidence matrix and prose.

### Equation staging
Recurring useful sequence:
physical question -> control volume/boundary -> assumptions -> equation -> symbols/units -> implementation/source values -> result -> interpretation.

Main text should retain equations that establish conservation, heat transfer, lifecycle and economic logic. Long algebra or historical exploratory derivations belong in repository technical files/appendices.

**Reject:** equations appearing without a preceding physical question; long derivations that do not change interpretation.

### Results and discussion
Experimental/model papers present numerical results together with the condition under which they were obtained. Discussion then interprets mechanisms, compares literature/design limits and states what the result does not prove.

**Adopt:** final quantitative result first; then CO2/cost ledgers and feasibility interpretation.
**Reject:** presenting a threshold PASS as the whole feasibility conclusion.

## Paragraph-level lessons

Useful patterns observed:
- **Claim -> evidence -> interpretation -> limitation/implication** for nuclear safety and maturity.
- **Problem -> model -> result -> significance** for quantitative sections.
- One paragraph should usually have one engineering job.
- Citations should attach to the precise source-dependent sentence, not merely appear at paragraph end after several different claims.
- Elementary process explanations do not need citation saturation; temperatures, accident behaviour, maturity, costs, policy and safety performance do.

## Figure/table lessons

1. Schematics answer topology/isolation questions; charts answer quantitative comparison/trend questions.
2. Captions should state what is plotted/shown, the condition/basis, and the key limitation if a visual could be over-read.
3. Axes carry quantity and unit; legends distinguish physical series, not decorative categories.
4. Quantitative figures should not duplicate a table unless the visual exposes a relationship/margin.
5. Nuclear loop figures make boundaries and coolant identities explicit.
6. Safety figures distinguish barriers/functions from claimed accident probabilities.
7. Significant figures should match evidence precision.

## Nuclear-safety writing lessons

Credible nuclear papers avoid standalone labels such as 'inherently safe'. Instead they identify:
- physical characteristic (e.g. negative temperature coefficient, low power density);
- safety function affected;
- analysed/tested initiating condition;
- measured/modelled response;
- acceptance/design criterion where applicable;
- residual mechanisms/limitations.

**Adopt:** HTTR LOFC evidence and TRISO tests as condition-specific evidence.
**Reject:** 'meltdown impossible', 'TRISO cannot fail', or transferring a 9 MW test result quantitatively to a 600 MWth design.

## Supervisor Rust plotting lessons

Actual OUTRAM PARK files inspected:
- `crates/outram-park-digital-twin-engine/examples/htgr_sim_v1/app/panels.rs`
- `crates/tampines/examples/fhr_sim_v2/app/graph_pages/mod.rs`
- `crates/boon-lay/examples/triso_simulator/triso_simulator_v1/front_end/graph_page.rs`

Patterns worth adopting:
- physics/state is separate from plotting;
- plots consume dedicated snapshot/data structures;
- units are explicit and, in newer HTGR code, typed with `uom`;
- axis labels contain engineering units;
- plot series are built as explicit vectors of points;
- legends identify physical series;
- display-unit choices are display-only and cannot alter physics;
- CSV export/data inspection exists alongside visualisation.

Patterns deliberately rejected:
- interactive egui screenshots as publication figures;
- operator controls, threading and simulator state machinery irrelevant to a static screening paper;
- importing simulator correlations or claiming its V&V belongs to this project.

For publication, this project preserves the same model/data/plot separation but uses deterministic vector LaTeX/TikZ generated by Rust plus committed CSV data. This is appropriate for static PDF output and avoids screenshot workflows.

## Manuscript architecture decision

The current architecture is retained with targeted additions rather than wholesale restructuring:
1. Abstract
2. Introduction/problem
3. Background and conceptual visuals
4. System boundary
5. Final design/method
6. Nuclear engineering/safety feasibility
7. Integrated configuration
8. Verification
9. Quantitative results and simple ledgers
10. Comparators/uncertainty
11. Singapore/CCS/implementation feasibility
12. Limitations
13. Discussion
14. Conclusions
15. Reproducibility/traceability appendices

Reason: this follows the benchmark pattern of configuration/evidence before equations/results, then separates quantitative evidence from real-world feasibility while preserving the two-reader standard.
