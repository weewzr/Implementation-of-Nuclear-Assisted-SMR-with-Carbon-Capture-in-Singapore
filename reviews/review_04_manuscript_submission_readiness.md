# Independent Review 04 — Manuscript, Scientific Claims, Reproducibility and CN4252 Submission Readiness

## Repository state reviewed

Review freeze: current HEAD `ec4a28c580b238c12043ee21c8d672ccd7b8c505`.

The only change after the declared pre-review manuscript status commit
`fbbd3eccc137749c71b563069f6666271ac8feac` is the mistakenly added
current-state Review-3 re-verification file. No manuscript/model file changed,
so the scientific manuscript reviewed is the manuscript state at
`fbbd3eccc137749c71b563069f6666271ac8feac`.

Paper CI 36566879465: PASS.
Research CI 36566879451: PASS.
Actual CI artifact 11032795799 was downloaded and inspected. It contains
`main.pdf`, 13 pages.

The prior Review-3 regression check is retained as supporting evidence only and
was not repeated.

## Manuscript/PDF reviewed

Reviewed:
- official CN4252 requirements and project brief;
- STATUS.md;
- Gate-5 result and closure records;
- R3 canonical result contract;
- Reviews 1-3 and closure records;
- paper/main.tex;
- every paper/sections/*.tex file;
- paper/references.bib;
- paper/README.md and REQUIREMENTS.md;
- Research CI and Paper/reproducibility CI;
- Gate-5 result-generation script;
- actual 13-page CI-generated PDF artifact;
- all manuscript figures and tables as rendered.

## Scope

Final adversarial manuscript-level review: assignment alignment, claim-to-evidence
traceability, equations, model-class honesty, threshold claims, lifecycle,
economics, comparators, Singapore context, citations, figures/tables,
reproducibility, PDF quality and oral-defence readiness.

No scientific implementation or manuscript correction was performed.

## Prior-review regression check

The already-completed current-state Review-3 re-verification found the corrected
R3/Gate-5 computational foundation intact. Review 4 accepts that targeted
regression evidence and does not reopen Reviews 1-3.

No retired 0.737 fresh-feed fraction, 162 MWth point duty, purge-free recycle,
universal predictive PSA recovery or obsolete Case-2A constant-penalty result
was found being presented as the current canonical manuscript result.

## Assignment alignment

The manuscript explicitly states both CN4252 thresholds, evaluates them jointly,
explains the proposed abatement mechanism, gives Singapore implementation
conditions and retains the adverse result rather than manufacturing a pass.

The paper therefore addresses the assignment question scientifically. However,
it does not establish a qualifying nuclear solution: the tested nuclear domain
has zero joint passes. It also does not establish a definitive alternative
winner because Case-1A Singapore economics and eSMR/electrolysis matched
economics remain incomplete. The manuscript is appropriately explicit about
this limitation.

## Independent checks performed

- Verified the Case-1A annual direct avoided-CO2 calculation:
  (0.8091-0.3704) kg/Nm3 * 100,000 Nm3/h * 8322 h/y / 1000
  = 365,086 t/y, matching Table 2.
- Verified the dimensional form of annual avoided emissions and forward
  abatement cost.
- Checked the 950/920/900 C temperature hierarchy for positive approaches.
- Checked the Singapore-scale storage identity and claim labels.
- Cross-checked IEAGHG public source summary: 100,000 Nm3/h H2, >99.9% purity,
  ~0.81 kgCO2/Nm3 base emissions, 47-70 EUR/t avoidance range.
- Cross-checked JAERI GTHTR300C primary report: 600 MWth reactor, 950 C outlet,
  170 MW IHX.
- Cross-checked current Singapore official position: no nuclear deployment
  decision; INIR Phase 1 begins in 2027.
- Cross-checked 13 June 2025 Singapore-Indonesia CCS MOU.
- Rendered and visually inspected all 13 PDF pages.
- Inspected all manuscript tables/figures and their generation paths.

## Work that survived review

The following should NOT be unnecessarily rewritten:

1. Abstract-level scientific conclusion: zero joint nuclear passes in the tested
   domain and no claim of preferred nuclear solution.
2. Falsifiable research question and neutral treatment of adverse results.
3. System-boundary description and explicit purge/recycle topology.
4. Core SMR/WGS equations and equilibrium statement.
5. N2 purge steady-state equation.
6. Total-enthalpy reformer-duty formulation.
7. Forward abatement-cost equation and explicit statement that S$100/t is a
   threshold, not an input.
8. Verification language distinguishing CI from scientific validation.
9. Model-class caveats: equilibrium screening, bounded PSA, bounded helium
   hydraulics and scenario economics.
10. Original integrated process schematic and its non-vendor/non-design caveat.
11. Case-1A annual direct-abatement calculation and currency-year caution.
12. Singapore nuclear and cross-border CCS conditional framing.
13. Singapore-scale Table 3 claim-strength labels.
14. Limitations, Discussion and Conclusions: these are unusually disciplined
    and do not overclaim.
15. End-to-end source -> Rust -> generated CSV -> LaTeX -> PDF reproducibility
    architecture.
16. The actual PDF has no clipping, missing pages, unresolved references or
    broken floats.

## Blockers

### R4-B01 — Central 0/64 conclusion is not reproducible or interpretable from the manuscript because the tested domain and forward economic inputs are not specified

**Finding**

The manuscript's principal result is "0 of 64" joint passes in an
"evidence-backed domain", but the paper does not report the actual two-level
values/ranges defining that 64-point Cartesian design, nor the cost assumptions
needed to reproduce its S$/t axis.

**Exact evidence**

Section 7 says the design spans reformer temperature/pressure, PSA recovery,
capture fraction, upstream/auxiliary-carbon conditions and heat/fixed-cost
conditions. Section 8 repeats the six dimensions. It does not state their actual
levels.

Section 4 gives only the generic forward cost equation. It does not state the
candidate/baseline annual-cost construction, natural-gas price, electricity
price, nuclear-heat price, allocated reactor/fixed cost, CCS T&S cost,
annualisation convention or the paired low/high cost corners used by Gate 5.

The abstract and conclusion make 0/64 the principal evidence for non-compliance.

**Independent verification**

The repository's Gate-5 records show that the 64 points are 2^6 paired cases
and that economics are scenario-dependent. The manuscript therefore omits
information necessary to understand what "tested evidence-backed domain" means.
A reader with only the paper cannot reproduce or critically evaluate the
central threshold experiment.

**Affected manuscript claim/result**

The main 0/64 joint-pass conclusion; the claim that simultaneous threshold
compliance is not demonstrated in the evidence-backed domain; Figure 2;
sensitivity interpretation; CN4252 cost-threshold assessment.

**Why it matters**

A falsification statement is only meaningful relative to the tested domain.
Without parameter levels and economic assumptions, the domain is undefined to
the reader. The paper is repository-reproducible but not scientifically
self-contained enough for submission.

**Required correction**

Add a compact canonical uncertainty/economic-input table to Methods or
Uncertainty. State all six paired dimensions and their exact levels, with units,
claim class and source/assumption basis. State the forward annual-cost terms
used to produce S$/t, including currency/year treatment and which quantities
are scenario assumptions.

Do not add new physics or broaden ranges to seek a passing point.

**Acceptance criterion**

A technically competent reader can reconstruct the 64 design points and the
forward S$/t calculation from the manuscript plus cited sources, without
reverse-engineering Rust. The table values must be generated from or
regression-checked against the canonical Gate-5 configuration.

**Recommended verification test**

Add a manuscript-data generator for the uncertainty/economic-input table and a
CI test that its six binary dimensions imply exactly 64 cases and that one
selected point independently reproduces its reported annual abatement and
S$/t.

---

### R4-B02 — The central threshold figure is scientifically unusable in the actual PDF

**Finding**

Figure 2 does not visually display the 64-case threshold topology at a useful
scale.

**Exact evidence**

In `paper/sections/07_results.tex`, the vertical threshold is drawn with
coordinates `(250000,-1e9) (250000,1e9)` and the horizontal threshold with
`(-1e9,100) (1e9,100)`. PGFPlots includes those coordinates in automatic axis
limits.

The actual CI PDF therefore renders axes of roughly +/-1e9 and collapses the
physical 64-case scatter near the origin. Visual inspection of page 5 confirms
that the central data cannot be meaningfully read against the thresholds.

**Independent verification**

The PDF artifact was downloaded from workflow artifact 11032795799, rendered
page-by-page and inspected. This is not a source-code-only concern: the defect
is present in the submission artifact.

**Affected manuscript claim/result**

The main graphical evidence for the CN4252 joint-threshold result.

**Why it matters**

The caption calls this the threshold map, but a reader cannot inspect the
failure topology from the plotted data. A technically correct CSV and caption
do not rescue a figure whose axis construction hides the result.

**Required correction**

Draw threshold lines using axis-relative commands or explicit data limits
derived from the finite canonical data, so threshold lines do not determine the
axis extent. Preserve infinite-cost cases in the canonical dataset and explain
them separately as currently done; do not fabricate finite coordinates.

**Acceptance criterion**

In the rebuilt PDF, the finite 64-case cloud and both threshold lines are
visually distinguishable at a scientifically meaningful scale, axis labels and
units remain readable, and infinite/non-positive-abatement states remain
explicit in text/table classification rather than being silently discarded.

**Recommended verification test**

Render the rebuilt PDF in CI and add a deterministic plot-data sanity check that
axis bounds derive from finite physical data plus controlled margins, not
sentinel +/-1e9 threshold coordinates.

## Major findings

### R4-M01 — Central lifecycle and economic inputs lack direct manuscript citations and provenance

**Finding**

Several parameters that materially drive the central lifecycle/economic result
are stated as screening inputs without citations in the manuscript, and their
supporting sources are absent from the paper bibliography.

**Exact evidence**

Section 7.1 gives:
- 11.5 gCO2e/MJ upstream NG;
- 5.5 gCO2e/kWh-e nuclear LCA proxy input;
- 5.5 gCO2e/kWh auxiliary electricity;
- 2.5% captured-CO2 CCS transport burden.

The bibliography contains IEAGHG, NIST, JAEA/JAERI and Singapore MTI sources,
but not the IEA upstream-gas/LNG source, nuclear lifecycle source or CCS
transport source used by the research repository.

Likewise the manuscript does not cite evidence for the scenario economic inputs
underlying the 64-case S$/t calculation.

**Independent verification**

The project research notes explicitly classify these as literature-derived
global/proxy anchors rather than Singapore measurements. The manuscript correctly
calls them screening inputs, but a reader cannot audit their origin from the
paper.

**Affected manuscript claim/result**

Lifecycle decomposition, annual avoided emissions, conservative/reference
threshold behaviour, and the cost-threshold uncertainty result.

**Why it matters**

Claim-strength labels prevent overstatement but do not replace citations.
Central sensitivity anchors need traceable provenance, particularly because the
paper's conclusion depends on their tested ranges.

**Required correction**

Add the authoritative/primary sources actually used for upstream gas/LNG,
nuclear lifecycle allocation basis, CCS transport-chain burden and material
economic anchors. Cite them where the assumptions are introduced. Keep
Singapore-specific versus global/proxy distinctions explicit.

**Acceptance criterion**

Every material lifecycle/economic input appearing in the canonical threshold
experiment is either directly cited to its source or explicitly labelled as an
author-declared scenario assumption with a documented rationale. Bibliography
entries support the exact context in which each value is used.

**Recommended verification test**

Maintain a generated claim/input provenance table and fail Paper CI if a
canonical Gate-5 input lacks a source/assumption class.

---

### R4-M02 — Results section under-exposes the canonical nuclear case and threshold magnitudes

**Finding**

The paper reports the binary failure topology very clearly but provides too
little numerical information about the nuclear cases' actual threshold
coordinates.

**Exact evidence**

Table 1 reports only 0 joint / 0 abatement-only / 0 cost-only / 64 both-fail.
Figure 2 is currently unreadable because of R4-B02. Table 3 reports physical
deployment scale but not reference annual avoided emissions, reference S$/t,
best-abatement case, best-cost case, or nearest-joint case.

Gate-5 Experiment 01 already computes these diagnostics, but the manuscript does
not expose them numerically.

**Independent verification**

The Gate-5 result contract explicitly identifies best-abatement, best-cost and
closest-joint cases. These are exactly the quantities needed to understand how
far the nuclear design misses each threshold.

**Affected manuscript claim/result**

Interpretation of 0/64, Discussion statement that lifecycle/economic constraints
are restrictive, oral-defence readiness.

**Why it matters**

"All 64 fail both" is much more informative if the reader can see whether the
closest case misses by 1%, 50% or an order of magnitude. Binary classification
alone weakens scientific interpretation and makes the paper harder to defend.

**Required correction**

Add a compact generated results table reporting at least:
- reference case annual avoided emissions and S$/t;
- conservative case;
- best-abatement case;
- best finite-cost case;
- closest-joint-threshold case;
- corresponding pass/fail flags and key parameter coordinates.

Do not cherry-pick a favourable case; use the existing deterministic Gate-5
diagnostics.

**Acceptance criterion**

A reader can quantify the magnitude and direction of threshold failure from the
paper itself, and every reported number is generated from the canonical Rust
results.

**Recommended verification test**

Generate the table directly from the existing Experiment-01 diagnostics and
regression-check it against the 64-case dataset.

---

### R4-M03 — Comparator evidence is scientifically honest but too incomplete to support a strong CN4252 solution-selection narrative

**Finding**

The manuscript correctly avoids ranking alternatives, but the assignment-facing
comparison remains asymmetric: the nuclear candidate has lifecycle + forward
Singapore scenario economics, while Case 1A has direct emissions plus EUR2014
source cost, and eSMR/electrolysis remain qualitative.

**Exact evidence**

Section 9 explicitly says Case 1A Singapore S$/t is unresolved and that no
definitive eSMR/electrolysis winner is established. Section 11 repeats that some
alternatives lack a fully matched Singapore forward-cost model.

**Independent verification**

IEAGHG publicly reports avoidance costs of roughly EUR47-70/t for its SMR+CCS
cases on the source basis, supporting the manuscript's statement that
conventional CCS is a serious comparator, but this does not itself establish a
Singapore S$100/t result. The manuscript is correct not to convert unlike
currency/year bases casually.

**Affected manuscript claim/result**

Assignment-level answer to "what solution should be pursued" and comparative
effectiveness discussion.

**Why it matters**

The paper successfully falsifies robust compliance of the tested nuclear
configuration, but it does not complete a matched alternative solution. For a
research paper this is acceptable; for a CN4252 solution assignment it should
be made unmistakable that the work establishes a negative/conditional result,
not a replacement compliant solution.

**Required correction**

Do not fabricate matched economics. Strengthen the assignment-facing wording
and comparator table so it explicitly distinguishes:
- nuclear: tested and no joint pass in declared domain;
- Case 1A: annual direct-abatement threshold demonstrated, Singapore cost
  unresolved;
- eSMR/electrolysis: insufficient matched evidence in this project.

If time/evidence permit, a matched comparator screen would strengthen the paper,
but it is not acceptable to manufacture one solely for submission.

**Acceptance criterion**

Abstract/Discussion/Conclusion and comparator table consistently state what is
and is not established for each comparator, and no language implies that the
paper has identified a CN4252-compliant alternative when it has not.

**Recommended verification test**

Perform a sentence-level claim audit of all "feasible", "preferred", "meets",
"passes", "better" and "lower-cost" language against the comparator evidence
classes.

## Minor findings

### R4-m01 — PDF typography is dense

The 13-page artifact is complete and technically readable, but several figures,
tables and rotated labels are small. Figure 4 is particularly dense. Improve
only after B01/B02/M01/M02; this is not a scientific blocker.

### R4-m02 — Bibliography metadata is sparse

Several web references have generic titles/notes and no URL/access metadata.
The citations resolve, but publication-quality traceability would improve with
stable URLs/DOIs/report identifiers where available.

### R4-m03 — NIST bibliography year is misleading

The NIST Chemistry WebBook entry uses year 2026, apparently reflecting access
context rather than the underlying thermochemical dataset publication. Use an
access date or appropriate bibliographic form so readers do not infer the
Shomate data were published in 2026.

### R4-m04 — Terminology should keep SMR ambiguity controlled

The nomenclature correctly distinguishes steam methane reforming from small
modular reactor usage. Preserve this discipline throughout presentation/oral
materials because the project topic makes the acronym intrinsically ambiguous.

## Claim-to-evidence audit

Representative traceability result:

| Manuscript quantity/claim | Review classification |
|---|---|
| 100,000 Nm3/h H2, 8994 kg/h | SOURCE VALUE / source conversion |
| 96 MW conventional radiant duty | SOURCE-BACKED / VERIFIED CONVERSION |
| plant C/H/O/N closure <1e-6 | VERIFIED MODEL RESULT |
| 91.784 MWth high-grade nuclear process heat | SCREENING RESULT |
| 42.026 kg/s secondary He | SCREENING RESULT |
| 527,928 t/y CO2 to storage | VERIFIED MODEL RESULT within screening flowsheet |
| 0/64 joint passes | VERIFIED EXPERIMENTAL RESULT for declared domain |
| conservative case fails both | VERIFIED FALSIFICATION RESULT for declared case |
| Case-1A 365,086 t/y direct avoided CO2 | SOURCE-BACKED DERIVED RESULT |
| Case-1A Singapore S$/t | NOT ESTABLISHED |
| nuclear preferred for Singapore | NOT SUPPORTED and correctly rejected |

## Equation audit

Equations (1)-(7), (9) and (10) are dimensionally and conceptually consistent
with the reviewed model scope.

Equation (6) is presented without the explicit kg-to-tonne conversion used for
annual tCO2e reporting; the surrounding notation states the unit conversion.
This is acceptable but could be made clearer.

No equation was found that upgrades the screening model into kinetics or a
detailed equipment design.

## Figure audit

### Figure 1 — integrated topology
PASS. Scientifically consistent with the canonical architecture and clearly
labelled as an original screening schematic, not vendor design.

### Figure 2 — threshold map
FAIL / R4-B02. Axis sentinel coordinates make the central plot unusable in the
actual PDF.

### Figure 3 — lifecycle decomposition
PASS with claim-strength caveat. The plot is generated and its caption correctly
labels nuclear heat as an allocation proxy. Source citations for the underlying
anchors are missing (R4-M01).

### Figure 4 — local driver effects
PASS scientifically, MINOR visual-density concern. Infinite-cost perturbation is
explicitly described rather than assigned an artificial finite bar.

## Table audit

### Table 1 — binding counts
PASS. Counts sum to 64 and agree with canonical Gate-5 result: all 64 are
both-threshold failures.

### Table 2 — Case-1A comparator
PASS. Independent annual avoided calculation reproduces 365,086.14 t/y. Claim
classes appropriately distinguish EUR2014 source values and Singapore screening
T&S values.

### Table 3 — Singapore scale
PASS for screening scope. Values are clearly claim-labelled and the 170 MW IHX
ratio is not presented as a reactor-module count.

A missing central nuclear threshold-results table is R4-M02.

## Lifecycle verification

The lifecycle structure is internally consistent with the R3/Gate-5 boundary
and the manuscript does not equate captured CO2 with avoided CO2.

The principal weakness is provenance presentation: central upstream gas,
nuclear-LCA proxy and CCS-chain anchors need direct manuscript citations
(R4-M01).

The manuscript correctly labels the nuclear process-heat LCA term as a proxy and
does not call the 11.5 g/MJ input a Singapore measurement.

## Economic verification

The forward cost equation is correct and the manuscript correctly states that
S$100/t is not used as an input.

The paper is not sufficiently self-contained about the actual forward cost
model and tested economic corners (R4-B01). Therefore the economic conclusion
is repository-reproducible but not yet manuscript-reproducible.

## CN4252 threshold verification

### >0.25 MtCO2e/y
The nuclear 64-case domain has no joint pass and all 64 are classified as
both-threshold failures. The conservative case has non-positive lifecycle
abatement.

Case 1A independently gives ~365,086 t/y direct avoided CO2 at the common H2
scale, above 0.25 Mt/y. This is a direct-plant comparison, not a lifecycle
equivalence.

### <S$100/tCO2e
No tested nuclear case jointly passes the cost and annual-abatement thresholds.
The manuscript does not establish Case 1A Singapore S$/t because its source
EUR2014 cost basis is not directly combined with Singapore SGD T&S.

This is scientifically appropriate. The manuscript must expose the nuclear
economic input domain before submission (R4-B01).

## Comparator/falsification assessment

The paper genuinely attempts falsification and preserves adverse outcomes.
There is no evidence of parameter expansion solely to obtain a nuclear pass.

The strongest available comparator is conventional shifted-syngas MDEA Case 1A:
its annual direct-abatement scale clears the assignment quantity threshold.
Its Singapore economic result remains unresolved.

eSMR/electrolysis are correctly not ranked without matched evidence.

## Singapore feasibility assessment

Current manuscript statements are consistent with official evidence:
Singapore has not decided to deploy nuclear energy and is building capability
to assess possible future deployment; INIR Phase 1 is planned from 2027.
Singapore and Indonesia signed a CCS cooperation MOU on 13 June 2025.

The manuscript appropriately treats both nuclear deployment and cross-border
storage as conditions, not existing infrastructure.

## Citation audit

The citations that are present are generally attached to appropriate claims:
IEAGHG for the merchant SMR/CCS reference; NIST for thermochemistry; JAERI/JAEA
for high-temperature/IHX precedent; MTI for Singapore nuclear/CCS context.

The central missing citation layer is R4-M01: lifecycle and economic sensitivity
anchors are material to the result but absent from the paper bibliography.

No evidence was found that a cited source is being used to support a materially
stronger deployment claim than it contains.

## Reproducibility audit

Strong.

`paper/build.sh` is the canonical entry point; Paper CI uses the same path.
Rust tests run before result generation. Generated datasets feed LaTeX
figures/tables. PDF is not independently maintained.

Paper CI 36566879465 and Research CI 36566879451 passed.

Important limitation: current CI checks build/reference integrity and numerical
tests but does not detect scientific plot usability such as R4-B02, nor does it
ensure that the manuscript states all canonical uncertainty/economic inputs
(R4-B01).

## PDF quality control

Actual artifact 11032795799 inspected.

- 13 pages: PASS.
- no clipping/overlap/blank pages: PASS.
- citations/references rendered: PASS.
- equations rendered: PASS.
- Figure 1 readable: PASS.
- Figure 2 scientific scale: FAIL (R4-B02).
- Figure 3 readable: PASS.
- Figure 4 readable but dense: MINOR.
- Tables 1-3 readable: PASS.
- references complete on page 13: PASS.

## Oral-defence vulnerabilities

The manuscript can defend:
- why nuclear is not assumed to win;
- why purge exists;
- why equilibrium is only screening physics;
- why 950 C alone does not prove integration;
- why Case 1A is not directly ranked economically;
- why Singapore deployment remains conditional.

It is presently vulnerable to:
1. "What exact 64 cases did you test?" — not answerable from the paper itself.
2. "What costs make your S$/t points?" — not specified sufficiently in Methods.
3. "How close is the best nuclear case to the thresholds?" — not numerically
   exposed in the paper.
4. "Where do 11.5 g/MJ, 5.5 g/kWh and 2.5% come from?" — not cited in the paper.
5. "Show me the threshold cloud." — Figure 2 currently collapses it.

These map directly to B01, B02, M01 and M02.

## Claim-strength corrections

No central conclusion currently requires reversal.

The principal required correction is evidence exposure, not changing the
scientific result.

Keep these categories:
- 0/64: VERIFIED EXPERIMENTAL RESULT FOR DECLARED DOMAIN.
- 91.784 MWth nuclear process heat: SCREENING RESULT.
- 42.026 kg/s helium flow: SCREENING RESULT.
- Singapore deployment: CONDITIONAL SCENARIO.
- nuclear lifecycle heat term: BOUNDED ALLOCATION PROXY.
- Case-1A annual direct avoided CO2: SOURCE-BACKED DERIVED RESULT.
- Case-1A Singapore economic threshold: NOT ESTABLISHED.

## Submission-readiness decision

The manuscript's scientific direction and conclusion survive independent review.
The adverse result is credible within the declared model scope and the paper is
substantially more careful than a typical feasibility pitch.

It is not yet submission-ready because the central tested domain/economic model
is not self-contained in the manuscript and the principal threshold figure is
visually invalid in the generated PDF. These are correctable manuscript-level
defects; they do not require reopening the R3/Gate-5 scientific model.

## Main Research Handoff

1. **R4-B01 — expose the complete 64-case domain and forward economic inputs.**
   Acceptance: a reader can reconstruct all six binary dimensions and one
   complete S$/t point from the paper and cited/declared inputs without reading
   Rust.

2. **R4-B02 — repair Figure 2 axis construction.**
   Acceptance: the rebuilt PDF visibly resolves the finite 64-case cloud and
   both CN4252 threshold lines at meaningful data limits, while infinite-cost
   states remain explicitly represented outside the finite scatter.

3. **R4-M01 — add provenance/citations for all material lifecycle and economic
   sensitivity anchors.**
   Acceptance: every canonical threshold input has a source or explicit
   scenario-assumption classification in the manuscript.

4. **R4-M02 — expose threshold-failure magnitudes.**
   Acceptance: a generated table reports reference, conservative,
   best-abatement, best-cost and closest-joint cases with annual abatement,
   S$/t, pass/fail and key coordinates.

5. **R4-M03 — sharpen assignment-facing comparator status without fabricating
   matched economics.**
   Acceptance: manuscript consistently states nuclear no-pass in tested domain,
   Case-1A annual-scale pass but Singapore cost unresolved, and eSMR/electrolysis
   unmatched; no unsupported technology winner is implied.

