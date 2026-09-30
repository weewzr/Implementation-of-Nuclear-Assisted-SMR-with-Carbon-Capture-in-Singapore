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


## Targeted-extension Rust figures

| Figure/output | Purpose | Generator | Canonical data/model | Publication use |
|---|---|---|---|---|
| Rust thermal-capacity figure | Compare 600 MWth reactor, 370 MWth source heat branch, 176.8 MWth process duty and 423.2 MWth remaining thermal capacity | `model/src/bin/final_design_heat_figure_tex.rs` | constants from `model/src/final_design.rs` | generated TikZ included directly in manuscript |
| CN4252 threshold SVG | Static vector publication plot of abatement multiple versus abatement cost/ceiling | `model/src/figures.rs` via `model/src/bin/final_design_figures.rs` using Rust `plotters` SVG backend | `final_design(0.0,false)` | deterministic SVG reference/publication asset |
| Visual data CSV | Machine-readable heat/temperature/threshold values | `model/src/bin/final_design_visual_data_csv.rs` | canonical final-design model/constants | provenance/data inspection |
| Lifecycle ledger CSV | Saved/added lifecycle terms | `model/src/bin/final_design_lifecycle_ledger_csv.rs` | `final_lifecycle_ledger()` | manuscript table via pgfplotstable |
| Cost ledger CSV | Saved/added annual-cost terms | `model/src/bin/final_design_cost_ledger_csv.rs` | `final_cost_ledger()` | manuscript table via pgfplotstable |

Supervisor-code pattern reference: OUTRAM PARK uses dedicated plot-data/snapshot structures feeding `egui_plot::Plot`, `Line`, and `PlotPoints`, with explicit engineering units and display/physics separation. This project adopts the model/data/plot separation and unit discipline, but uses deterministic static SVG/TikZ rather than GUI screenshots.
