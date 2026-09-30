# Review 06 Resolution — Targeted Scientific Extension

Authoritative review: `reviews/review_06_targeted_scientific_extension.md`.

Review decision received: **TARGETED SCIENTIFIC EXTENSION VERIFIED — MINOR CORRECTIONS ONLY**.

This resolution pass is bounded to TE-R01 through TE-R04. Reviews 1--5 and FDV2 remain closed. The verified quantitative foundation is unchanged.

## TE-R01 — Accident-mechanism citations

**Original issue:** Section 5's "Accident mechanisms retained" paragraph named depressurisation/air ingress, graphite oxidation, water/steam ingress, loss of heat sink, leakage and fission-product transport without direct inline authoritative citations.

**Correction:** Added direct NRC evidence:
- NUREG/CR-6844 Vol. 1 for depressurization accidents with water ingress and air ingress;
- NUREG/CR-6944 Vol. 6 for intermediate-loop blowdown/loss of heat sink, leakage into the primary system and reactive-gas ingress in coupled process-heat systems;
- NUREG/CR-6944 Vol. 3 for HTGR fission-product transport.

The prose remains bounded: it says these mechanisms require design-specific analysis and does not infer accident probability or project-scale consequences.

**Files:** `paper/sections/05_nuclear_feasibility_safety.tex`, `paper/references.bib`.

**Evidence:** official U.S. NRC publication records for NUREG/CR-6844 and NUREG/CR-6944 Volumes 3 and 6.

**Verification:** bibliography/build/visual verification pending final CI.

**Status:** **RESOLVED subject to final build verification**.

**Scientific conclusion changed?** No.

## TE-R02 — NRC PIRT bibliography metadata

**Original issue:** `nrcngnppirt2010` used year 2010 and a paraphrased title.

**Correction:** Corrected to:
- title: *Process Heat and Hydrogen Co-Generation PIRTs*;
- report: NUREG/CR-6944, Volume 6;
- publication: March 2008;
- authors/panel chair metadata from NRC record;
- stable official NRC URL.

**File:** `paper/references.bib`.

**Evidence:** official NRC publication page.

**Verification:** manuscript citation key retained, so no active citation rewrite was required; final BibTeX convergence pending CI.

**Status:** **RESOLVED subject to final build verification**.

**Scientific conclusion changed?** No.

## TE-R03 — Figure provenance

**Original issue:** provenance wording could imply the Rust/Plotters CN4252 threshold SVG was the threshold graphic rendered in the active manuscript.

**Correction:** Reorganized `results/FIGURE_PROVENANCE.md` into:
A. Rust-generated assets included directly in the manuscript;
B. Rust-generated reproducible/reference assets not directly included;
C. manuscript-authored TikZ visualisations using canonical generated values.

The Plotters SVG is explicitly retained as a reproducible/reference asset and explicitly marked **not included** in the active manuscript. The active threshold graphics are identified as TikZ presentations of canonical verified values.

**File:** `results/FIGURE_PROVENANCE.md`.

**Verification:** provenance paths cross-checked against active `paper/main.tex`, `07_final_results.tex`, Rust generators and ledger CSV chain.

**Status:** **RESOLVED**.

**Scientific conclusion changed?** No.

## TE-R04 — High-school accessibility

**Original issue:** dense Section-5 terminology around TRISO/IPyC/SiC, creep-fatigue, source term, accident mechanisms and coupled transients.

**Correction:** Added short plain-language takeaways while preserving engineering detail:
- TRISO: tiny layered fuel barrier; defined IPyC and SiC; defined source-term potential;
- accident mechanisms: explained why favourable HTGR features do not remove air/water ingress, loss-of-heat-removal and transport analysis;
- IHX: explained its heater/separator role and defined creep-fatigue;
- coupled transients: explained cross-plant disturbance propagation.

**File:** `paper/sections/05_nuclear_feasibility_safety.tex`.

**Verification:** final exact-PDF readability inspection pending CI.

**Status:** **RESOLVED subject to final visual verification**.

**Scientific conclusion changed?** No.

## Quantitative foundation

No Review-06 correction changes:
- ~97,946 tH2/y;
- ~862,094 t/y direct avoided CO2;
- ~917,139 tCO2e/y lifecycle avoided;
- ~1.95 kgCO2e/kgH2;
- 176.8 MWth process heat;
- S$0/MWh project electricity revenue;
- ~S$3.725/tCO2e controlling screening abatement cost.

## Closure condition

Review 06 will be marked fully closed after fresh Research CI, Paper/reproducibility CI, bibliography convergence, zero undefined citations/references and exact-artifact page-by-page inspection pass.
