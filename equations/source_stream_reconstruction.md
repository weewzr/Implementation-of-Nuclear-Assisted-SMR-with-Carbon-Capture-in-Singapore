# Source-resolved pre-reformer / reformer / HTS reconstruction

## New source evidence

The full IEAGHG 2017-02 heat/material balance provides streams upstream of the PSA, eliminating the need to infer the entire reformer/WGS section from PSA data alone.

For the reference/base process, relevant published states include:

| State | T (C) | P (MPa) | Flow (kmol/h) | Key composition |
|---|---:|---:|---:|---|
| pre-reformer feed | 500 | 3.39 | 5514.0 | H2O 0.7307, CH4 0.2350, C2H6 0.0185, C3H8 0.0026 |
| HTS inlet | 320 | 2.80 | 8370.3 | H2 0.5171, H2O 0.2927, CO 0.1156, CO2 0.0492, CH4 0.0238 |
| HTS outlet | 412 | 2.77 | 8370.3 | H2 0.5961, H2O 0.2137, CO 0.0366, CO2 0.1283, CH4 0.0238 |
| PSA inlet | 35 | 2.58 | 6596.9 | H2 ~0.75, CO2 0.1627, CO 0.0464, CH4 0.0302 |

Source: IEAGHG Report 2017-02 full heat/material-balance tables.

## HTS verification

For:
CO + H2O -> CO2 + H2,

the reaction extent can be independently reconstructed four ways:

xi_W,CO = n_CO,in - n_CO,out

xi_W,CO2 = n_CO2,out - n_CO2,in

xi_W,H2 = n_H2,out - n_H2,in

xi_W,H2O = n_H2O,in - n_H2O,out.

Using the rounded source table, all four are approximately 661 kmol/h within rounding tolerance.

This is a stronger verification than the previous reduced model because it checks the actual published HTS section without assuming an extent.

The Rust model now asserts C/H/O elemental closure across this section.

## Steam-to-carbon diagnostic

The published pre-reformer-feed stream contains about 73.07 mol% water. Dividing water molar flow by carbon atoms in its carbon-bearing species gives a directly reconstructed stream-level steam/carbon ratio of approximately 2.55.

This value is retained as a source-stream diagnostic, not silently replaced by a generic design S/C assumption. The stream already contains small amounts of H2 and CO2 and sits downstream of feed preparation, so it is not automatically identical to the plant's nominal design S/C specification.

## Process-temperature hierarchy

The source data now provides an important heat-integration constraint:

- pre-reforming occurs around 500 C;
- reformer product is described by IEAGHG as normally around 900-950 C;
- syngas is cooled before HTS, with the reference HTS inlet at 320 C;
- the HTS outlet is 412 C because the WGS reaction is exothermic;
- gas is then cooled/condensed to the 35 C PSA inlet.

Therefore a future HTGR integration model must distinguish:
1. high-grade reformer reaction/process heat near reformer temperature;
2. medium-grade preheat/prereforming duties;
3. steam-generation duties;
4. low-/medium-grade heat recovery and CCS regeneration.

A single scalar "reactor heat" is not an adequate integration model.

## Next derivation

The next model layer should use the actual NG species and the pre-reformer feed to reconstruct the prereformer + primary-reformer transformation into the published HTS-inlet composition. That model must explicitly account for higher hydrocarbons rather than collapsing them into equivalent methane.

Once that transformation closes C/H/O, temperature-dependent enthalpy can be attached to each source state.
