# FINAL SUBMISSION QA

Status: **PASS — FINAL SUBMISSION QA COMPLETE.**

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
Material final-design external claims are cited to INL TEV-953/961, Nishihara/JAEA/JAERI, IEAGHG, IEA, UNECE, EMA and MTI as applicable. Final Paper CI confirmed zero undefined citations and zero undefined references.

## Equation/unit audit
No scientific equations were changed in QA. Context and definitions were strengthened. Nomenclature now explicitly defines CO2e, TRISO, S/C, ROT, MWth, MWe and T&S in addition to SMR, WGS, PSA, CCS, HTGR and IHX.

## Historical-clutter audit
Submission-facing manuscript no longer uses retired 161.92/144.62-MWe project mapping, old electricity-credit economics, the 170/202 source conflation, Gate-5 0/64 results or Review-5 600 C state as current results. Historical evidence remains in GitHub.

## Reproducibility
Canonical path remains `sh paper/build.sh`, with Rust tests, deterministic result generation, latexmk/BibTeX and manuscript integrity checks. Final Research CI, Paper CI and exact-artifact evidence are recorded below.

## Final visual inspection
PASS on exact artifact `11078290219`.
- 13 pages inspected page by page;
- no clipping, node overlap or accidental blank pages;
- repaired final architecture remains separated and readable;
- conventional-SMR, nuclear-heat and IHX-isolation teaching figures are readable at normal page scale;
- final results table and CN4252 threshold graphics fit and are legible;
- equations and captions render correctly;
- bibliography is present and the final long MTI URL wraps within the page after the final typography correction;
- no obsolete 0/64, Review-5 600 C, 170/202 source conflation, 161.92/144.62-MWe mapping or electricity-credit economics is presented as current.

Two-reader test:
- technically curious high-school reader conceptual test: PASS;
- university engineering-marker traceability test: PASS.

## Acceptance evidence
- final manuscript QA HEAD: `7e4dbb7912eedefb9c91b07112f2cc76ee79137a`;
- Research CI `36670713040`: PASS;
- Paper/reproducibility CI `36670713033`: PASS;
- PDF artifact: `11078290219`;
- PDF page count: 13;
- bibliography convergence: PASS (`main.bbl` and `references.bib` detected; latexmk targets up to date);
- undefined citations: 0;
- undefined references: 0;
- exact-artifact visual inspection: PASS;
- independent verification: `reviews/final_design_independent_verification_02_final_rereview.md`;
- figure provenance: `results/FIGURE_PROVENANCE.md`.

All Final Submission QA acceptance criteria are satisfied.
