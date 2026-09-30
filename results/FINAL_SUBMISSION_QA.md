# FINAL SUBMISSION QA

Status: **IN PROGRESS — final CI/PDF evidence pending.**

## Scientific freeze
The independently verified scientific model is frozen. Final Submission QA changes are presentation, explanation, provenance and current-facing documentation only. No verified model equation, architecture, lifecycle result or economic result was altered.

Verified headline state:
- H2: ~97,946 t/y;
- direct avoided CO2: ~862,094 t/y;
- lifecycle avoided: ~917,139 tCO2e/y;
- candidate lifecycle intensity: ~1.95 kgCO2e/kgH2;
- controlling project electricity revenue: S$0/MWh;
- controlling abatement cost: ~S$3.725/tCO2e;
- CN4252: CONDITIONAL MODEL PASS.

## Independent verification and FDV2 closure
Controlling evidence:
- `reviews/final_design_independent_verification_02_final_rereview.md`;
- FDV2-B01 RESOLVED;
- FDV2-M01 REMAINS RESOLVED;
- FDV2-M02 RESOLVED;
- FDV2-RR-B01 RESOLVED.

FDV2-FR-M01 was the remaining presentation finding. Artifact `11076595823` still showed a small overlap, so a second bounded layout correction was made at `4693b2a5e5f8923f956a9447196a2a441d81ae8d`. Its Research/Paper CI passed and exact-artifact inspection confirmed separated H2/PSA and remaining-capacity branches. FDV2 scientific corrective sequence is therefore closed at the presentation level.

## Accessibility QA
Completed:
- conventional SMR+CCS beginner schematic;
- nuclear-heat substitution schematic;
- dedicated HTGR/IHX isolation schematic;
- final integrated architecture;
- explicit explanation that reactor primary coolant does not enter the chemical process;
- concise HTGR/TRISO/helium explanation;
- plain-language introduction before dense engineering detail;
- threshold graphics and intuitive interpretation.

High-school reader test:
- what SMR does: PASS;
- why it produces CO2 / needs heat: PASS;
- what CCS does: PASS;
- what HTGR/helium/IHX do: PASS;
- why primary coolant remains isolated: PASS;
- meaning of 176.8 MWth: PASS;
- meaning of 0.917 MtCO2e/y and S$3.725/t: PASS;
- CN4252 pass/fail answer: PASS.

## University engineering-rigour QA
The manuscript retains source-backed process conditions, reaction/model equations, thermal duty, helium heat-transfer screen, annualization, lifecycle boundary, economic boundary, abatement-cost equation, threshold test, limitations and reproducibility chain.

Traceability test source -> assumption -> equation/calculation -> result: PASS for the submission-facing final design.

Unit checks explicitly distinguish:
- MWth vs MWe;
- CO2 vs CO2e;
- short ton/day source data vs metric annual tonnes;
- tCO2e/tH2 numerically equal to kgCO2e/kgH2;
- S$/tCO2e.

## Project Source visual inspection
User-provided source images were inspected. Useful equipment/flow concepts include fired reforming, WGS, amine capture, PSA, HTGR helium loops, IHX coupling and nuclear-assisted reforming. Because supplied filenames do not consistently match depicted systems and direct reuse would add attribution/copyright ambiguity, the final manuscript uses original/recreated TikZ schematics with primary-source technical citations.

Detailed provenance: `results/FIGURE_PROVENANCE.md`.

## Citation audit
Material final-design external claims are cited to INL TEV-953/961, Nishihara/JAEA/JAERI, IEAGHG, IEA, UNECE, EMA and MTI as applicable. Final acceptance requires Paper CI to confirm zero undefined citations and references.

## Equation/unit audit
No scientific equations were changed in QA. Context and definitions were strengthened. Nomenclature now explicitly defines CO2e, TRISO, S/C, ROT, MWth, MWe and T&S in addition to SMR, WGS, PSA, CCS, HTGR and IHX.

## Historical-clutter audit
Submission-facing manuscript no longer uses retired 161.92/144.62-MWe project mapping, old electricity-credit economics, the 170/202 source conflation, Gate-5 0/64 results or Review-5 600 C state as current results. Historical evidence remains in GitHub.

## Reproducibility
Canonical path remains `sh paper/build.sh`, with Rust tests, deterministic result generation, latexmk/BibTeX and manuscript integrity checks. Final acceptance evidence will record fresh Research CI, Paper CI and exact artifact.

## Final visual inspection
Pending fresh final QA artifact. Required page-by-page checks:
- all figures/tables/equations/captions readable;
- no clipping/overlap/blank pages;
- repaired architecture remains separated;
- threshold graphics readable;
- bibliography/citations render;
- no obsolete final-design claims appear.

## Acceptance evidence
To be completed after final HEAD workflows:
- final submission commit: pending;
- Research CI: pending;
- Paper CI: pending;
- artifact ID: pending;
- page count: pending;
- exact-artifact visual inspection: pending.
