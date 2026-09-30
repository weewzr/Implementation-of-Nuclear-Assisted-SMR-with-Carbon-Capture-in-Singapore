# Nuclear-Assisted SMR + CCS for Singapore

## Final design verified
The submission-facing design is the literature-anchored INL/NGNP high-temperature HTGR-assisted steam-methane-reforming process with carbon capture:
- 871 C reformer outlet; steam/carbon = 3.0; 78.1% methane conversion; 88% PSA recovery;
- 925 C INL HTGR case supplying 900 C process heat;
- 130 MMSCFD H2 source service;
- 176.8 MWth nuclear process heat;
- one 600 MWth GTHTR300C-class cogeneration module;
- JAEA doubled-IHX/secondary-loop economic sensitivity.

Independent final-design verification: `reviews/final_design_independent_verification.md`.
Final verified headline result: approximately 0.917 MtCO2e/y lifecycle abatement; zero-value-cogeneration cost approximately S$30.2/tCO2e. This is a **conditional model result**, not observed Singapore commercial feasibility.

## Reproduce
```bash
sh paper/build.sh
```
This regenerates historical and final-design deterministic datasets, runs the Rust model path used by the manuscript and builds `paper/main.pdf`.

## Governing distinction
**FINAL SUBMISSION = FINAL BEST-SUPPORTED DESIGN.**

**REPOSITORY = COMPLETE SCIENTIFIC AUDIT TRAIL.**

Historical Gate-5 0/64 results, the Review-5 600 C state, Reviews 1--5 and their resolution records remain preserved for audit but are not the operative final submission design.

## Key locations
- final design model: `model/src/final_design.rs`
- independent verification: `reviews/final_design_independent_verification.md`
- final result/evidence: `results/final_design/`
- historical model paths: `model/src/lib.rs`, `model/src/deployment.rs`
- historical Gate-5 results: `results/GATE5_RESULTS.md`
- canonical manuscript: `paper/main.tex`
- paper build instructions: `paper/README.md`
