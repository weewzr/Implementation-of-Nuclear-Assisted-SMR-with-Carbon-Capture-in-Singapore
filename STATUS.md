# STATUS

## FINAL CURRENT DESIGN
**FINAL DESIGN REFINEMENT: COMPLETE pending final CI/PDF verification.**

The current submission-facing design is the literature-anchored INL/NGNP high-temperature HTGR-assisted SMR+CCS process:
- 871 C reformer outlet; S/C 3.0; 78.1% methane conversion; 88% PSA recovery;
- INL Case-6 925 C reactor outlet / 900 C process heat;
- 130 MMSCFD H2 source plant;
- 176.8 MWth nuclear process heat;
- one 600 MWth GTHTR300C-class cogeneration module;
- conservative JAEA doubled-IHX/secondary-loop economic sensitivity because final duty exceeds the 170 MWth reference IHX.

Current final-design result/evidence: `results/final_design/`.
Current final-design model: `model/src/final_design.rs`.
Canonical manuscript: `paper/main.tex`.

Headline model result: approximately 0.917 MtCO2e/y lifecycle abatement. The mature doubled-IHX/S$150-MWh scenario gives a negative screened abatement cost because cogenerated electricity plus natural-gas savings exceed incremental annual cost; zero-value cogeneration remains below S$100/t in the deterministic screen. Any joint pass is a **CONDITIONAL MODEL RESULT**, not observed Singapore commercial feasibility.

## PRESERVED HISTORICAL RESULTS
These remain reproducible but are superseded for submission-facing design:
- Gate-5 fixed-scale study: approximately 74.85 ktH2/y, 64 cases, 0 joint passes;
- Review-5 600 C HTTR/mock-up deployment state: non-positive lifecycle abatement;
- Reviews 1-5 and all resolution records.

Historical evidence is not deleted or rewritten.

## Literature hierarchy
- final nuclear-SMR process: INL TEV-953 / TEV-961;
- HTGR/IHX hardware and helium architecture: JAEA/JAERI GTHTR300C/HTTR;
- one-module cogeneration economics: Nishihara et al. GTHTR300C;
- CCS: IEAGHG;
- Singapore electricity/CCS context: EMA/MTI.

## Verification
Starting commit for this consolidation: `255fe02061b7660b6cc8384881ee4d2e84e3654b`.
Final-design CI/PDF evidence will be recorded after the current workflows complete.

## Next step
Complete only deterministic final-design CI, Paper CI and page-by-page PDF inspection. Then set **FINAL DESIGN REFINEMENT: COMPLETE** and STOP.

Do not begin Review 6. Do not begin Final Submission QA automatically.
