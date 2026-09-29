# Economic foundation for the CN4252 abatement-cost test

## 1. Required metric

The assignment-level economic metric is incremental cost per tonne of CO2e avoided:

C_abatement
=
(C_candidate,annual - C_baseline,annual)
/
(E_baseline,annual - E_candidate,annual).

This is not the same as:
- LCOH;
- total candidate annual cost;
- CCS capture cost per tonne captured;
- Singapore carbon-tax liability.

Every comparison must use the same currency year, plant output, capacity factor, financing convention and lifecycle/emissions boundary.

## 2. Immediate threshold implication

The assignment requires:

C_abatement < S$100/tCO2e

and:

annual abatement > 0.25 MtCO2e/y.

At the minimum qualifying abatement scale, the maximum incremental annual cost compatible with the threshold is therefore:

S$100/t * 250,000 t/y
= S$25 million/y.

For a plant avoiding more than 0.25 Mt/y, the allowable incremental annual cost scales proportionally.

This is a useful budget identity, not a prediction.

## 3. Conventional SMR+CCS benchmark

IEAGHG 2017-02 reports for its standalone 100,000 Nm3/h H2 reference:
- Base LCOH: EUR 0.114/Nm3 H2;
- CCS increases LCOH by EUR 0.021-0.051/Nm3 H2;
- CO2 avoidance cost: EUR 47-70/tCO2;
- additional total capital requirement: roughly EUR 40-176 million;
- CCS increases plant cost by 18-79%;
- CCS raises operating cost of H2 production by 18-33%.

These are Q4 2014 estimates.

They are retained as a historical/reference benchmark and must NOT be compared directly with S$100/t without:
1. inflation to a common price year;
2. currency conversion;
3. consistent transport/storage boundary;
4. lifecycle-vs-plant-gate emissions alignment.

## 4. Singapore CCS cost is currently unresolved officially

MTI's April 2025 parliamentary reply states that Singapore is studying a cross-border CCS project and working with potential service providers to obtain clearer estimates for capture, transport and permanent storage.

Therefore this project should not claim an official Singapore CCS cost per tonne exists when the Government itself says estimates are still being developed.

CCS T&S cost remains a sensitivity parameter until a defensible route/service quotation or published project estimate is available.

## 5. Singapore carbon tax is a policy comparator, not the assignment metric

Singapore's carbon tax:
- S$45/tCO2e in 2026-2027;
- previously announced view of S$50-80/tCO2e by 2030.

This can be used as an economic context / avoided-tax sensitivity.

It is NOT interchangeable with:
- abatement cost;
- willingness-to-pay;
- CCS cost;
- social cost of carbon.

A technology with S$90/t abatement cost does not automatically become profitable because the assignment threshold is S$100/t.

## 6. Nuclear integration economics

The candidate cost difference must include, where applicable:

### Direct-heat architecture
- HTGR capital allocation to process heat;
- IHX;
- secondary-helium loop;
- reformer modifications;
- helium circulator electricity;
- steam-generator/superheater integration;
- tail-gas recycle/treatment;
- CCS capture/compression;
- CO2 transport/storage;
- nuclear O&M/fuel allocation.

### Nuclear-electric eSMR
- HTGR/power-cycle capital allocation;
- electric reformer;
- electrical infrastructure;
- eSMR efficiency;
- tail-gas recycle/treatment;
- CCS capture/compression;
- CO2 transport/storage;
- nuclear O&M/fuel allocation.

Shared items must not be charged twice.

## 7. External 2026 benchmark

Ahn & Lee (2026) report:
- helium-heated SMR LCOH: USD 2.37/kg H2;
- electrified SMR LCOH: USD 2.99/kg H2.

Those values support the plausibility of an economic difference between the architectures, but they are not an abatement-cost result for Singapore.

The project's model must reproduce its own:
- baseline LCOH;
- candidate LCOH;
- annual incremental cost;
- lifecycle CO2e avoided;
- S$/tCO2e avoided.

## 8. Annualisation framework

CAPEX is annualised with:

CRF = r(1+r)^n / [(1+r)^n - 1]

and:

C_annual = CAPEX*CRF + fixed OPEX + variable OPEX.

No final discount rate or project life is frozen yet.

The Rust implementation provides the equation and regression tests, while economic assumptions remain external inputs with provenance.

## 9. Economic falsification conditions

The proposed architecture fails the assignment economic criterion if, under a defensible central scenario:

C_abatement >= S$100/tCO2e.

The analysis should also report sensitivity rather than hiding threshold crossings.

Likely high-value sensitivities:
- HTGR CAPEX;
- nuclear heat/electricity allocation price;
- capacity factor;
- natural-gas price;
- CCS T&S charge;
- financing/discount rate;
- capture fraction;
- upstream NG intensity;
- reformer/loop CAPEX;
- tail-gas treatment cost.

## 10. Next required data

Before producing a central S$/t result, obtain defensible economic inputs for:
1. conventional Singapore/Asian NG price basis;
2. HTGR capital and O&M ranges;
3. IHX/secondary-loop capital;
4. eSMR incremental CAPEX;
5. CCS capture CAPEX/OPEX;
6. cross-border T&S sensitivity;
7. financing assumptions;
8. price-year/currency conversion methodology.

The model should then identify the **break-even HTGR/CCS cost combinations** compatible with S$100/tCO2e rather than relying on a single fragile point estimate.
