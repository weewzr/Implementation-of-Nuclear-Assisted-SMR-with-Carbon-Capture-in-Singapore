# STATUS

## Current state
Reviews 1-4: CLOSED.
Gate 5 original experiments/results: COMPLETE and preserved.
Gate 6 original paper/reproducibility: COMPLETE.
Review-4 corrections: COMPLETE.
Bounded visual-communication pass: COMPLETE.
Final Submission QA: superseded before freeze by one user-authorised bounded scientific extension.
Singapore deployment-scale techno-economic extension: IN PROGRESS.

## Scientific lock
The original canonical Gate-5 result is unchanged:
- 64 coupled nuclear-assisted cases at approximately 74.85 ktH2/y;
- 0/64 joint CN4252 passes;
- conservative case fails both thresholds.
No original parameter range has been altered to obtain a pass.

## Deployment-scale extension
A separate post-canonical study is testing larger useful H2/process-heat/CCS deployment against the same CN4252 thresholds using JAEA/JAERI mature-design economics, INL/GAIN modern HTGR cost evidence and IEAGHG CCS capital evidence.
The extension is implemented separately in `model/src/deployment.rs` and manuscript Section `Singapore deployment-scale techno-economic extension`.
Current forward result encoded by regression tests: a conditional JAEA mature-design cogeneration case can jointly pass at the larger deployment scale; the same JAEA module charged hydrogen-only and the modern-central/FOAK-adverse cogeneration screens fail the cost threshold. This remains subject to final CI/PDF verification.

## Canonical locations
- Original model: `model/src/lib.rs`.
- Deployment extension: `model/src/deployment.rs`.
- Original Gate-5 results: `results/GATE5_RESULTS.md`, `results/r3_canonical_results.md`.
- Deployment literature basis: `results/deployment/LITERATURE_BASIS.md`.
- Manuscript: `paper/main.tex`.
- Reproduction: `sh paper/build.sh`.
- Review records: `reviews/`.

## Next step
Complete only the bounded deployment-scale extension: verify tests/generated data, inspect the generated tables/figures and actual PDF, record the extension result and CI evidence, then STOP. Do not begin another review automatically and do not mark FINAL SUBMISSION CANDIDATE: READY automatically.
