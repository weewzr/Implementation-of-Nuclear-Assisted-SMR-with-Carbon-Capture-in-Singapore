# Independent Review 1 resolution record

## Provenance note

The requested path `reviews/review_01_research_foundation.md` was absent from the
GitHub default branch when the Main Research review-response pass began.
GitHub code search also returned no copy. The substantive review was available
in the CN4252 Project conversation and was used as peer-review input. This file
records the response; it does not pretend the missing repository copy existed.

## Decision

Review 1 gate: **CLOSED**.

Detailed integrated nuclear modelling remains prohibited until the remaining
valid blockers below satisfy their acceptance criteria.

## Finding dispositions

### B1 — Primary-reformer material control volume not closed
**Disposition: RESOLVED.**

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

The associated energy-control-volume requirement is now resolved separately
under B2; therefore no unresolved component remains under B1.

### B2 — Full temperature-resolved reformer/furnace energy balance absent
**Disposition: RESOLVED.**

The conventional energy model is no longer a 298 K reaction-duty surrogate.
Temperature-dependent NIST Shomate enthalpies cover the major reformer species
through the source-stated 900-950 C outlet range, including the correct
high-temperature H2 interval. The model independently reconstructs syngas-WHB
heat recovery from 900-950 C to the published 320 C HTS inlet and verifies
consistency with IEAGHG's separate statement that about 75% of saturated HP
steam is generated in that WHB.

For the radiant section, a second independent calculation combines
temperature-corrected SMR/WGS reaction enthalpy with product-stream sensible
heating over a declared 600-700 C reformer-inlet uncertainty range and the
source 900-950 C outlet range. The authoritative IEAGHG radiant duty
(82.63 million kcal/h, about 96.0 MW) is not used to construct this envelope.

Acceptance tolerance was probed progressively rather than chosen after seeing a
pass: <20%, then <10%, then <5% nearest-envelope relative error. All three pass
Rust CI; therefore the independent reconstruction agrees with the authoritative
radiant benchmark to better than 5% at the nearest admissible bound. Given that
IEAGHG does not publish the exact primary-reformer inlet state, this is accepted
as a source-limited temperature-resolved validation rather than pretending a
unique inlet composition is known.

Evidence: temperature-property/WHB commits
`e38e210646b69cb34f8fd505340c367386e8bdce`,
`8b0d3eb54b6800cf94f3283da1ec313f4feb078e`;
independent radiant reconstruction `9a36146ac5e093c10978428242796535f0d7e671`;
validation probes `d2c9db182bc663026d20593594e5f1590066d155`,
`8724d295d8633bc392bdc421e2226198ba9bd58b`,
`974708425ad227dbcc711142a91feaefe5571d2d`.

### B3 — PSA-tail-gas disposition unresolved when fired reformer is removed
**Disposition: RESOLVED.**

The furnace-free candidate now has an explicit species disposition and a
steady-state inert purge. CO2 in the PSA-tail/recycle path is removed to a
high-pressure process-carbon capture train; H2/CO/CH4 are recycled to the
reformer; source N2 is carried explicitly as an inert and a nonzero purge is
solved from a specified maximum N2 mole fraction. The purge therefore cannot
silently accumulate inerts.

A nonselective purge necessarily carries H2/CO/CH4. Its CO/CH4 carbon is now an
explicit carbonaceous offgas routed to catalytic oxidation followed by capture,
rather than disappearing from the ledger. External fresh-feed carbon closes
exactly between captured and residual-emitted carbon after this purge route is
included.

Rust CI verifies N2 steady-state closure, monotonic purge requirement with inert
limit, purge-carbon accounting, and final topology carbon closure.

Evidence: `7ed4181592fd65715c14def44c2536619f0f7ce9`,
`90e0168480c4e6cbc9ee350e1a01e393a3c7a32d`.

### B4 — Capture topology cannot be fixed before carbon architecture closes
**Disposition: RESOLVED.**

With B3 closed, the furnace-free CCS boundary is now stream-specific rather
than a universal amine fraction:

1. shifted syngas: high-pressure MDEA, analogous to IEAGHG Case 1A (~2.5 MPa);
2. residual PSA-tail/recycle CO2: compressed-tail MDEA polishing, analogous to
   IEAGHG Case 2A (~1 MPa absorber feed after tail compression);
3. H2/CO/CH4: recycle to reformer;
4. inert-control purge H2/CO/CH4: catalytic oxidation, then CO2 joins the
   capture/compression chain;
5. no reformer-flue-gas MEA block exists because the fired reformer is removed.

IEAGHG explicitly distinguishes shifted-syngas MDEA, PSA-tail MDEA and
low-pressure flue-gas MEA because their pressure/composition differ. The model
therefore retains separate stream topology and does not use one solvent-duty
model for all locations.

The final terminal capture fraction remains a scenario parameter for the carbon
ledger; it is not interpreted as evidence that the same absorber operates on
all streams.

Evidence: IEAGHG 2017-02 Case 1A/2A/03 topology and Rust commit
`90e0168480c4e6cbc9ee350e1a01e393a3c7a32d`, CI passed.

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
**Disposition: RESOLVED with B4.**
IEAGHG's distinct shifted-syngas MDEA, compressed PSA-tail MDEA and flue-gas
MEA cases are now reflected explicitly in the topology. The furnace-free
candidate has no flue-gas MEA block; purge oxidation CO2 joins the process
capture/compression chain rather than being assigned a generic MEA penalty.

### M2 — Nuclear heat-integration literature needs quantitative secondary-loop/HX constraints
**Disposition: RESOLVED.**
After B2 established the high-grade process-duty envelope, the model now checks
that duty quantitatively against the JAEA GTHTR300C 170 MW IHX benchmark,
requires an explicit secondary-helium hot-end approach, calculates remaining
primary-to-secondary temperature budget, derives helium mass flow, and assigns
a nonzero circulator load from the published 58 kPa IHX pressure-drop anchor.
Feasible and infeasible temperature cases are both tested; no reactor vendor is
selected. Evidence: commit `469553a4dc98bcf383836f7e308307cc927a18a7`, CI passed.

### M3 — Nuclear technology should remain temperature-envelope based
**Disposition: RESOLVED / requirement retained.**
The model remains temperature-envelope based. Reactor outlet temperature,
secondary-helium approach and IHX duty are explicit variables/constraints; the
JAEA design is used only as a quantitative benchmark, not a selected product.

### M4 — Economic accounting framework incomplete
**Disposition: ACCEPTED.**
The incremental-abatement-cost equation is sound; currency year, annualisation,
capacity factor, energy prices, nuclear allocation and CCS transport/storage
basis remain to be frozen before assignment-level cost results.

### M5 — Current tests emphasize reconstruction more than predictive validation
**Disposition: RESOLVED for Review 1.**
Validation now extends beyond source reconstruction: NIST/JANAF SMR/WGS
equilibrium and limiting-case tests; IEAGHG HTS Q/K and apparent-equilibrium
temperature; analytical-vs-iterative recycle fixed point; temperature-resolved
WHB validation; and an independent primary-reformer radiant-duty envelope that
matches the authoritative IEAGHG ~96 MW equipment metric to better than 5% at
the nearest admissible source-limited bound. Nuclear heat-integration tests also
exercise both feasible and infeasible temperature budgets. This satisfies the
Review-1 requirement for meaningful independent duty/model validation; it does
not imply the recycle model is fully validated for later Review 2.

### M6 — Thermodynamic property layer missing
**Disposition: RESOLVED.**
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

## Remaining non-blocker work

M4 economic-accounting details remain intentionally deferred. They are not a
scientific blocker for the Review 1 foundation gate and must not displace the
requested mathematical/computational Review 2 that follows closure.
