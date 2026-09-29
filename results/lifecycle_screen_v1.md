# First assignment-level lifecycle and abatement-scale screen

## Scope

This is a screening calculation, not the final lifecycle result.

It combines:
- matched 90% feedstock-carbon capture;
- global IEA gas/LNG upstream-intensity anchors;
- UNECE nuclear lifecycle factors;
- a historical IEAGHG CO2-shipping emissions sensitivity;
- the current matched direct/electric process-service model.

It intentionally excludes unresolved storage-site-specific emissions, hydrogen leakage, construction differences outside the nuclear proxy, and route-specific Singapore gas data.

## 1. Singapore gas context

EMA 2024:
- natural-gas imports: 11 Mtoe;
- LNG: 6 Mtoe;
- pipeline/non-LNG balance: about 5 Mtoe.

Thus LNG represented roughly 55% of reported natural-gas imports by energy.

## 2. Upstream sensitivity

IEA global anchors:
- gas supply average: 11.5 gCO2e/MJ;
- LNG delivered production-to-regasification: 18.6 gCO2e/MJ.

On the IEAGHG feedstock basis these become:
- ~1.56 kgCO2e/kgH2;
- ~2.52 kgCO2e/kgH2.

The IEA states that gas-supply emissions vary by more than five-fold across geographies, so these are not interpreted as a tight uncertainty interval.

## 3. Matched 90% capture example

Feedstock-carbon equivalent:
7.72 kgCO2/kgH2.

At 90% permanent carbon capture:
- residual plant carbon: ~0.77 kgCO2/kgH2;
- captured carbon: ~6.95 kgCO2/kgH2.

For a transport sensitivity of 2.5% of captured CO2:
- CO2 transport-chain contribution: ~0.17 kgCO2e/kgH2.

This 2.5% anchor comes from an older IEAGHG 200-km ship case and is NOT a Singapore-Indonesia route estimate.

## 4. Nuclear contribution

Using:
- UNECE midpoint nuclear electricity factor: 5.5 gCO2e/kWh_e;
- direct-heat allocation proxy based on 50.4% net generation efficiency;
- representative 140 MW process service;
- eSMR heater efficiency 90%;

gives approximately:
- direct nuclear-heat lifecycle proxy: ~0.04 kgCO2e/kgH2;
- nuclear-electric heating: ~0.10 kgCO2e/kgH2.

Thus the direct/electric nuclear lifecycle difference is small relative to current upstream-gas uncertainty.

## 5. First lifecycle totals

Illustrative lower/global-gas anchor, 90% capture, 2.5% CO2-transport fraction:

direct heat:
~0.77 plant
+ ~1.56 upstream gas
+ ~0.04 nuclear
+ ~0.17 CO2 transport
= roughly 2.5-2.6 kgCO2e/kgH2.

electric:
same plant/upstream/transport terms
+ ~0.10 nuclear
= roughly 2.6 kgCO2e/kgH2.

Replacing the upstream anchor with the global LNG value raises either case by roughly 1 kgCO2e/kgH2.

These values are screening outputs, not final Singapore lifecycle intensities.

## 6. CN4252 abatement scale

IEA gives 10-12 kgCO2e/kgH2 for unabated natural-gas hydrogen.

Against a candidate intensity around 2.5-3.6 kgCO2e/kgH2, the specific abatement is of order:

~6.4-9.5 kgCO2e/kgH2.

The H2 production needed for 0.25 MtCO2e/y is therefore of order:

250 / Delta-e
~= 26-39 ktH2/y.

The IEAGHG reference plant at 8994 kgH2/h and 8000 h/y produces:

~72 ktH2/y.

Therefore this plant scale is large enough **in principle** to exceed 0.25 MtCO2e/y over much of the current lifecycle screening range.

This is not yet assignment compliance because:
- the actual Singapore counterfactual must be selected;
- lifecycle terms need route-specific refinement;
- plant capacity factor must be justified;
- the <S$100/tCO2e cost threshold remains untested.

## 7. Most important result

The current model suggests that once plant carbon capture is high, the dominant emissions uncertainty shifts away from the HTGR heat-delivery architecture and toward the imported natural-gas supply chain.

This is consistent with IEA's warning that upstream/midstream emissions dominate the residual intensity of high-capture natural-gas hydrogen.

Therefore the project's strongest falsification test is becoming:

**Can Singapore source sufficiently low-emission natural gas, in addition to achieving high carbon capture, for nuclear-assisted SMR to remain genuinely low-carbon?**
