# Deployment-scale extension — independent sanity checks

These checks are independent arithmetic reconstructions of representative generated results. They do not replace the Rust tests.

## 1. Minimum annual H2 scale
Original annual H2 = 74,848.068 t/y.
Best verified Gate-5 annual avoided emissions = 194,759.021 tCO2e/y.
Specific abatement = 194,759.021 / 74,848.068 = 2.602 tCO2e/tH2.
Required annual H2 for 250,000 t/y = 250,000 / 2.602 = 96,077.8 tH2/y.
Annual scale factor = 96,077.8 / 74,848.068 = 1.28364.
The evaluated extension uses 1.30, giving 97,302.488 tH2/y and 253,186.727 tCO2e/y avoided.

## 2. Reactor/process-heat scale
JAEA mature availability = 80%, so annual operating hours = 0.80 x 8760 = 7008 h/y.
To deliver 1.30 times the canonical annual service, hourly throughput factor = 1.30 x 8322 / 7008 = 1.54375.
Canonical high-side process heat = 91.784 MW.
Required process heat capacity = 91.784 x 1.54375 = 141.692 MW.
One 600 MWth GTHTR300-class module is sufficient.
Thermal-service utilisation = 141.692 / 600 = 0.2362 (23.6%); the remaining reactor output must have a useful cogeneration outlet for the cogeneration allocation to be valid.

## 3. JAEA heat-cost conversion
Source heat cost = 0.7 JPY/MJ.
Project FX screening assumption = S$0.0087/JPY.
Converted heat price = 0.7 x 0.0087 x 1000 = S$6.09/GJ.
Annual process heat = 141.692 MW x 7008 h/y x 3.6 GJ/MWh = 3.5747 million GJ/y.
Annual nuclear heat-service cost = about S$21.77m/y.

## 4. CCS capital and T&S
IEAGHG Case-1A reference captured CO2 = 0.466 kg/Nm3 H2 x 100,000 Nm3/h x 8322 h/y /1000 = 387,805.2 t/y.
Generated extension storage throughput = 653,718.615 t/y.
IEAGHG incremental TCR = EUR41.02m; project FX = S$1.50/EUR.
Linear scaled CCS capital = 41.02m x 1.50 x (653,718.615/387,805.2) = S$103.72m.
At 8%, 25 y, CRF = 0.09368, annual CCS capital = about S$9.72m/y.
Low T&S scenario = S$15/t captured, annual T&S = S$9.81m/y.

## 5. Mature cogeneration forward cost
Converted JAEA 50 bn JPY upper design-target module CAPEX = S$435m.
Capacity-allocated reactor capital indicator = 435m x 0.2362 = S$102.73m; it is NOT separately annualised in the mature forward cost because the 0.7 JPY/MJ heat-service price already aggregates reactor economics.
Integration/site allowance = 10% x (allocated reactor indicator + scaled CCS capital) = about S$20.64m, annualised at 3%/40 y = about S$0.893m/y.
Generated net NG/auxiliary incremental contribution = -S$30.12m/y.
Total incremental annual cost = -30.12 + 21.77 + 9.72 + 0.893 + 9.806 = about S$12.06m/y.
Abatement cost = 12.061m / 253,186.727 = S$47.64/tCO2e.
Both CN4252 thresholds pass in this conditional mature cogeneration scenario.

## 6. Hydrogen-only allocation falsification
Charging the full 600 MWth module at the same JAEA heat-service price gives about S$92.19m/y nuclear service before CCS/integration.
With the same other terms, the hydrogen-only case is approximately S$331/tCO2e and fails the cost threshold.
Therefore the conditional pass depends materially on useful cogeneration/capacity allocation; unused reactor capacity cannot be free.

## 7. Break-even heat diagnostic
At the mature low-T&S structure, the generated non-heat annual cost terms imply a S$100/t boundary at about S$9.80/GJ delivered nuclear heat.
The converted JAEA mature value S$6.09/GJ lies below that boundary.
This is a break-even diagnostic, not a literature observation.

## 8. Modern cost falsification
Generated cogeneration results at the same 1.30 annual H2 scale:
- JAEA mature: S$47.637/t, joint PASS.
- Modern central (INL moderate): S$174.401/t, cost FAIL.
- FOAK/adverse (INL conservative + high-financing screen): S$372.814/t, cost FAIL.
All three exceed 0.25 MtCO2e/y; cost is the binding discriminator for the modern/adverse cases.

## Scientific lock
The original Gate-5 result remains 64 cases and 0 joint passes. These deployment results are separate post-canonical scenarios.
