# Dedicated versus cogeneration HTGR economic allocation

## Assignment target authority

The official CN4252 problem statement supplied in Project Sources is authoritative for this project and requires:

- >0.25 MtCO2e/year within Singapore;
- <S$100/tCO2e.

A current public Singapore SPEED webpage uses different programme thresholds, including <S$250/tCO2e as a desirable decarbonisation-cost criterion. That is a different/current programme page and does **not** replace the supplied CN4252 assignment statement.

Therefore this repository continues to test S$100/tCO2e.

## JAEA cogeneration architecture

JAEA's GTHTR300C concept uses a 600 MWth reactor with:
- 170 MWth process heat to hydrogen production;
- 430 MWth remaining for power generation;
- approximately 900 C secondary helium for the hydrogen branch.

Thus a simple energy-share allocation of common reactor cost gives:

f_H2 = 170/600 = 0.2833.

This is one transparent allocation convention, not a universal accounting rule.

The project's current 131-151 MWth process-service requirement lies below the 170 MWth process-heat branch of that published cogeneration architecture.

## Evidence that allocation matters economically

JAEA's HTGR hydrogen cogeneration study reports an IS-process example where attributed hydrogen cost changes from:
- H2-only: 24.2 US cents/Nm3;
- H2 + electricity cogeneration: 11.8 US cents/Nm3.

The source attributes the reduction to changing the share of HTGR depreciation cost between hydrogen and power and crediting electricity sales.

This result is **not transferred numerically to nuclear-assisted SMR**, because:
- the hydrogen process is IS water splitting, not SMR;
- the economic assumptions are Japanese and legacy;
- electricity-market and financing assumptions differ.

It is retained as direct evidence that multi-product HTGR cost allocation can be first-order.

## Allocation cases for this project

### DED — dedicated reactor
Hydrogen/process heat bears 100% of common HTGR annualised capital and fixed O&M.

### COG-E — thermal-energy-share cogeneration
Hydrogen bears:

f_H2 = Q_H2 / Q_reactor.

For the GTHTR300C reference split:

f_H2 = 28.33%.

### COG-V — economic-value allocation
Common cost is allocated according to product economic value or avoided-cost benefit.

This method is conceptually defensible for cogeneration but is not implemented until Singapore electricity/heat values are sourced.

## Break-even consequence

Let:
- B = total incremental annual-cost budget allowed by S$100/t;
- C_R = total annualised common reactor cost;
- f = allocation fraction;
- C_other = CCS + integration + incremental O&M + other costs - avoided-cost credits.

Then:

C_abatement < 100 S$/t

requires:

f C_R + C_other < B.

Equivalently, the maximum common-reactor annual cost compatible with the target is:

C_R,max = (B - C_other)/f.

Thus reducing f from 1.0 to 0.283 increases the allowable total common-reactor cost by a factor of ~3.53 for the same hydrogen-side budget, provided the remaining reactor output has a real economic use and is not double-counted.

This is not free cost reduction: the other product must genuinely carry its allocated cost.

## Singapore CCS cost warning

An IEAGHG 2023 network-cost workshop includes an ExxonMobil analysis titled *South East Asia CO2 Transport and Storage Cost Estimates for CO2 Sources in Singapore*. Its cost curve groups prospective Singapore-source T&S opportunities approximately as:
- Group A: USD50-75/tCO2;
- Group B: USD75-150/tCO2;
- Group C: USD150-450/tCO2.

These are study/workshop cost estimates, not current contracted Singapore CCS tariffs.

They nevertheless show why CCS T&S may consume a large fraction of the CN4252 S$100/tCO2e budget even before capture and nuclear costs are counted.

MTI stated in April 2025 that clearer end-to-end Singapore CCS cost estimates were still being developed. EMA's current position likewise highlights relatively high transport/storage cost and risk.

Therefore the economic feasibility window may require:
- low-cost Group-A-like storage;
- high capture utilisation/capacity factor;
- cogeneration/shared HTGR allocation;
- or some combination of these.

## Next calculation

Construct a two-dimensional break-even map in terms of:

x = allocated reactor annual cost;
y = CCS T&S cost per tonne captured;

with capture cost/integration OPEX as explicit additional parameters.

The S$100/t boundary should be plotted analytically rather than selecting a single favourable point.
