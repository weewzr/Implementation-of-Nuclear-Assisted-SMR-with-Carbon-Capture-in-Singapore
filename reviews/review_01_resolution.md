# Independent Review 1 resolution record

## Provenance note

The requested path `reviews/review_01_research_foundation.md` was absent from the
GitHub default branch when the Main Research review-response pass began.
GitHub code search also returned no copy. The substantive review was available
in the CN4252 Project conversation and was used as peer-review input. This file
records the response; it does not pretend the missing repository copy existed.

## Decision

Review 1 gate: **OPEN**.

Detailed integrated nuclear modelling remains prohibited until the remaining
valid blockers below satisfy their acceptance criteria.

## Finding dispositions

### B1 — Primary-reformer material control volume not closed
**Disposition: PARTIALLY ACCEPTED, material-balance component now RESOLVED.**

The reviewer was correct that the then-current two-row stream comparison was not
a closed control volume. The stronger source-level diagnosis is now established:
IEAGHG stream 4 is feed to the pre-reformer, while stream 5 is HTS inlet after
pre-reforming, a second HP-steam addition, BFW desuperheating, primary reforming
and the reformer waste-heat boiler.

The rounded source rows independently imply about 154.4 kmol/h aggregate H2O
addition between streams 4 and 5. Including this source-described but unnumbered
water/steam addition closes C/H/O to <0.1% of stream-4 elemental inventories
without tuning carbon species.

Evidence and equations: `equations/source_stream_reconstruction.md`.
Verification: new Rust regression test.

The *energy* control volume is not closed and remains a separate blocker.

### B2 — Full temperature-resolved reformer/furnace energy balance absent
**Disposition: ACCEPTED — OPEN BLOCKER.**

The existing 298 K reaction-duty layer is explicitly not furnace duty. A
temperature-dependent enthalpy/steam/heat-recovery model and an authoritative
plant-duty benchmark remain required.

Acceptance criterion remains: close the conventional energy balance and
reproduce an authoritative reference energy/duty metric within a declared
tolerance before nuclear heat substitution.

### B3 — PSA-tail-gas disposition unresolved when fired reformer is removed
**Disposition: PARTIALLY RESOLVED — carbon ledger verified by CI; inert/purge closure remains.**

The furnace-free candidate now has an explicit reduced-CHO disposition rather
than deleting the conventional PSA tail gas. Existing tail CO2 is routed to the
high-pressure process-carbon capture train; H2/CO/CH4 are returned to the
reforming loop; converted CO/CH4 carbon is routed to capture; uncaptured fresh
feed carbon leaves as residual process CO2. Recycled carbon is treated as an
internal flow, not a new external carbon source.

A new `ConvergedCarbonLedger` closes external fresh-feed carbon exactly between
captured and residual-emitted carbon, and a separate function accounts for the
tail carbon delivered to capture. The remaining limitation is explicit: N2 and
other inerts are outside the current CHO reduced model, so a purge cannot yet be
sized. B3 will be marked fully resolved only after the strengthened
reformer/shift/recycle model retains or explicitly bounds that purge requirement.

Evidence: `model/src/lib.rs`, commit `8b5c40c25afd56a78f5c680dd99cd3e406e74108`
plus compile fix `27add5ade930a60a7444c63f38f63fefb9b3ab22`; Rust CI run 36524925119 passed.

### B4 — Capture topology cannot be fixed before carbon architecture closes
**Disposition: ACCEPTED — OPEN BLOCKER, dependent on B3.**

No universal amine capture fraction will be imposed. Shifted syngas, PSA-tail
gas and flue-gas capture have materially different pressure/composition and
solvent/energy requirements.

### B5 — Functional unit/common comparison boundary needs freezing
**Disposition: RESOLVED.**

The common basis is now frozen to the authoritative IEAGHG merchant-plant
reference: 100,000 Nm3/h = 8,994 kg/h H2, purity >99.9%, with the PSA equipment
list specifying 2.58/2.51 MPa on the H2 side. The canonical plant-gate product
pressure is therefore 2.51 MPa, before any downstream merchant/pipeline
compression. Specific results use 1 kg H2 on this product basis; annual results
scale only after the specific calculation.

The model now also encodes strict nested boundaries:
ProcessGate < PlantGate < Lifecycle. Process-gate chemistry/heat is not silently
mixed with plant utilities; lifecycle adds upstream NG, nuclear-LCA and CCS-chain
burdens outside plant gate.

Evidence: IEAGHG 2017-02 base-case heat/material and equipment tables; Rust
`COMMON_H2_BASIS` and `SystemBoundary` tests, commit
`d9cac760df2a534c1d6a5039b0ec72f35cf8c150`.

### M1 — CCS literature needs stream-specific treatment
**Disposition: ACCEPTED.**
Resolve with B4; do not create a generic MEA penalty.

### M2 — Nuclear heat-integration literature needs quantitative secondary-loop/HX constraints
**Disposition: ACCEPTED.**
Resolve after conventional heat-grade demand is known.

### M3 — Nuclear technology should remain temperature-envelope based
**Disposition: ACCEPTED.**
No reactor vendor/product will be selected as a model prerequisite.

### M4 — Economic accounting framework incomplete
**Disposition: ACCEPTED.**
The incremental-abatement-cost equation is sound; currency year, annualisation,
capacity factor, energy prices, nuclear allocation and CCS transport/storage
basis remain to be frozen before assignment-level cost results.

### M5 — Current tests emphasize reconstruction more than predictive validation
**Disposition: PARTIALLY RESOLVED.**
The model now includes independent thermodynamic checks beyond source-row
reconstruction: NIST/JANAF SMR/WGS equilibrium constants, pressure-dependence
limiting tests, an IEAGHG HTS Q/K diagnostic, apparent-equilibrium-temperature
reconstruction, analytical-vs-iterative recycle fixed-point verification, and
CI tests for pressure/compression/heat bounds. This is meaningful validation,
but the primary reformer has not yet been reproduced on an authoritative
once-through state; therefore M5 is not closed.

### M6 — Thermodynamic property layer missing
**Disposition: SUBSTANTIALLY RESOLVED, retained under B2 until energy closure.**
The Rust model now contains NIST Shomate enthalpy and entropy functions for the
major CH4/H2O/CO/CO2/H2 species, temperature-dependent sensible enthalpies,
IAPWS saturation calculations used for steam/condensation bounds, and
NIST/JANAF-derived Gibbs-energy/equilibrium constants for SMR and WGS. The
remaining property work is tied specifically to closing B2's conventional
energy balance, not absence of a thermodynamic layer.

### Minor scope warning
**Disposition: ACCEPTED.**
Publication-grade extensions must not displace the CN4252 critical path.

## Sound work preserved

No rewrite is required for the assignment matrix, research question, competing
hypotheses, comparator set, CN4252 scaling identity, capture-versus-avoidance
distinction, plant-gate/lifecycle distinction, IEAGHG carbon reconstruction,
HTS verification, PSA-tail-gas finding, reduced-model diagnostic, Singapore
future-scenario framing, or existing Rust regression tests.

## Next blocker

B2: reconstruct and validate the conventional SMR energy balance and temperature
grades. Do not attach nuclear heat to the 298 K reaction-duty layer.
