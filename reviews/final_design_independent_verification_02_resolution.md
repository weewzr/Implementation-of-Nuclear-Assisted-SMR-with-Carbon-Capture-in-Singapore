# Final Design Independent Verification 02 — Correction Resolution

Status: **CORRECTIONS IMPLEMENTED — INDEPENDENT REVIEWER RE-VERIFICATION REQUIRED.**

This is a Main-Research correction record responding to
`reviews/final_design_independent_verification_02.md`. It does not close the
independent verification.

## FDV2-B01 — JAEA source-variant conflation

### Finding
The frozen design mixed the 170 MWth / ~202 MWe GTHTR300C variant with the
Nishihara 2007 370 MWth / 88 MWe economic configuration.

### Root cause
Hardware precedent and economic-source quantities from two GTHTR300C variants
were treated as one architecture.

### Correction
The final mature architecture is now explicitly Nishihara 2007:
- 600 MWth reactor;
- 370 MWth source hydrogen/IHX branch;
- 230 MWth complementary source power branch;
- 88 MWe source gross electricity;
- 59.7 bn JPY reference plant cost;
- 0.52 JPY/MJ heat;
- 4.9 JPY/kWh electricity;
- 85% availability.

The INL process draws 176.8 MWth, below the 370 MWth source heat branch.
The project maps the residual 423.2 MWth through the source-implied
88/230 gross conversion, giving 161.92 MWe gross. After 17.3 MWe process
demand, net export is 144.62 MWe.

The 70.9 bn JPY / 0.57 JPY/MJ / 5.5 JPY/kWh case is now labelled only as
Nishihara's doubled-IHX/secondary-loop COST sensitivity. No capacity claim is
made from it.

### Primary-source evidence
Nishihara, Mouri and Kunitomi (2007), ICONE15-10157, reports a 600 MWth
GTHTR300C whose hydrogen production plant uses 370 MWth. The source economic
configuration and associated cost sensitivity are kept separate from the
historical 170/202 variant.

### Numerical impact
Principal reference mapping:
- process heat: 176.8 MWth;
- residual power thermal: 423.2 MWth;
- gross electricity: 161.92 MWe;
- net export: 144.62 MWe;
- annual net export: ~1.077 TWh/y.

Corrected reference-economics screens:
- zero electricity value: ~S$3.45m/y, ~S$3.76/tCO2e;
- S$100/MWh: ~-S$104.23m/y, ~-S$113.65/t;
- S$150/MWh: ~-S$158.07m/y, ~-S$172.35/t;
- S$200/MWh: ~-S$211.92m/y, ~-S$231.06/t.
Doubled-cost sensitivity at S$150/MWh: ~-S$162.29/t.

### Tests
Added source-variant identity, reactor thermal closure, source-efficiency gross
power derivation, net export, zero-value and doubled-cost tests.

### Acceptance status
**IMPLEMENTED; awaiting CI and Independent Reviewer acceptance.**

## FDV2-M01 — lifecycle intensity factor-of-1000

### Finding
The frozen implementation multiplied a tCO2e/tH2 ratio by 1000 even though
t/t and kg/kg are numerically identical.

### Correction
Removed the factor 1000. Added dimensional regression test.

### Numerical impact
Candidate lifecycle intensity under the preserved proxy boundary is
approximately **1.95 kgCO2e/kgH2**, not ~1,953.

The independently reproduced ~917,139 tCO2e/y lifecycle-abatement result is
unchanged.

### Tests
`lifecycle_ci_units_are_correct` requires the corrected result to lie near
1.9-2.1 kg/kg.

### Acceptance status
**IMPLEMENTED; awaiting CI and Independent Reviewer acceptance.**

## FDV2-M02 — integrated thermal/power state

### Finding
The frozen design independently inserted INL process heat, a 585 C return
assumption and 202 MWe from a different JAEA variant.

### Correction
The evidence layers are now explicit:
- INL chemical process: 176.8 MWth; 900 C supply; ~466 C return; source flow
  78.49 kg/s; 17.3 MWe process electricity.
- JAEA/Nishihara hardware economics: 600/370/230 MWth and 88 MWe source split.
- Project integration: 176.8 MWth process draw, 423.2 MWth residual power
  branch, 161.92 MWe gross using the source 88/230 conversion, 144.62 MWe net.

Constant-Cp helium screening with 900/466 C gives ~78.34 kg/s, within 0.2% of
the INL source flow.

### Tests
Added reactor thermal-balance closure, INL helium-flow source check, gross
electric derivation and net-export derivation.

### Acceptance status
**IMPLEMENTED; awaiting CI and Independent Reviewer acceptance.**

## Preserved scientific result

The independently reproduced INL process basis was not changed:
- 97,946.015 tH2/y;
- 862,093.803 tCO2/y direct avoided;
- ~917,138.896 tCO2e/y lifecycle avoided under the declared proxy boundary.

## Required handoff

Do not close this review here.

After Research CI, Paper CI, generated-result regeneration and PDF inspection
are complete, return this corrected state to the Independent Reviewer. The
Independent Reviewer must determine whether FDV2-B01/M01/M02 now satisfy the
verification acceptance criteria.
