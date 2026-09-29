# Independent Review 2 — Mathematical and computational model

## Scope and provenance

Review target: current `main` model state after Independent Review 1 closure.
This review examines mathematical closure, thermodynamic consistency, numerical
solver behaviour, conservation, validation logic and whether model outputs are
physically supported. It does not repeat the literature/research-foundation
review and does not reopen Review 1.

## Decision

**Review 2 gate: OPEN — BLOCKERS PRESENT.**

The current repository contains substantial new full-species/equilibrium work,
but the new recycle solver fails its own CI convergence tests. Legacy reduced
recycle results therefore remain screening diagnostics and must not be promoted
to validated predictions.

## Findings

### R2-B01 — Full-species recycle steady-state solver fails acceptance cases
**Disposition: RESOLVED.**

The new `solve_full_recycle6` couples a six-species wet-gas state, SMR
conversion, equilibrium WGS, composition-sensitive PSA recovery, inert purge and
fixed-H2 product feedback. This is directionally the correct strengthening over
the legacy CHO fixed-point model.

However Rust CI run 36533494216 fails:
- `finite_purge_solution_is_nonnegative_when_converged`;
- `fresh_feed_depends_on_reformer_and_psa_performance`.

The first failure means the nominal finite-purge case does not reach the stated
steady-state tolerance. The second means both contrasting reformer/PSA cases do
not converge, so the intended physical dependence cannot yet be demonstrated.

The unstable simultaneous proportional controller has been superseded by a
nested numerical formulation. The inner solve closes recycle composition at
fixed fresh feed; the outer scalar bisection solves the fixed-H2 production
residual. The inert purge is imposed from the steady-state balance
purge_N2 = fresh external N2, eliminating the unphysical zero-purge switch.

The new nominal and bracketing cases converge. CI verifies fixed-H2 product
closure, nonnegative species, exact inert balance, C/H/O/N conservation through
SMR and WGS steps, and sensitivity of fresh-feed demand to reformer/PSA
performance. The old proportional-controller tests were retired only after the
new acceptance tests passed; their tolerances were not weakened.

Evidence: commits `33b9bba5f3b8f229502099758d4b5b5eb7a8d295`,
`331d231afe4a325b338f86b2b4f35f650f6ca49b`,
`5e3fd0e5f33d79c8feaec05b21f9ef8ef19298a5`; CI run 36533912202 passed.

### R2-B02 — Legacy analytical recycle fixed point is structurally purge-blind
**Severity: BLOCKER for any result using the ~0.737 fresh-feed fraction.**

The legacy analytical result
`analytical_tail_recycle_fresh_ng_fraction()` has no purge argument and is
algebraically invariant to all positive CO/CH4 conversion and H2-recovery
coefficients. The later purge/carbon ledger removes CO/CH4 but does not feed
those losses back into fresh-feed demand or fixed-H2 production.

This is mathematically self-consistent only for the reduced no-purge surrogate;
it is not a closed physical recycle prediction once Review-1 inert control is
required.

**Acceptance criterion:** retire the legacy 0.737 result from predictive
lifecycle/energy/economic propagation, or reproduce it as a limiting case of a
converged purge-aware full-species solver. All headline recycle predictions must
come from the physically closed solver.

### R2-B03 — Thermodynamic layer is not yet coupled to SMR conversion
**Disposition: RESOLVED for the ideal-equilibrium screening model.**

The model now has NIST/JANAF equilibrium constants and an equilibrium WGS solve,
but `solve_full_recycle6` still takes `smr_conversion` as an arbitrary input.
Thus reformer chemistry is only partially thermodynamically constrained.
Numerical convergence would not by itself validate the reformer.

An ideal-gas SMR equilibrium extent solver now closes ln(Q/K)=0 while
conserving C/H/O/N. Coupled SMR/WGS equilibrium sweeps close both reaction
residuals at declared reformer T/P. The formulation is checked first against
the source-limited IEAGHG hot-product composition over 900-950 C; this remains
a thermodynamic compatibility diagnostic because the exact primary-reformer
inlet state is not published. The independently validated radiant-duty envelope
from Review 1 remains the energy-side once-through benchmark.

The same equilibrium formulation is now coupled into the nested full-species
recycle solver. No arbitrary `smr_conversion` appears on this predictive path.
CI verifies convergence at the 900 C reference screen and a physically
non-invariant fresh-feed response when reformer temperature changes to 950 C.

This resolves the mathematical blocker for an ideal-equilibrium screening
model; it does not claim kinetic/catalyst validation.

Evidence: commits `66fbcaaf51f59b3fa6d7308a67ee9dd239f883ea`
and `aab7779e81a462bd02eb65f84c40a552b542b0be`; CI passed.

### R2-M01 — PSA model is a bounded surrogate, not adsorption validation
**Severity: MAJOR.**

`psa6_bounded` conserves H2 but imposes pure-H2 product and a linear impurity
penalty with an arbitrary sensitivity parameter. This is useful for sensitivity,
not predictive PSA performance.

**Acceptance criterion:** either calibrate/validate the reduced PSA response
against authoritative multi-condition data, or explicitly bound recycle
conclusions across a justified recovery/selectivity envelope and label them
screening results.

### R2-M02 — Review-1 radiant validation is source-limited, not unique state validation
**Severity: MAJOR / limitation, not a reopened Review-1 blocker.**

The <5% nearest-envelope agreement is a useful independent duty check, but the
600-700 C inlet interval is an uncertainty envelope because the exact primary
reformer inlet is unavailable. The test establishes consistency with the
authoritative radiant metric, not uniqueness of the reconstructed state.

**Acceptance criterion:** preserve this interpretation in downstream model
validation; do not describe it as reproducing the exact IEAGHG reformer state.

### R2-M03 — STATUS contains stale contradictory gate/task statements
**Severity: MAJOR reproducibility issue.**

`STATUS.md` begins with `Independent Review 1 gate: CLOSED` but later still
states `Independent Review 1 gate OPEN` and describes obsolete recycle tasks
and pending-CI states. This can mislead future work selection.

**Acceptance criterion:** after scientific blockers are addressed, reduce
`STATUS.md` to the canonical current gate, verified results, blockers and next
scientific task without deleting provenance held elsewhere.

## Sound work retained

- Source-stream C/H/O reconstruction and interstage-water diagnosis.
- Temperature-dependent NIST/IAPWS property layer.
- IEAGHG radiant-duty and WHB validation as source-limited consistency checks.
- WGS equilibrium/Q/K diagnostics and limiting-case tests.
- Explicit N2 purge requirement and stream-specific CCS topology.
- Common H2 product basis and nested system boundaries.
- Analytical-vs-iterative verification of the legacy reduced model as a
  mathematical surrogate (not physical validation).

## Required next action

Resolve **R2-B02** next by removing the legacy purge-blind ~0.737 fresh-feed
fraction from lifecycle/energy/economic predictive propagation and replacing it
with outputs from the thermodynamically constrained nested full-species solver.
Then address R2-M01 PSA uncertainty before closing Review 2.
