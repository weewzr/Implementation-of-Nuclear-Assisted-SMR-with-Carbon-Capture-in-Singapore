# FINAL MANUSCRIPT FIGURE PROVENANCE

This record covers submission-facing figures. Figures 1--3 are now included directly from user-provided project artwork under `attachments/`. The artwork provides **visual/design provenance only**; scientific validation remains tied to the cited INL, IEAGHG and JAEA/JAERI/Nishihara sources and to the canonical project model.

| Figure | Active asset | Purpose | Visual provenance | Scientific basis |
|---|---|---|---|---|
| Figure 1 -- conventional SMR + CCS | `attachments/figure1_conventional_smr_ccs.png` | Conventional fired SMR-H2 + CCS reference | User-provided project artwork | IEAGHG merchant-hydrogen architecture; INL conventional SMR evidence |
| Figure 2 -- reference HTGR power pathway | `attachments/figure2_htgr_reference_power.png` | Explain primary-He heat transport and a separate steam/water power cycle | User-provided project artwork | JAEA/JAERI HTGR precedent; orientation only, no project electricity claim |
| Figure 3 -- proposed HTGR-assisted SMR + CCS | `attachments/figure3_proposed_htgr_smr_ccs.png` | Proposed nuclear process-heat integration and chemical train | User-provided project artwork | INL TEV-953/961 process states plus GTHTR300C/JAEA design architecture |

## Attachment scientific-use qualification

The attachment images are conceptual project artwork, not numerical authorities. Embedded artwork labels that differ from the canonical final-design basis are treated as illustrative and are explicitly qualified in the manuscript captions. For Figure 3, the manuscript text/caption controls the scientific interpretation: 600 MWth design basis; 925 C reactor-outlet source case; 900 C secondary-He supply; approximately 466 C return; 78.49 kg/s secondary-He flow; 176.8 MWth reformer duty; 871 C reformer outlet; S/C=3.0; 88% PSA H2 recovery; and 130 MMSCFD H2 product. The approximately 97,946 t/y value is project-derived annual output. The 423.2 MWth remainder is thermal capacity only. The approximately 170 MWth physical-IHX reference and approximately 370/371 MWth source process-heat branch remain distinct. CO2 transport/storage remains conditional.

Any power-side/steam-generator equipment visible in the Figure-3 artwork is source-architecture orientation only and does not establish a project electricity output. Likewise, artwork temperatures/purities that are not part of the canonical basis are not promoted to model inputs.

The previous Rust/TikZ conceptual schematics may remain in generated/reproducibility outputs, but they are **not the active manuscript Figures 1--3**. Quantitative model-derived figures and tables remain deterministically generated where reproducibility matters.

## Targeted-extension figure/data provenance

### A. Rust-generated assets included directly in the manuscript

| Figure/output | Purpose | Generator | Canonical data/model | Publication use |
|---|---|---|---|---|
| Rust thermal-capacity figure | Compare 600 MWth reactor, 370 MWth source heat branch, 176.8 MWth process duty and 423.2 MWth remaining thermal capacity | `model/src/bin/final_design_heat_figure_tex.rs` | constants from `model/src/final_design.rs` | generated TikZ included directly in manuscript |
| Lifecycle ledger CSV | Saved/added lifecycle terms | `model/src/bin/final_design_lifecycle_ledger_csv.rs` | `final_lifecycle_ledger()` | manuscript table via pgfplotstable |
| Cost ledger CSV | Saved/added annual-cost terms | `model/src/bin/final_design_cost_ledger_csv.rs` | `final_cost_ledger()` | manuscript table via pgfplotstable |

Supervisor-code pattern reference: OUTRAM PARK uses dedicated plot-data/snapshot structures feeding `egui_plot::Plot`, `Line`, and `PlotPoints`, with explicit engineering units and display/physics separation. This project adopts the model/data/plot separation and unit discipline, but uses deterministic static SVG/TikZ rather than GUI screenshots.


### B. Rust-generated reproducible/reference assets not directly included

| Asset | Purpose | Generator | Canonical model/data | Manuscript role |
|---|---|---|---|---|
| CN4252 threshold SVG | Deterministic vector check of abatement multiple versus abatement cost/ceiling | `model/src/figures.rs` via `model/src/bin/final_design_figures.rs` using Rust `plotters` SVG backend | `final_design(0.0,false)` | reproducible/reference asset; **not** the threshold graphic included in the active manuscript |
| Visual data CSV | Machine-readable heat/temperature/threshold values | `model/src/bin/final_design_visual_data_csv.rs` | canonical final-design model/constants | provenance/data inspection and independent reproduction |

### C. Manuscript-authored TikZ visualisations using canonical generated values

| Active manuscript visual | Authorship/generator | Data provenance | Status |
|---|---|---|---|
| Annual-abatement threshold graphic | TikZ authored in `paper/sections/07_final_results.tex` | canonical verified lifecycle result (~0.917 MtCO2e/y) and CN4252 0.25-Mt threshold | included in manuscript; not the Plotters SVG |
| Abatement-cost threshold graphic | TikZ authored in `paper/sections/07_final_results.tex` | canonical verified zero-credit cost (~S$3.725/tCO2e) and CN4252 S$100/t ceiling | included in manuscript; not the Plotters SVG |
| Process/isolation schematics | manuscript TikZ | primary INL/JAEA/IEAGHG architecture plus canonical final-design values where labelled | included; technical sources cited in captions/text |

The honest provenance chain is therefore:
- Rust thermal-capacity figure: **figure -> Rust generator -> canonical constants -> primary/source-backed model inputs**.
- Ledger tables: **table -> generated CSV -> Rust ledger function -> final model/source assumptions**.
- Threshold TikZ figures: **manuscript TikZ -> canonical verified result values -> final Rust model**.
- Plotters threshold SVG: **reference SVG -> Rust Plotters generator -> final Rust model**; retained for reproducibility but not rendered in the active paper.


## Deep-feasibility quantitative figures

| Figure asset | Manuscript role | Generator | Data/model dependency | Evidence class |
|---|---|---|---|---|
| `availability_abatement_figure.tex` | availability vs lifecycle-abatement robustness | Rust `deep_feasibility_tikz` | `availability_sensitivity()` and `gas_backup_sensitivity()` | PROJECT-DERIVED VERIFIED SCREENING |
| `ccs_robustness_figure.tex` | captured-stream delivery fraction vs lifecycle abatement | Rust `deep_feasibility_tikz` | `ccs_capture_sensitivity()` | PROJECT-DERIVED VERIFIED SCREENING |
| `co2_bridge_figure.tex` | lifecycle contribution bridge | Rust `deep_feasibility_tikz` | canonical `final_design()` lifecycle ledger | PROJECT-DERIVED VERIFIED SCREENING |
| `cost_bridge_figure.tex` | annual cost-contribution bridge | Rust `deep_feasibility_tikz` | `final_cost_ledger()` | PROJECT-DERIVED VERIFIED SCREENING |
| `availability_abatement.svg` | vector reference asset for availability screen | Rust/Plotters `figures.rs` | same availability model | REPRODUCIBLE REFERENCE ASSET |
| `ccs_robustness.svg` | vector reference asset for CCS robustness | Rust/Plotters `figures.rs` | same CCS model | REPRODUCIBLE REFERENCE ASSET |

The active manuscript uses the generated TikZ assets so labels remain LaTeX-native. The SVGs are retained as independently reproducible vector/reference assets. Both originate from the same Rust model functions; neither is manually redrawn.
