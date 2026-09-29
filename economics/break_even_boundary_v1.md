# Analytical S$100/tCO2e feasibility boundary

## Purpose

Instead of choosing a favourable cost point, solve the CN4252 threshold as a boundary.

Let:

B = allowed annual incremental cost at S$100/tCO2e;
M_C = annual CO2 captured and transported;
p_TS = CCS transport/storage tariff in S$/t captured;
C_other = all other incremental annual costs.

Feasibility requires:

M_C p_TS + C_other < B.

Therefore:

p_TS,max = (B - C_other)/M_C.

And:

C_other,max = B - M_C p_TS.

## Reference screening point

Using:
- 71.952 ktH2/y;
- 7.5 kgCO2e/kgH2 lifecycle abatement;
- B ~= S$54.0m/y;
- M_C ~= 0.50 MtCO2/y captured;

the **absolute** T&S tariff ceiling if every other incremental cost were zero is:

p_TS,max ~= S$108/t captured.

This is an intentionally impossible best case because capture, nuclear integration and O&M cannot all be zero.

## Singapore-source T&S study pressure

IEAGHG's 2023 Singapore-source network analysis gives Group A T&S at USD50-75/t.

Using an explicit representative late-Sep-2026 FX sensitivity:

1 USD = 1.276 SGD,

this corresponds to approximately:

S$63.8-95.7/t captured.

At 0.50 Mt/y captured:

T&S annual cost ~= S$31.9-47.85m/y.

Residual budget:

S$54.0m - T&S
~= S$22.1m/y at the low Group-A end;
~= S$6.15m/y at the high Group-A end.

That residual must pay for **all** of:
- CO2 capture incremental capital/O&M not included in T&S;
- allocated HTGR capital/O&M;
- IHX and secondary helium loop;
- reformer/tail-gas modifications;
- circulator electricity;
- financing/construction differentials;
minus any avoided-cost/product credits.

## Dedicated versus cogeneration consequence

If C_R is common-reactor annual cost:

Dedicated:
C_H2,reactor = C_R.

GTHTR300C-like energy-share cogeneration:
C_H2,reactor = 0.2833 C_R.

At the low Group-A T&S end, the S$22.1m/y residual implies:
- dedicated common-reactor annual cost must be below S$22.1m/y even if every other non-T&S cost is zero;
- under 28.33% allocation, total common-reactor annual cost could be up to ~S$78m/y under the same unrealistic zero-other-cost assumption.

At the high Group-A end:
- dedicated ceiling ~S$6.15m/y;
- 28.33% allocation total common-reactor ceiling ~S$21.7m/y.

These are **upper ceilings**, not expected reactor costs.

## Strong falsification pressure

IEAGHG's conventional SMR+CCS Case 1A already reports EUR47.1/tCO2 avoided on a Q4-2014 basis while assuming only EUR10/t stored for CO2 T&S.

Therefore replacing its low T&S assumption with a Singapore cross-border Group-A-like T&S range is likely to materially raise the conventional CCS cost before nuclear integration is charged.

A harmonised price-year/currency calculation is required before stating that the S$100/t target is definitely failed.

Nevertheless, the feasibility region is now visibly narrow enough that the project must treat **SMR+CCS without nuclear** as a serious competing hypothesis:
- if conventional CCS itself cannot meet S$100/t under Singapore T&S conditions, nuclear integration must create enough avoided fuel/utility value to offset its additional capital;
- if it cannot, the original concept fails the assignment economic criterion even if technically feasible.

## Next task

Harmonise IEAGHG Case 1A economics from EUR2014Q4 to a declared SGD price year, separating its EUR10/t storage assumption from capture/integration cost. Replace the storage term with Singapore T&S sensitivities.

This will create a Singapore-adjusted **conventional SMR+CCS comparator** before any nuclear premium is added.

That comparator is the correct next economic baseline.
