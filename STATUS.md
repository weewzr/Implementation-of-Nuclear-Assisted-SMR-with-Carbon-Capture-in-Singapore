# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation: REVIEW 2 RECONCILIATION OPEN.

Independent Review 1 gate: CLOSED.
Independent Review 2 gate: OPEN.

## Reconciliation result
The prior Review-2 CLOSED status was premature. Reconciliation against the
original Review-2 acceptance criteria found one genuine unresolved BLOCKER and
three dependent MAJOR findings. No completed scientific finding was reopened
merely because a document was stale.

## Resolved Review-2 findings
- R2-B01: full-species recycle/purge steady state — RESOLVED by the nested,
  thermodynamically constrained WetGas6 solver with fixed-H2 outer residual,
  explicit N2 purge, conservation and CI tests.
- R2-B03: thermodynamic property-range handling — RESOLVED with interval-safe
  NIST Shomate dispatch and independent equilibrium checks.
- R2-M02: PSA behaviour — RESOLVED for screening through the literature-bounded
  70-90% H2-recovery envelope.
- R2-M03: independent/system-level falsification — RESOLVED for the declared
  screening-model scope.

## Open Review-2 findings
- **R2-B02 BLOCKER:** the integrated candidate temperature-resolved energy
  balance has not yet been rebuilt on the canonical thermodynamic full-species
  recycle state. Existing ~162 MWth/recycle heat functions originate from the
  retired reduced 0.737 surrogate; a closed reaction+sensible+steam/capture/
  recovery ledger with nuclear heat as residual is still required.
- **R2-M01:** stream-specific CCS topology is closed, but candidate capture
  duties are not yet scaled from the current solved stream state on one
  validated formulation.
- **R2-M04:** secondary-helium temperature/IHX screening exists, but full-loop
  candidate pressure loss and circulator power are not closed.
- **R2-M05:** legacy economic outputs are historical; final lifecycle/economic
  results have not been regenerated from the eventual closed B02/M01/M04
  physical ledgers.

## Current verified model state
- Conventional IEAGHG material and temperature-resolved energy reconstruction
  is source-validated; radiant-duty envelope agrees with the authoritative
  ~96 MW benchmark to better than 5% at the nearest admissible source-limited
  bound.
- Canonical recycle model is the nested full-species ideal-equilibrium solver
  with explicit N2 purge and fixed-H2 product closure.
- Legacy analytical ~0.737 fresh-feed fraction is provenance only and is
  retired from predictive propagation.
- Recent scientific commits through PSA-envelope closure pass Rust CI.

## Retained scientific limitations
- Reformer chemistry is ideal-gas equilibrium screening, not catalyst kinetics.
- PSA is a bounded recovery uncertainty model, not a bed-resolved adsorption
  cycle simulator.
- Exact IEAGHG primary-reformer inlet state is unpublished; validation is
  source-limited.

## Next scientific task
Resolve R2-B02: construct and validate one integrated candidate
temperature-resolved energy ledger on the canonical thermodynamic recycle
state. Do not regenerate economics until R2-B02, R2-M01 and R2-M04 are closed.

Canonical Review-2 disposition record:
- `reviews/review_02_resolution.md`

The earlier `reviews/review_02_model_verification.md` CLOSED decision is
superseded by the acceptance-criterion reconciliation in
`reviews/review_02_resolution.md`.
