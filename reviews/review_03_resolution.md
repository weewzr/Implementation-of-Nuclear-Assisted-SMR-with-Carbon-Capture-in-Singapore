# Independent Review 3 resolution record

Independent Review 3 / Gate 4: **OPEN**.

This file tracks corrective work against
`reviews/review_03_integrated_model_results.md`. It is not a new review.

## R3-B01 — One physical canonical stream graph

**Disposition: RESOLVED for the declared screening-model scope.**

### Correction
The new R3 canonical path no longer scales the already-reacted HTS-inlet gas as
fresh feed. It begins from IEAGHG stream 4 pre-reformer feed. C2+ hydrocarbons
are reduced into WetGas6 with an explicit complete-steam-prereforming
stoichiometric screen that conserves C/H/O, then the source-reconstructed
interstage water is added.

The ordered loop is now:
external NG/steam-equivalent feed + recycle -> coupled SMR/WGS reformer ->
HTS/WGS -> explicit CO2 removal -> PSA -> N2 purge -> recycle.

Captured CO2 is removed from the downstream state before PSA and therefore
cannot simultaneously remain in recycle.

### Code/equation location
`model/src/lib.rs`:
- `r3_ieaghg_external_reformer_feed_wet6`
- `remove_co2_fraction`
- `r3_solve_at_fixed_fresh`
- `r3_canonical_recycle_case`
- `r3_external_element_residuals`
- `r3_external_normalized_element_residuals`
- `wet6_mass_kg_h`
- `r3_external_mass_residual_fraction`
- `r3_once_through_reformer_relative_errors`

### Validation evidence
The canonical external boundary treats recycle as internal and explicitly
closes fresh external feed against H2 product + captured CO2 + purge.
Normalized C/H/O/N residuals and total-mass residual are required below 1e-6.

The zero-recycle external-feed reformer is independently compared with IEAGHG
stream 5 before recycle fitting. Because the exact pre-reformer outlet and
primary-reformer inlet are unpublished and the prereformer is a reduced
stoichiometric screen, the source-compatibility criterion is deliberately broad:
major reactive species must be within 50% of the rounded source row and N2
within 2%. This is a screening validation, not an exact source-state claim.

### CI evidence
- commit `0b4daaa08734c2b2e738850497a13dea639f04da`, CI run 36540122585: PASS.
- commit `81e272bbda62d1ad4780667935151d2a398b22b1`, CI run 36540261744: PASS.

### Acceptance criterion status
- actual source NG/steam-derived external state: PASS for reduced prereformer scope
- physical unit ordering: PASS
- captured CO2 absent downstream: PASS
- normalized plant C/H/O/N residuals <=1e-6: PASS
- plant total-mass residual <=1e-6: PASS
- fixed H2 product and finite purge: PASS
- zero-recycle source compatibility: PASS at declared screening tolerance

R3-B01 is closed. This does not validate detailed prereformer kinetics.

## Next corrective action

R3-B02 — close a temperature-feasible integrated heat cascade using this new
R3 canonical state. Review-2 energy/CCS/helium ledgers remain superseded for
Gate-4 purposes until rebuilt on the R3 state.
