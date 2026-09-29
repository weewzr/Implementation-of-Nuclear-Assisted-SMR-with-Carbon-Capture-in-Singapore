# Matched lifecycle + economic case for 80% recycle/shared HTGR v1

## Purpose

Replace the illustrative 7.5 kgCO2e/kgH2 / S$54m/y denominator with a lifecycle calculation for the same physical 80% recycle + shared direct-HTGR case used in the economic boundary.

## Boundary and assumptions

Baseline:
- IEAGHG unabated SMR direct plant CO2;
- upstream NG applied to both feedstock and supplementary furnace NG.

Candidate:
- fixed H2 output;
- 80% reduced tail-recycle sensitivity;
- fresh NG reduced by the modelled 26.57%;
- supplementary furnace NG eliminated;
- external feed carbon reduced in proportion to fresh NG;
- 90% permanent capture of reduced external feed carbon;
- 162 MWth direct nuclear service;
- nuclear LCA proxy from 5.5 gCO2e/kWh_e and 50.4% electric efficiency;
- upstream NG = 11.5 gCO2e/MJ global-gas screening anchor;
- CCS transport emissions = 2.5% of captured CO2.

Important: applying 90% capture to reduced external feed carbon is a screening closure assumption. The reduced recycle model is not yet a fully converged process simulation.

## Lifecycle result

Internally consistent unabated baseline:

plant direct ~= 8.996 kgCO2/kgH2;
upstream NG ~= 1.817 kgCO2e/kgH2;

total:
**~10.813 kgCO2e/kgH2.**

80% recycle/shared-HTGR candidate:

residual plant carbon ~= 0.567;
upstream NG ~= 1.145;
nuclear lifecycle proxy ~= 0.050;
CCS transport ~= 0.128;

total:
**~1.890 kgCO2e/kgH2.**

Specific lifecycle abatement:

**~8.923 kgCO2e/kgH2.**

At 8994 kgH2/h and 8322 h/y:

annual lifecycle abatement:
**~0.668 MtCO2e/y.**

This exceeds the CN4252 >0.25 MtCO2e/y requirement at the reference-plant scale.

At S$100/tCO2e, the matched annual incremental-cost allowance is:

**~S$66.8m/y.**

This replaces the previous illustrative ~S$54m/y budget for this physical case.

## Why recycle increases the abatement denominator

The candidate removes two NG inputs relative to the baseline:
1. all purchased supplementary furnace NG;
2. ~26.6% of fresh feedstock NG.

Therefore upstream gas emissions fall materially as well as direct plant emissions.

This is why freezing the old 7.5 kg/kg abatement value understated the allowable S$100/t budget for the recycle case.

## Full-cost boundary rerun

Use:
- matched budget ~= S$66.8m/y;
- 162 MWth;
- nuclear-heat operating anchor S$5.69/GJ;
- separation electricity S$150/MWh;
- allocated MHR-T reactor cost ~S$50m/y;
- annualised IAEA IHX/secondary-loop ~S$8.2m/y;
- other reformer/recycle allowance S$5m/y sensitivity.

At low Group-A T&S (~S$31.9m/y):

minimum NG value becomes approximately:

**~S$14.6/GJ.**

Previously, with the inconsistent S$54m/y denominator, the result was ~S$17.5/GJ.

At S$20/GJ gas:

maximum compatible T&S becomes approximately:

**~S$55.5m/y.**

This exceeds both previous Group-A annual sensitivities (~S$31.9m/y and ~S$47.85m/y).

At S$15/GJ gas, maximum compatible T&S is only about:

**~S$33.7m/y,**

so only the low end of Group-A-like T&S remains feasible.

## Revised architecture interpretation

With lifecycle consistency restored:

- dedicated large HTGR remains outside the present cost region;
- shared HTGR at S$10/GJ gas remains strongly pressured;
- shared HTGR at ~S$15/GJ + low Group-A T&S becomes marginally feasible in this screen;
- shared HTGR at S$20/GJ has a materially broader Group-A T&S window.

Thus the lifecycle-consistent denominator reopens part of the shared-reactor region that the frozen S$54m/y screen had incorrectly excluded.

## Remaining limitations

This is still a screening result, not final assignment compliance:
- 80% recycle is reduced stoichiometric, not converged flowsheet simulation;
- 90% capture is imposed on reduced external feed carbon;
- nuclear heat LCA is a proxy, not a published HTGR process-heat LCA;
- Singapore gas upstream intensity is route-sensitive;
- CCS transport emissions are a fractional sensitivity;
- economics still use a legacy nuclear-heat operating anchor and approximate reactor-cost allocation.

## Next scientific dependency

The largest scientific weakness is now the reduced recycle closure itself.

Before tightening economics further, implement the iterative fixed-H2 recycle model so tail-gas composition, fresh-NG displacement, captured carbon and heat duty converge together.

Acceptance criterion:
- converged recycle mass/carbon/H2 balance;
- no imposed double counting of recycled carbon;
- lifecycle and economic functions consume the converged result rather than the 80% reduced sensitivity.
