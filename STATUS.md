# STATUS

## Current state
Final-design second independent verification: **CORRECTIONS IMPLEMENTED — RE-VERIFICATION PENDING**.

Controlling review:
`reviews/final_design_independent_verification_02.md`.

Correction record:
`reviews/final_design_independent_verification_02_resolution.md`.

Do NOT begin Final Submission QA. Do NOT declare the independent verification
closed. The Independent Reviewer must re-check the corrected design.

## Preserved INL process basis
- 871 C reformer; S/C 3.0; 78.1% methane conversion; 88% PSA recovery.
- 130 MMSCFD H2.
- 925 C INL ROT / 900 C process heat.
- 176.8 MWth process heat; 17.3 MWe process electricity.
- 34.0 MMSCFD candidate NG versus 52.5 MMSCFD conventional baseline.
- 1,927 short t/day captured CO2; 142 candidate emitted; 3,205 baseline emitted.
- ~97,946 tH2/y.
- ~862,094 t/y direct avoided.
- ~917,139 tCO2e/y lifecycle avoided under the declared proxy boundary.

## Corrected JAEA integration
Principal mature architecture is now the coherent Nishihara 2007 configuration:
- 600 MWth reactor;
- 370 MWth source hydrogen/IHX branch;
- 230 MWth source power branch;
- 88 MWe source gross electricity;
- 59.7 bn JPY / 0.52 JPY/MJ / 4.9 JPY/kWh reference economics.

Project-derived mapping for the 176.8 MWth INL draw:
- residual power thermal = 423.2 MWth;
- gross electricity = 161.92 MWe using source 88/230 conversion;
- net export = 144.62 MWe after 17.3 MWe process demand;
- annual export ~1.077 TWh/y at 85% availability.
- process-side He screen ~78.34 kg/s using INL 900/466 C state.

The 70.9 bn JPY / 0.57 JPY/MJ / 5.5 JPY/kWh case is adverse COST
sensitivity only, not a capacity claim.

## Corrected deterministic economics
Reference mature architecture:
- zero electricity value: ~S$3.76/tCO2e;
- S$100/MWh: ~-S$113.65/t;
- S$150/MWh: ~-S$172.35/t;
- S$200/MWh: ~-S$231.06/t;
- doubled-cost sensitivity at S$150/MWh: ~-S$162.29/t.

These are model outputs pending independent reviewer acceptance.

## Corrected lifecycle intensity
~1.95 kgCO2e/kgH2. The previous factor-of-1000 implementation error is removed.

## Verification
Code/tests/manuscript/source register have been corrected. Await current
Research CI and Paper CI before handoff evidence is complete.

## Next step
STOP after CI/result/PDF verification and return the corrected state to the
Independent Reviewer. Do not begin Final Submission QA.
