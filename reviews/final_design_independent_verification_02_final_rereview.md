# Final FDV2 Corrective Re-Verification

## Authoritative state

Current GitHub HEAD independently recovered:
`f29495e7b7c366a9f1f510a71bd367e8117eb7e4`.

This review uses the current repository as authoritative and re-verifies only:
FDV2-B01, FDV2-M01, FDV2-M02 and FDV2-RR-B01.

No model or manuscript files were modified.

## Current CI and artifact

Current HEAD:
- Research CI run 36665989364: **PASS**.
- Paper and reproducibility run 36665989403: **PASS**.
- Paper build step: PASS.
- Artifact upload: PASS.
- Canonical PDF: `paper/main.pdf`, 11 pages.
- Artifact ID: **11075968905**.

The canonical `paper/build.sh` performs a final grep for unresolved references
or citations and exits nonzero if any remain. The successful workflow therefore
establishes zero unresolved citations and zero unresolved references after
LaTeX/BibTeX convergence. Earlier-pass warnings in the workflow log are
transient and resolved before the final successful build.

The exact artifact was downloaded, rendered and visually inspected page by
page.

## Primary-source consistency recheck

Nishihara et al. 2007 is now used consistently for the selected mature
hardware/economic architecture:
- reactor: 600 MWth;
- source heat/IHX branch: 370 MWth;
- complementary source thermal branch: 230 MWth;
- source electricity: 88 MWe;
- availability: 85%;
- plant cost: 59.7 bn JPY;
- heat cost: 0.52 JPY/MJ;
- electricity cost: 4.9 JPY/kWh.

The source adverse case is correctly treated as a COST sensitivity:
- 70.9 bn JPY;
- 0.57 JPY/MJ;
- 5.5 JPY/kWh;
when assigned IHX/secondary-helium-loop cost doubles.

The current final calculation no longer uses the 170-MWth/202-MWe variant.

INL TEV-961 remains the chemical-process source:
- 925 C reactor outlet;
- 900 C supplied process heat;
- 871 C reformer outlet;
- 176.8 MWth process heat;
- 17.3 MWe process demand;
- approximately 900/466 C process-side helium state;
- 78.49 kg/s source helium flow.

## FDV2-B01 — RESOLVED

The active final implementation uses only the coherent Nishihara 600/370/230/88
source architecture for mature hardware/economic provenance.

The active manuscript now also uses the corrected architecture:
- Section 4 explicitly states 370 MWth and 88 MWe;
- Section 5 states that 176.8 MWth is below the source 370-MWth heat branch;
- the obsolete claim that 176.8 MWth exceeds a 170-MWth economic-source IHX is
  gone;
- the doubled-loop case is described only as an adverse COST sensitivity.

No active final-design calculation uses 170/202 to create the result.

The bibliography retains the older 170-MW engineering-precedent source, but the
operative final result does not use it as the Nishihara economic architecture.

There is stale non-operative documentation in `STATUS.md` and
`results/final_design/SANITY_CHECKS.md` describing the retired 161.92/144.62
MWe project mapping. This should be cleaned during final QA, but it does not
feed the current implementation, generated final-design CSV, or active
manuscript conclusion.

**Decision: FDV2-B01 — RESOLVED.**

## FDV2-M01 — REMAINS RESOLVED

Current code computes lifecycle intensity without the erroneous factor 1000.

Independent reconstruction:
- annual H2 = 97,946.015 t/y;
- baseline direct emissions = 902,060.280 t/y;
- baseline upstream NG = 206,321.689 tCO2e/y;
- lifecycle avoided = 917,138.896 tCO2e/y.

Candidate lifecycle emissions:
902,060.280 + 206,321.689 - 917,138.896
= 191,243.073 tCO2e/y.

Lifecycle intensity:
191,243.073 / 97,946.015
= **1.95254 tCO2e/tH2
= 1.95254 kgCO2e/kgH2**.

The dimensional regression test remains present and Research CI passes.

No current manuscript result contains the old ~1,953 kg/kg value.

**Decision: FDV2-M01 — REMAINS RESOLVED.**

## FDV2-M02 — RESOLVED

The previous re-review rejected the 161.92-MWe / 144.62-MWe project power
mapping because no source-supported off-design turbine/internal-load model
validated it.

The current repository has removed that mapping from the controlling final
design.

Current physical claim:
- source reactor rating = 600 MWth;
- source heat-branch capacity = 370 MWth;
- required INL process heat = 176.8 MWth;
- remaining reactor thermal capacity = 600 - 176.8 = **423.2 MWth**;
- no exact project gross MWe is claimed;
- no project net/export MWe is claimed;
- no electricity revenue is credited.

This is a materially narrower and defensible claim. The source-backed 370-MWth
heat branch exceeds the required 176.8-MWth process duty, establishing the
required heat-capacity compatibility without inventing a power-cycle output.

The process-side heat transfer also closes at screening level:
m_dot = 176800 / [5.2(900-466)]
= **78.34 kg/s**,
within about 0.2% of INL's reported 78.49 kg/s.

The project therefore no longer needs to prove an off-design turbine state in
order to support its headline CN4252 result.

Economically, unused/uncertain reactor output is not treated as free:
the zero-credit calculation charges the full selected Nishihara source-product
economic burden:
- source 370 MWth heat product at source heat cost;
- source 88 MWe generation at source electricity cost;
- project electricity revenue = zero.

This is explicitly an economic allocation, not a claim that the project
physically exports 88 MWe.

The current code includes a regression test showing electricity-value input
cannot create an unsupported project credit.

**Decision: FDV2-M02 — RESOLVED.**

## Preserved INL process and lifecycle result

Independent annualization remains:
- H2 = **97,946.015 t/y**;
- baseline direct CO2 = **902,060.280 t/y**;
- candidate direct CO2 = **39,966.477 t/y**;
- direct avoided CO2 = **862,093.803 t/y**.

Using the declared lifecycle proxy boundary:
- upstream NG reduction = approximately 72,703.833 tCO2e/y;
- nuclear heat proxy = approximately 3,649.207 tCO2e/y;
- incremental auxiliary proxy = approximately 450.483 tCO2e/y;
- CCS transport proxy = approximately 13,559.050 tCO2e/y.

Lifecycle avoided:
**917,138.896 tCO2e/y**.

CN4252 annual-abatement margin:
917,138.896 - 250,000
= **+667,138.896 tCO2e/y**.

## Independent zero-electricity-credit economics

This is the controlling economic case.

Current source normalization:
- Japan deflator factor = 112.27/99.59;
- JPY/SGD = 0.008117;
- normalized 59.7-bn-JPY plant = approximately **S$546.283m**;
- normalized heat price = approximately **S$4.75825/GJ**;
- normalized electricity-generation cost = approximately **S$44.8373/MWh**.

At 85% availability (7446 h/y), the full selected source-product burden is:
- source 370-MWth heat product: approximately **S$47.193m/y**;
- source 88-MWe generation: approximately **S$29.380m/y**;
- combined source-product reactor burden: approximately **S$76.572m/y**.

Other reconstructed terms:
- baseline NG: approximately **S$269.115m/y**;
- candidate NG: approximately **S$174.284m/y**;
- CCS CAPEX: approximately **S$114.036m**;
- annualized CCS capital: approximately **S$10.683m/y**;
- integration/site allowance annualization: approximately **S$2.857m/y**;
- T&S: approximately **S$8.135m/y**;
- electricity export revenue: **S$0/y**.

Annual incremental cost:
approximately **S$3.416m/y**.

Abatement cost:
3.416m / 917,138.896
= approximately **S$3.725/tCO2e**.

CN4252 cost margin:
100 - 3.725
= approximately **S$96.275/tCO2e** below the maximum.

Thus the headline joint pass does not depend on unsupported electricity export.

## Electricity-credit status

The previous S$100/150/200-MWh project-export cases are no longer canonical in
the final design.

Current `final_design.rs` forces project power value to zero irrespective of
the electricity-value argument, and its test verifies that changing the input
cannot change annual incremental cost.

This is the correct conservative treatment given the absence of a validated
off-design project power-cycle/internal-load model.

## Doubled-loop COST sensitivity

Current code retains Nishihara's 70.9-bn-JPY / 0.57-JPY/MJ /
5.5-JPY/kWh case only as an adverse source economic sensitivity.

It is not used as evidence of additional exchanger capacity.

The principal CN4252 pass does not require this sensitivity.

## CN4252 threshold test

Annual lifecycle avoided:
**917,138.896 tCO2e/y > 250,000 tCO2e/y**.

Margin:
**+667,138.896 t/y**.

Controlling zero-electricity-credit abatement cost:
**~S$3.725/tCO2e < S$100/tCO2e**.

Margin:
**~S$96.275/tCO2e**.

Under the declared source/design-study and lifecycle assumptions, both numerical
thresholds pass.

The manuscript correctly labels this a **CONDITIONAL MODEL RESULT**, not
demonstrated Singapore commercial feasibility.

## FDV2-RR-B01 — RESOLVED

The previous corrective state failed Paper CI.

Current HEAD:
- Research CI 36665989364: PASS.
- Paper/reproducibility CI 36665989403: PASS.
- manuscript reproduction/build: PASS.
- artifact upload: PASS.
- canonical PDF: 11 pages.
- artifact ID: 11075968905.

The canonical build script fails if the final `main.log` contains unresolved
references/citations. Because the workflow passes, final unresolved citations
= 0 and final unresolved references = 0.

The corrected PDF was successfully generated and independently inspected.

**Decision: FDV2-RR-B01 — RESOLVED.**

## PDF visual inspection

The 11-page PDF is complete:
- no blank/missing pages;
- equations render;
- citations render;
- final-results table renders;
- corrected 370-MWth source architecture is visible;
- obsolete 202-MWe/170-MW economic-source claim is not foregrounded;
- conditional-result language is visible.

One visual defect remains on page 6:
the "Remaining thermal capacity: 423.2 MWth / power output not claimed" box
overlaps the adjacent H2-product box in the final architecture figure.

This does not alter the scientific calculation, but it materially reduces
conceptual readability of the key process diagram and fails the requested
high-school-level accessibility/visual-polish standard.

### New MAJOR FDV2-FR-M01 — final architecture figure has overlapping nodes

**Evidence:** exact canonical PDF artifact 11075968905, page 6.

**Impact:** the central technology diagram is visually confusing even though the
surrounding text is correct. A novice reader can misread the remaining-thermal-
capacity branch and H2 product as one overlapping element.

**Required correction:** reposition/resize the lower-left thermal-capacity node
so it does not overlap the H2-product/PSA chain. Do not change the science.

**Acceptance criterion:** regenerated canonical PDF shows all architecture
nodes/arrows separated and readable at normal page scale; Research and Paper CI
remain PASS.

## Non-blocking repository cleanup for Final QA

Two stale non-operative documents should be reconciled during Final Submission
QA:
- `STATUS.md` still summarizes the retired 161.92/144.62-MWe mapping;
- `results/final_design/SANITY_CHECKS.md` still contains the retired project
  power mapping and S$100/150/200 cases.

They do not feed the current final-design implementation or manuscript, so they
do not reopen FDV2-M02, but leaving them stale would weaken repository
traceability.

## Finding disposition

- **FDV2-B01 — RESOLVED**
- **FDV2-M01 — REMAINS RESOLVED**
- **FDV2-M02 — RESOLVED**
- **FDV2-RR-B01 — RESOLVED**

No new scientific BLOCKER was found.

One new manuscript/visual MAJOR correction remains:
**FDV2-FR-M01**, architecture-figure overlap.

## Final decision

The final scientific/design result is now independently verified at the stated
conditional-model level. The controlling zero-electricity-credit case passes
both CN4252 numerical thresholds without relying on unsupported electricity
export.

The project is not yet ready to enter Final Submission QA because the explicit
visual-inspection acceptance condition is not fully satisfied: the central
architecture figure must be repaired first. This is a bounded presentation
correction and does not require reopening the scientific model.

**FINAL DESIGN VERIFIED WITH SPECIFIC CORRECTIONS REQUIRED**
