# Gate 5 closure — Experiments and Results

## Decision

**Gate 5 — experiments/results: CLOSED.**

Closure means the verified Gate-4 model has been exercised sufficiently to
answer the current CN4252 research questions within its declared screening
scope. It does not mean the nuclear-assisted configuration satisfies the
assignment thresholds.

## Repository state assessed

Latest Gate-5 state before closure:
`8ec3a4c09d5608f537d3f1f40f7748d289e8d2ad`.

GitHub Actions run `36555452744`: PASS.

## Acceptance assessment

### Threshold behaviour — PASS
Experiment 01 and the 64-case uncertainty design establish the joint CN4252
threshold topology. Result: **0/64 joint passes**.

### Driver attribution — PASS
Experiment 02 quantifies local effects of reformer temperature/pressure, PSA
recovery, capture fraction, carbon/electricity conditions and heat/fixed-cost
conditions on annual abatement and S$/t.

### Binding-constraint diagnosis — PASS
Experiment 04 partitions the complete tested domain into abatement-only,
cost-only, both-fail and joint-pass classes and quantifies normalized threshold
gaps.

### Falsification/comparator evidence — PASS to available evidence
Experiment 03 places IEAGHG Case 1A on the same H2/output-hours scale and shows
that it exceeds 0.25 Mt/y direct avoided CO2. Its Singapore S$/t is correctly
left unresolved because source non-T&S cost and Singapore T&S scenario inputs
lack a common validated currency/year basis.

eSMR/electrolysis remain explicitly data-limited rather than filled with
fabricated common-boundary economics.

### Conservative-case falsification — PASS
The conservative credible nuclear case produces non-positive lifecycle
abatement and therefore fails both assignment thresholds. The result is
preserved rather than optimized away.

### Singapore deployment interpretation — PASS
Annual process scale is derived from the corrected model. Nuclear deployment
and cross-border CO2 storage remain scenario conditions, not assumed existing
Singapore infrastructure.

### Reproducibility — PASS
Experiments 01-04, canonical CSV renderers, synthesis, binding counts and
figure-data manifest are deterministic Rust outputs. The generation script
`model/scripts/generate_gate5_results.sh` regenerates the canonical Gate-5
result datasets.

## Scientific result entering Gate 6

The current evidence does **not** support the claim that nuclear-assisted
SMR+CCS satisfies both CN4252 requirements within the tested evidence-backed
domain.

Specifically:
- 64 coupled nuclear cases were tested;
- 0 pass both >0.25 MtCO2e/y and <S$100/tCO2e;
- the conservative credible case fails both thresholds;
- conventional IEAGHG Case 1A exceeds the annual direct-abatement scale on the
  common H2 production basis;
- no fair Singapore economic result currently establishes Case 1A, eSMR or
  electrolysis as a definitive winner either.

Therefore the defensible conclusion is comparative and conditional, not a
technology recommendation.

## Claim-strength boundary

The 0/64 result is a verified experimental result **for the declared parameter
domain**. It is not proof that all conceivable nuclear-assisted SMR+CCS
configurations fail.

Economic results remain scenario-dependent where unit prices/CAPEX/T&S are
assumptions. Reactor deployment and cross-border storage remain deployment
conditions.

## Next milestone

Gate 6 — paper/reproducibility.

Gate 6 should synthesize the verified mathematics, model validation,
experiments, adverse findings, comparator evidence and limitations into the
canonical LaTeX manuscript and reproducibility package. It should not reopen
completed model work merely to obtain a more favourable conclusion.
