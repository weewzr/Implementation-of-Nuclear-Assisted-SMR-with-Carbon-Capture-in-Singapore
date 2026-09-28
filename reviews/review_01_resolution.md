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
**Disposition: ACCEPTED — OPEN BLOCKER.**

The existing carbon reconstruction demonstrates why this is real: conventional
PSA tail gas carries feed-derived H2/CO/CH4/CO2 and is normally used as furnace
fuel. Deleting the furnace does not delete this carbon or chemical energy.

Acceptance criterion remains: every tail-gas species has a physical destination
for each candidate nuclear architecture and total carbon closes.

### B4 — Capture topology cannot be fixed before carbon architecture closes
**Disposition: ACCEPTED — OPEN BLOCKER, dependent on B3.**

No universal amine capture fraction will be imposed. Shifted syngas, PSA-tail
gas and flue-gas capture have materially different pressure/composition and
solvent/energy requirements.

### B5 — Functional unit/common comparison boundary needs freezing
**Disposition: ACCEPTED — resolution required before comparative results.**

Canonical modelling basis will be 1 kg H2 at a declared plant-gate purity and
pressure, with annual scaling after specific results. Process-gate and lifecycle
boundaries must remain nested and separately reported. Product pressure still
requires selection from an authoritative industrial basis before this finding
is marked closed.

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
**Disposition: ACCEPTED.**
Existing provenance/regression tests are retained. Independent duty/model
validation must be added rather than replacing them.

### M6 — Thermodynamic property layer missing
**Disposition: ACCEPTED.**
This is part of B2.

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
