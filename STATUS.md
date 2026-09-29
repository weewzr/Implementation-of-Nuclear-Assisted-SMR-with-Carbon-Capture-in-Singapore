# STATUS

## Current research gate
Gate 3 — Mathematical/model foundation: COMPLETE.

Independent Review 1 gate: CLOSED.
Independent Review 2 gate: CLOSED.

## Verified model state
- IEAGHG conventional material and temperature-resolved energy reconstruction is source-validated; the independent radiant-duty envelope agrees with the authoritative ~96 MW benchmark to better than 5% at the nearest admissible source-limited bound.
- NIST/IAPWS thermodynamic properties and ideal-gas SMR/WGS equilibrium relations are implemented with interval/limiting/conservation tests.
- The canonical recycle model is the nested full-species thermodynamic solver: fixed-fresh recycle composition is converged internally, then an outer scalar solve closes fixed H2 product.
- Nonzero source N2 is closed by an explicit steady-state purge; C/H/O/N conservation and nonnegative species are CI-tested.
- The legacy analytical ~0.737 fresh-feed fraction is retired from predictive propagation and retained only for provenance/regression.
- PSA uncertainty is represented by a literature-bounded 70-90% H2-recovery envelope with the IEAGHG reconstructed recovery as the central source anchor; all envelope cases converge and preserve fixed H2 product.
- Furnace-free CCS topology is stream-specific: shifted-syngas process capture, tail/recycle CO2 polishing, reformer recycle, and explicit purge-carbon disposition.
- Nuclear heat integration remains reactor-agnostic and is constrained by duty, temperature approach, IHX benchmark and helium-loop pressure-drop/circulator calculations.

## Current CI state
Recent Review-2 scientific commits through PSA-envelope closure pass GitHub Rust CI (`cargo test --all-targets`).

## Retained scientific limitations
- Reformer chemistry is an ideal-gas equilibrium screening model, not catalyst-kinetic validation.
- PSA is a bounded recovery uncertainty model, not a bed-resolved adsorption-cycle simulator.
- The exact IEAGHG primary-reformer inlet state is unpublished; validation is therefore source-limited rather than a unique-state reconstruction.
- Economic results derived from the retired legacy recycle fraction are historical screening outputs and must not be treated as current predictive results.

## Blockers
No unresolved Review-1 or Review-2 mathematical/model-foundation blockers.

## Next step
Gate 4 — verified computational model. Begin only in the next research iteration; do not infer that Review-2 closure itself validates a detailed plant design.

Canonical review records:
- `reviews/review_01_resolution.md`
- `reviews/review_02_model_verification.md`
