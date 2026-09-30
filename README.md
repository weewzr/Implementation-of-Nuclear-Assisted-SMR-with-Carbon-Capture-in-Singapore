# Nuclear-Assisted SMR + CCS for Singapore

## Current final design
The submission-facing design is the literature-anchored INL/NGNP high-temperature HTGR-assisted steam-methane-reforming process with carbon capture:
- 871 C reformer outlet;
- steam/carbon = 3.0;
- 78.1% methane conversion;
- 88% PSA recovery;
- 925 C INL HTGR case supplying 900 C process heat;
- 130 MMSCFD H2;
- 176.8 MWth nuclear process heat;
- one 600 MWth GTHTR300C-class cogeneration module;
- JAEA one-module/doubled-IHX economics with explicit Singapore electricity-value sensitivities.

Current final-design outputs are under `results/final_design/`.
Canonical manuscript: `paper/main.tex`.

## Reproduce
```bash
sh paper/build.sh
```
This regenerates historical and final-design deterministic datasets, runs the Rust model path used by the manuscript and builds `paper/main.pdf`.

## Scientific interpretation
The final design is evaluated forward against:
- annual lifecycle abatement >0.25 MtCO2e/y;
- forward abatement cost <S$100/tCO2e.

Any passing result is a **conditional model result**, not an observed Singapore commercial project cost.

## Historical research preserved
The repository intentionally retains:
- original Gate-5 fixed-scale 64-case study (0 joint passes);
- Review-1 through Review-5 evidence and resolutions;
- the Review-5 600 C HTTR/mock-up deployment state;
- superseded generated datasets and regression tests.

These materials remain reproducible for audit but are not the operative final submission design.

## Key locations
- final design model: `model/src/final_design.rs`
- historical verified model: `model/src/lib.rs`
- Review-5 deployment model: `model/src/deployment.rs`
- final result/evidence: `results/final_design/`
- historical Gate-5 results: `results/GATE5_RESULTS.md`
- reviews: `reviews/`
- manuscript: `paper/main.tex`
- paper instructions: `paper/README.md`
