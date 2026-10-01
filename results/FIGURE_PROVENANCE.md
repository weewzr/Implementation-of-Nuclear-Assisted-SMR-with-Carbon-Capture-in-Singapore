# FINAL MANUSCRIPT FIGURE PROVENANCE

This record covers submission-facing figures. User-provided Project Source images were inspected as technical references during Final Submission QA; they were not copied directly because the supplied filenames/depicted systems were not consistently aligned and original schematics provide clearer provenance.

| Figure | Purpose | Status | Technical basis | Manuscript citation | Generated-data dependency |
|---|---|---|---|---|---|
| Singapore energy context | Establish Singapore energy/emissions context | Original data panel | EMA Singapore Energy Statistics | EMA citations in caption | `data/external/ema_singapore_energy_context_2024.csv` |
| Conventional SMR + CCS explainer | Teach feed -> reformer -> WGS -> capture -> PSA -> H2 and CCS | Original/recreated schematic | IEAGHG merchant-hydrogen process architecture; user Project Source conventional-SMR visual inspected as reference | `ieaghg2017smr` | none |
| Nuclear heat substitution | Show what changes relative to fired reforming | Original/recreated schematic | INL TEV-961 + JAEA/JAERI HTGR precedent; user Project Source nuclear-assisted visual inspected as reference | `inltev961`, `jaeri2004gthtr300c` | none |
| HTGR/IHX isolation | Explain primary helium / IHX / secondary helium isolation | Original/recreated schematic | JAEA/JAERI HTGR/IHX architecture; Project Source HTGR loop images inspected as reference | `jaeahttr`, `jaeri2004gthtr300c`, `nishihara2007potential` | none |
| Final integrated architecture | Show verified final process and remaining thermal capacity without power claim | Original project schematic | INL TEV-953/961 + Nishihara 2007 architecture | `inltev953`, `inltev961`, `nishihara2007potential` | none |
| Annual-abatement threshold | Make 0.917 versus 0.25 Mt/y immediately visible | Original project-result graphic | Independently verified final lifecycle result | model/review provenance in text | frozen final result |
| Abatement-cost threshold | Make S$3.725 versus S$100/t immediately visible | Original project-result graphic | Independently verified zero-credit economic result | model/review provenance in text | frozen final result |

## Project Source visual inventory

Useful technical-reference content observed in supplied images:
- fired SMR furnace, WGS, amine absorber/stripper, PSA and offshore CO2-storage pathway;
- HTGR core, helium circulator, high-temperature helium loop and heat exchanger/steam-generation concepts;
- proposed HTGR-assisted reformer with primary/secondary heat-transfer architecture.

These were used to check equipment ordering and explanatory needs, not as numerical authorities. Primary INL/JAEA/IEAGHG literature remains the technical citation basis. No supplied raster figure is reproduced directly in the final manuscript.


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
| `availability_cost.svg` | vector reference asset for cost robustness | Rust/Plotters `figures.rs` | same availability/backup model | REPRODUCIBLE REFERENCE ASSET |
| `ccs_robustness.svg` | vector reference asset for CCS robustness | Rust/Plotters `figures.rs` | same CCS model | REPRODUCIBLE REFERENCE ASSET |

The active manuscript uses the generated TikZ assets so labels remain LaTeX-native. The SVGs are retained as independently reproducible vector/reference assets. Both originate from the same Rust model functions; neither is manually redrawn.
