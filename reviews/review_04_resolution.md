# Independent Review 4 resolution

Independent Review 4 gate: **CLOSED**.

This is a resolution record for `reviews/review_04_manuscript_submission_readiness.md`; it is not another review.

## Verified state

- Corrected manuscript/model-data commit verified: `94cb2daac5c0dc537af9d7af2abf49213855b355`.
- Research CI run `36642345227`: PASS.
- Paper/reproducibility run `36642345111`: PASS.
- Generated manuscript artifact ID `11066319048`.
- Artifact SHA-256: `a32d0b55fb2c032a1ac28a9214b24404d266fd05f539504fac7b28248fda728c`.
- Converged PDF: 14 pages; zero remaining undefined citations/references under the canonical build check.
- The actual PDF artifact was rendered and visually inspected after CI.

## R4-B01 — Self-contained 64-case domain

**Disposition: RESOLVED.**

### Correction
Added deterministic Rust manuscript data `gate6_domain_table_csv()` and Table 1 in Section 8.1. It exposes all six binary dimensions:
- reformer temperature 900/950 C;
- pressure 20/28 bar;
- PSA recovery 0.70/0.90;
- capture fraction 0.85/0.95;
- paired carbon/energy-price corner (11.5;5.5;15;150) / (18.6;402;20;200);
- paired heat/fixed-cost corner (5.69;80) / (8.0;120).

The manuscript defines tuple units and explains that paired corners move together, giving exactly 2^6 = 64 cases. It also states the forward annual-cost construction, 8322 h/y basis, bundled fixed allowance, absence of a separate Gate-5 CCS T&S cash term, and currency/unit bases.

### Manuscript location
Section 8.1, Table 1 and Eq. (9).

### Evidence/provenance
IEA gas/LNG lifecycle anchors, UNECE nuclear lifecycle basis, EMA Singapore grid/electricity-price context and JAEA heat-cost anchor are cited. Exact gas-price and bundled fixed-cost values without authoritative source support are explicitly labelled project screening scenarios.

### Verification performed
Rust regression test requires six data rows and 64 canonical cases. Research CI passed. Table was inspected in the actual PDF and is readable without overlapping columns.

### Acceptance-criterion status
**PASS.** A technically competent reader can reconstruct the six binary design dimensions and the forward S$/t construction without reverse-engineering Rust.

## R4-B02 — Principal threshold figure

**Disposition: RESOLVED.**

### Correction
Removed +/-1e9 sentinel threshold coordinates. Added `gate6_threshold_plot_bounds_csv()`, which derives bounds from finite canonical physical cases with controlled margins. Figure 2 now uses a logarithmic cost axis, explicit finite bounds and axis-coordinate threshold guides that do not control limits. The plotted layer is explicitly labelled finite-cost cases.

Infinite/no-abatement cases remain `inf` in canonical data and remain represented in failure classifications/text; no finite surrogate was invented.

### Manuscript location
Section 7, Figure 2.

### Verification performed
Research/Paper CI passed. The actual CI PDF was rendered at 160 dpi and visually inspected. The finite case cloud is resolved; x ticks are readable; the 0.25 MtCO2e/y vertical line and S$100/tCO2e horizontal line are both visible and labelled.

### Acceptance-criterion status
**PASS.**

## R4-M01 — Lifecycle/economic input provenance

**Disposition: RESOLVED.**

### Correction
Added/used manuscript provenance for:
- 11.5 gCO2e/MJ average gas supply and 18.6 gCO2e/MJ delivered LNG: IEA 2026;
- 5.5 gCO2e/kWh nuclear lifecycle proxy: UNECE lifecycle assessment;
- 402 gCO2/kWh Singapore-grid alternative: EMA 2024 grid factor;
- S$150-200/MWh electricity screen: EMA 2025 wholesale-price context;
- lower nuclear-heat anchor: JAEA 0.7 JPY/MJ evaluation assumption, converted by the project;
- 2.5%/3.5% CCS-chain burdens, S$15-20/GJ gas prices, S$80-120m/y bundled fixed allowance and adverse S$8/GJ heat value: explicitly identified as project screening assumptions where no authoritative exact-value source is claimed.

The NIST bibliography entry was also corrected to identify SRD 69 with an access note rather than implying 2026 publication of the thermochemical data.

### Manuscript location
Sections 7.1 and 8.1; `paper/references.bib`.

### Verification performed
Bibliography and LaTeX build converge successfully with canonical unresolved-citation checks enabled.

### Acceptance-criterion status
**PASS.** Material canonical inputs are source-backed or explicitly classified as scenario assumptions.

## R4-M02 — Magnitude of threshold failure

**Disposition: RESOLVED.**

### Correction
Added deterministic `gate6_threshold_magnitude_csv()` and manuscript Table 3. It reports:
- reference case;
- conservative case;
- best-abatement case;
- best finite-cost case;
- closest-joint case;
with annual avoided emissions, finite/infinite S$/t, threshold pass/fail and reformer/pressure/PSA/capture coordinates.

The best/closest cases are selected algorithmically by existing Gate-5 diagnostics.

### Manuscript location
Section 8.2, Table 3.

### Verification performed
Rust tests preserve zero joint passes and the generated table row count. The actual PDF table was visually inspected and is readable. It explicitly shows all listed cases fail both thresholds and preserves the conservative infinite-cost state.

### Acceptance-criterion status
**PASS.**

## R4-M03 — Comparator/assignment-facing claim boundaries

**Disposition: RESOLVED.**

### Correction
Abstract, comparator analysis, Discussion and Conclusions now consistently state:
- nuclear-assisted SMR+CCS: 0/64 joint passes in the tested domain; not established as CN4252-compliant or preferred;
- IEAGHG Case 1A: annual direct-abatement scale exceeded on common H2/output-hours basis; matched Singapore S$/t unresolved;
- electrified SMR/electrolysis: relevant comparators but insufficient matched Singapore lifecycle/economic evidence in this project for a definitive ranking.

No missing comparator economics were fabricated.

### Manuscript location
Abstract; Sections 9, 12 and 13.

### Verification performed
Canonical Paper CI and citation/reference integrity checks passed.

### Acceptance-criterion status
**PASS.**

## Minor findings

- R4-m01: improved Figure 2 readability and compacted the new domain table; no broad cosmetic rewrite was undertaken.
- R4-m02: added stable URLs/report identifiers for new authoritative provenance entries.
- R4-m03: corrected NIST bibliographic treatment so 2026 is an access context rather than a misleading publication year.
- R4-m04: existing nomenclature distinction between steam methane reforming and nuclear small modular reactor remains intact.

## Canonical-result regression

The Review-4 corrections did not broaden the nuclear domain or change the validated R3/Gate-5 scientific result:
- 64 coupled cases;
- 0 joint CN4252 passes;
- conservative case fails both thresholds;
- Case 1A exceeds the annual direct-abatement scale on the common source basis;
- no definitive Singapore technology winner is established.

## Decision

All valid Review-4 BLOCKER and required MAJOR findings satisfy their acceptance criteria.

**Independent Review 4 gate: CLOSED.**

Next step is final submission QA and, if required, presentation/oral-defence preparation. It is not started by this resolution.
