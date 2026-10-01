# Nuclear-Assisted SMR + CCS for Singapore

## Current final state

**FINAL TECHNICAL-REPORT / NUMERICAL-PROVENANCE PRESENTATION PASS — COMPLETE**  
**STRICT EQUATION / CITATION COMPLIANCE — COMPLETE**  
**REVIEW 07 — RESOLVED**  
**CONDITIONAL MODEL PASS — RETAINED**

The scientific research/review critical path is closed through Review 07. Current work is submission packaging only; historical research states remain preserved below as the audit trail.

### Canonical current pointers
- Current status and verified PDF artifact: `STATUS.md`
- Current manuscript: `paper/main.tex`
- Current final model: `model/src/final_design.rs`
- Current number provenance: `results/final_design/MASTER_NUMBER_PROVENANCE_REGISTER.md`
- Current assumptions: `results/final_design/ASSUMPTIONS.md`
- Current sources: `results/final_design/SOURCE_REGISTER.md`
- Current original-plan traceability: `results/FINAL_ORIGINAL_PLAN_TRACEABILITY.md`
- Current final independent review: `reviews/review_07_deep_feasibility.md`
- Current Review-07 resolution: `reviews/review_07_resolution.md`
- Reproducibility workflow: `paper/build.sh` and GitHub Actions Paper/reproducibility CI

### Final verified design and result
- INL process: 871 C reformer outlet; 925 C reactor outlet; 900 C supplied process heat.
- Hydrogen service: 130 MMSCFD; approximately **97,946 t/y** at 85% availability.
- Process heat: **176.8 MWth**.
- Selected hardware/economic architecture: **600 MWth GTHTR300C-class reactor** with a **370 MWth source heat/IHX branch**.
- Remaining reactor thermal capacity: **423.2 MWth**.
- **No exact project gross/net/export electricity output is claimed.**
- Controlling project electricity revenue: **S$0/MWh**.
- Direct avoided CO2: approximately **862,094 t/y**.
- Lifecycle avoided emissions: approximately **917,139 tCO2e/y**.
- Candidate lifecycle intensity: approximately **1.95 kgCO2e/kgH2**.
- Controlling abatement cost: approximately **S$3.725/tCO2e**.
- CN4252 thresholds: **CONDITIONAL MODEL PASS**.

The principal limitations are design-study rather than constructed-project economics, screening-level reactor/process integration, a derived nuclear process-heat lifecycle proxy, and conditional cross-border CO2 transport/storage.

## Canonical current evidence
- Current status: `STATUS.md`
- Canonical manuscript source: `paper/main.tex`
- Canonical final PDF: generated from `paper/main.tex`; final verified artifact is recorded in `STATUS.md`
- Final-design model: `model/src/final_design.rs`
- Final-design results/assumptions/source register: `results/final_design/`
- Quantitative-foundation independent verification: `reviews/final_design_independent_verification_02_final_rereview.md`
- Final independent review: `reviews/review_07_deep_feasibility.md`
- Final review resolution: `reviews/review_07_resolution.md`
- Prior Final Submission QA baseline: `results/FINAL_SUBMISSION_QA.md`
- Targeted-extension evidence/verification: `results/TARGETED_EXTENSION_EVIDENCE.md`
- Figure/source provenance: `results/FIGURE_PROVENANCE.md`
- Assignment brief: `PROJECT_BRIEF.md`

## Reproduce
From repository root:

```bash
sh paper/build.sh
```

This runs the deterministic model/test/data-generation path used by the manuscript and builds `paper/main.pdf`. See `paper/README.md` for requirements and provenance.

## Historical audit trail
**FINAL SUBMISSION = FINAL BEST-SUPPORTED DESIGN.**

**REPOSITORY = COMPLETE SCIENTIFIC AUDIT TRAIL.**

The repository intentionally preserves superseded scientific states, including:
- Gate-5 64-case / 0-joint-pass screening;
- Review-5 600 C deployment work;
- Reviews 1–5 and their resolution records;
- earlier economic/lifecycle/sensitivity studies;
- FDV2 initial findings, corrective records and re-reviews;
- historical model/equation/result files.

These records document what was known at earlier research stages. They must not be interpreted as the operative final result unless a current-facing file explicitly points to them for provenance.
