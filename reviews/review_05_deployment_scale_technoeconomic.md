# Independent Review 05 — Deployment-Scale Techno-Economic Verification

## Repository state reviewed

Frozen repository HEAD: `f2539eb8a22f44bdfd4bfc6dbe53e8acabfa8f0a`.

Reviewed the post-Review-4 deployment extension only. Reviews 1-4 and the
original Gate-5 result were not reopened.

Primary files:
- `model/src/deployment.rs`
- `results/deployment/LITERATURE_BASIS.md`
- `results/deployment/RESULTS.md`
- `results/deployment/SANITY_CHECKS.md`
- `paper/sections/10a_deployment_extension.tex`
- changed Abstract/Discussion/Conclusions and bibliography
- current deployment manuscript artifact 11068991762 (20 pages)
- Research/Paper CI evidence recorded in STATUS.

The actual 20-page PDF was downloaded, rendered and visually inspected.

## Scope

Independent falsification of the new claim that a 1.30-scale
nuclear-assisted-SMR+CCS deployment can conditionally satisfy both CN4252
thresholds under a JAEA mature cogeneration cost case.

The two strict thresholds are:
- annual avoided emissions >250,000 tCO2e/y;
- forward abatement cost <S$100/tCO2e.

## Original-result preservation

PASS.

The extension does not overwrite the original fixed-scale result:
~74.848 ktH2/y, 64 cases, 0 joint passes. The manuscript clearly distinguishes
the post-canonical deployment extension from Gate 5.

## Deployment-scale reproduction

Independent arithmetic reproduces the scale calculation:

- original annual H2 = 8994 kg/h * 8322 h/y /1000 = 74,848.068 t/y;
- best-positive Gate-5 annual avoided = 194,759.021 t/y;
- specific abatement = 2.602058 tCO2e/tH2;
- H2 required for exactly 250,000 t/y = 96,077.7935 t/y;
- scale = 1.28363759;
- 1.30 scale H2 = 97,302.4884 t/y;
- 1.30 scale avoided = 253,186.7273 t/y.

The code correctly uses strict `>250_000`.

The abatement margin is only 3,186.727 t/y, or 1.27% above the absolute
threshold. The 1.30 scale is therefore a narrow scale margin, not a robust
abatement margin.

Extensive process quantities are consistently scaled through hourly throughput
to preserve the chosen annual H2 service when availability changes. No
unscaled original-size CCS/storage quantity was found in the deployment path.

## Reactor-sizing audit

At 80% availability:
- operating hours = 7008 h/y;
- hourly throughput multiplier = 1.30*8322/7008 = 1.54375;
- canonical high-side process heat = 91.784 MW;
- deployment process heat = ~141.692 MW.

This is below the cited 170 MW GTHTR300C IHX duty and below one 600 MWth module
capacity.

However, MW matching alone is insufficient. See R5-B01: the source GTHTR300C
170 MW IHX produces about 900 C secondary helium, whereas the verified R3 heat
cascade requires 920 C secondary helium to supply a 900 C process hot end with
a positive 20 K approach.

## JAEA/JAERI source audit

Primary-source findings:

1. GTHTR300C engineering precedent:
   - reactor = 600 MWth;
   - reactor outlet = 950 C;
   - reference cogeneration IHX = 170 MWth;
   - secondary-helium IHX outlet = ~900 C;
   - balance reactor heat is used for gas-turbine electricity.
   JAEA/JAERI sources therefore support cogeneration architecture and duty
   scale, but not the R3 920 C secondary-helium state.

2. JAEA-Review 2014-037:
   - evaluates a future commercial HTGR-IS system, not observed plant costs;
   - acknowledges high-accuracy future optimized commercial economics are
     difficult;
   - assumes 80% HTGR availability;
   - 40-year reactor operation;
   - 3% discount rate for generation-cost calculation;
   - revised electricity cost 5.8 JPY/kWh;
   - heat-supply cost 0.7 JPY/MJ at 45% electric efficiency;
   - GTHTR300 economic basis is 4 units/plant, each 600 MWth;
   - assumes learning/equipment rationalization in the hydrogen-plant
     evaluation and excludes land/interest from that plant construction
     estimate.

The repository is correct to call these mature/future-design economics rather
than observed commercial costs. It is not correct to treat the 0.7 JPY/MJ
four-unit economic basis as demonstrated one-module economics without an
explicit transfer assumption (R5-B02).

3. Independent JAEA cogeneration work is relevant and actually stronger in one
respect: a 2007 GTHTR300C cost study reports ~59.7 billion JPY nuclear plant
cost including IHX/secondary-helium-loop additions and ~0.52 JPY/MJ nuclear
heat at 85% availability. This supports the existence of a mature-design
cogeneration cost concept, but it remains a design study and its secondary
helium is still ~900 C.

## Modern/FOAK source audit

INL/GAIN 2024 supports the extension's thermal-only HTGR-SMR cost anchors:
- moderate HTGR SMR OCC ~US$2,500/kWth;
- conservative ~US$3,250/kWth;
- thermal-only HTGR-SMR O&M moderate ~US$12/MWhth;
- conservative ~US$16/MWhth;
- general reference CF 93%;
- illustrative WACC 7.5%, with strong WACC sensitivity.

The report explicitly warns that advanced-reactor costs remain uncertain and
that more actual commercial-offer data are needed. The project's 10% adverse
WACC and 80% adverse availability are scenario assumptions, not INL source
values.

The reported modern-central and adverse failures are directionally and
arithmetically consistent with these much higher cost bases. No evidence was
found that they were artificially inflated to make the JAEA case look better.

## Currency and cost-year audit

Arithmetic conversions are correct:
- 0.7 JPY/MJ * S$0.0087/JPY *1000 = S$6.09/GJ;
- US$2,500/kWth *1.30 = S$3,250/kWth;
- US$3,250/kWth *1.30 = S$4,225/kWth;
- EUR41.02m *1.50 = S$61.53m before throughput scaling.

But the extension performs FX conversion without escalating the historical JAEA
or IEAGHG source-year costs to a common modern cost year. This is R5-M01.
The JPY FX assumption is actually somewhat conservative relative to late
September 2026 spot levels (~S$0.0081/JPY), but FX conservatism does not solve
cost-year/localization consistency.

## CAPEX and financing audit

The CRF implementation is correct:
CRF = i(1+i)^N / ((1+i)^N - 1).

Representative values reproduce:
- 3%/40 y = 0.0432624;
- 8%/25 y ~0.09368.

CCS capital:
- IEAGHG reference capture = 387,805.2 t/y;
- deployment storage = 653,718.615 t/y;
- incremental TCR EUR41.02m;
- S$1.50/EUR and linear throughput scaling -> ~S$103.72m;
- 8%/25 y -> ~S$9.72m/y.

Linear CCS scaling is explicitly conservative with respect to economy of scale.

The mature JAEA path does not separately annualize allocated reactor CAPEX
because 0.7 JPY/MJ is treated as an all-in nuclear heat-service price. That is
internally consistent only if the source heat price is transferable to the
deployment's cogeneration configuration; R5-B02 addresses that transfer.

Integration/site allowance (10/20/30%) is an explicit project assumption, not
source-backed Singapore CAPEX. This is acceptable for screening if claim
strength remains conditional.

## Cogeneration audit

This is the decisive economic boundary.

Physical precedent exists: JAEA's GTHTR300C sends ~170 MWth to the hydrogen
branch and the balance of the 600 MWth reactor to electricity generation. Thus
a useful surplus-output architecture is not invented by this project.

At the project's 141.692 MW process-heat requirement, ~458 MWth of reactor
thermal capacity remains for the power branch. Singapore's electricity system
is large enough that an output of order a few hundred MWe is physically
absorbable in principle; Singapore generated ~60 TWh in 2024 and has >12 GW
registered generation capacity. This does not prove a commercial offtake
contract or price.

The mature cogeneration calculation charges hydrogen for process heat at the
JAEA heat-service price and assumes the remaining reactor economic burden is
allocated to useful cogeneration. It does not explicitly model electricity
revenue, market price, power-cycle output or interconnection cost. The
manuscript correctly calls this a capacity-allocation screen, not a revenue
forecast.

Hydrogen-only falsification is meaningful: charging the full module at the same
heat-service rate gives roughly S$331/t and fails. Therefore the passing case
does not make unused capacity free; it is conditional on economically useful
cogeneration.

See R5-B02 and R5-M02 for what remains unverified.

## CCS audit

Captured/storage throughput scales to 653,718.615 t/y and CCS capital,
compression and T&S are retained.

T&S is explicitly scenario-based because Singapore has no authoritative
cross-border tariff. MTI confirms that capture/transport/storage cost estimates
are still being developed.

Low mature case = S$15/t stored; central = S$30/t; adverse = S$45/t.

At the JAEA mature structure, the current S$47.637/t result has ~S$52.36/t
headroom. Holding all else fixed, T&S alone could rise from S$15/t to roughly
S$35/t stored before the S$100/t threshold is reached. Thus the pass does not
depend uniquely on exactly S$15/t, but the adverse S$45/t case would erase it.

No CCS cost was found silently set to zero.

## Lifecycle audit

The extension preserves the positive specific-abatement physical corner and
scales extensive lifecycle terms linearly with annual service. Scaling does not
improve kgCO2e/kgH2; it only raises absolute avoided emissions.

The conservative non-positive-abatement corner correctly has no finite scale
that can satisfy the annual-abatement criterion.

The principal deployment lifecycle weakness is not arithmetic but the narrow
1.27% annual-abatement margin and the unresolved high-temperature interface in
R5-B01.

## Independent S$/t reconstruction

Using the declared mature-case inputs:

- process heat ~141.692 MW;
- hours = 7008/y;
- annual heat ~3.5747 million GJ;
- heat service at S$6.09/GJ ~S$21.77m/y;
- scaled CCS CAPEX ~S$103.72m;
- CCS annualization ~S$9.72m/y;
- low T&S ~S$9.81m/y;
- allocated nuclear capital indicator ~S$102.7m;
- 10% integration/site allowance annualized at 3%/40 y ~S$0.89m/y;
- repository net NG/auxiliary contribution ~-S$30.1m/y.

Total incremental annual cost reconstructs to ~S$12.1m/y.

S$12.1m / 253,186.727 t/y = ~S$47.6/tCO2e.

The reported S$47.637/t is therefore arithmetically reproducible to rounding.

This does NOT by itself verify the pass because B01/B02 challenge the physical
and source-equivalence assumptions underlying the priced heat service.

## Break-even and robustness assessment

At 253,186.727 t/y, the exact S$100/t annual incremental-cost ceiling is:

S$25.3187m/y.

Reported mature incremental cost is ~S$12.06m/y, leaving ~S$13.26m/y headroom.

Heat-price break-even is ~S$9.80/GJ versus the project's S$6.09/GJ, a ~61%
increase.

The abatement side is much less robust: 253,186.727 t/y is only 1.27% above the
strict 250,000 t/y requirement.

The mature pass therefore has meaningful arithmetic cost headroom under its
assumed allocation, but little annual-abatement scale headroom and material
dependence on the mature cogeneration heat-cost transfer.

## Manuscript claim audit

The manuscript successfully distinguishes:
- ORIGINAL fixed-scale result: 0/64 joint passes;
- DEPLOYMENT extension: conditional JAEA mature cogeneration pass.

Abstract, deployment section, Discussion and Conclusions all state that the
conditional pass does not establish commercial feasibility or a preferred
Singapore technology.

No sentence was found that converts the conditional scenario into a categorical
claim that nuclear-assisted SMR+CCS "meets CN4252" generally.

However, until B01/B02 are resolved, even the phrase "conditional scenario
pass" is not independently verified for the current implementation.

The proposed chronology is scientifically legitimate:
fixed-scale failure -> identify absolute-scale/economic constraints ->
deployment-scale redesign -> test conditional mature-design economics.
The original adverse evidence may be condensed later but should not be deleted.

## Figures/tables audit

The current 20-page PDF is readable with no clipping or broken deployment
tables/figures.

Deployment scale figure correctly shows linear absolute abatement scaling.
Cost curves distinguish mature/modern/adverse regimes.
Feasibility figure clearly marks scale and heat-price boundaries.
Cost-breakdown figure correctly shows NG/auxiliary savings as a negative
incremental-cost component.

The main scientific visual omission is the temperature incompatibility in
R5-B01: deployment tables show MWth but do not expose that the cited GTHTR300C
secondary-helium outlet is ~900 C versus the model-required 920 C.

## Reproducibility audit

Strong computational reproducibility:
literature/scenario constants -> deployment.rs -> generated CSV -> LaTeX ->
PDF is deterministic and CI-tested.

Green CI proves arithmetic/code consistency and manuscript generation. It does
not prove:
- source-equivalent transfer of 0.7 JPY/MJ to one module;
- adequacy of 900 C source secondary helium for a 920 C model requirement;
- actual Singapore cogeneration revenue/offtake;
- actual Singapore CCS tariff.

## Work that survived review

Do NOT rewrite or reopen:
- original 0/64 Gate-5 result;
- scale arithmetic itself;
- strict-threshold implementation;
- extensive linear annual scaling for the declared same-specific-performance
  screen;
- CCS throughput arithmetic and CRF;
- INL modern/adverse anchors;
- hydrogen-only falsification;
- conditional/non-commercial manuscript language;
- separation of source values from project scenario assumptions;
- reproducibility pipeline.

## Blockers

### R5-B01 — JAEA reactor/IHX anchor does not meet the verified model's required secondary-helium temperature

**Finding**
The deployment extension declares one GTHTR300C-class 600 MWth module with a
170 MW IHX physically sufficient because 141.692 MW <170 MW, but the temperature
interface is inconsistent.

**Exact evidence**
The verified R3 manuscript/model requires:
- process hot end 900 C;
- secondary helium hot end 920 C;
- primary outlet 950 C;
with 20 K process and 30 K IHX approaches.

JAEA GTHTR300C primary sources specify:
- 950 C primary inlet to IHX;
- ~900 C secondary-helium IHX outlet;
- 170 MWth reference IHX.

**Independent verification**
900 C secondary helium cannot supply a 900 C process hot end with a positive
20 K terminal approach. The source design satisfies duty but not the project's
own heat-quality constraint.

**Affected result**
Physical feasibility of the 1.30-scale JAEA mature cogeneration case and
therefore the reported S$47.637/t conditional joint pass.

**Why it matters**
CN4252 requires an engineering-consistent solution. Nominal reactor outlet
temperature and MWth capacity are not sufficient when the intermediate loop
cannot deliver the modelled process temperature.

**Required correction**
Either:
(a) provide primary/design evidence for an IHX/secondary-loop configuration
that can deliver >=920 C secondary helium at the required ~142 MW duty from the
chosen reactor envelope; or
(b) revise the process/heat-cascade state to a source-supported lower hot-end
temperature and re-solve the full physical/lifecycle/economic deployment case.

Do not simply set the source secondary temperature to 920 C.

**Acceptance criterion**
The deployment case has a source-supported reactor/IHX/secondary-helium state
with positive IHX and process terminal approaches at the solved duty, and the
recomputed annual abatement and S$/t still satisfy the strict CN4252 thresholds.

**Recommended verification test**
Automated deployment test must compare source-anchored secondary-He outlet,
process hot-end and declared minimum approach and reject zero/negative
temperature driving force.

---

### R5-B02 — The only passing economic anchor transfers a four-unit mature-design heat price to a one-module deployment without demonstrating source-equivalent economics

**Finding**
The S$47.637/t case uses JAEA 0.7 JPY/MJ as an all-in nuclear heat-service price
for one 600 MWth module, while the source's revised GTHTR300 economic basis is a
future commercial four-unit plant.

**Exact evidence**
JAEA-Review 2014-037 states:
- GTHTR300 configuration: 4 units/plant;
- 600 MWth per unit;
- 80% availability;
- 40-year reactor operation;
- 3% discount rate for generation cost;
- 5.8 JPY/kWh revised electricity cost;
- 0.7 JPY/MJ heat supply cost obtained from the gas-turbine generation
  economics at 45% efficiency.

The report explicitly frames this as a future-commercial evaluation and assumes
learning/rationalisation; it is not an observed tariff.

`deployment.rs` sizes one 600 MWth module and directly prices its hydrogen
process heat at 0.7 JPY/MJ.

**Independent verification**
The arithmetic conversion to S$6.09/GJ is correct. The source context is not
equivalent to the one-module project context. Multi-unit sharing/learning and
site/common-service economics may contribute to the source unit cost.

A separate JAEA GTHTR300C design study provides a cogeneration nuclear-plant
cost including IHX/secondary-loop equipment, showing that a cogeneration
economic model can be built directly; the current extension has not reconciled
that evidence with the 0.7 JPY/MJ one-module use.

**Affected result**
The only sub-S$100/t deployment case.

**Why it matters**
The pass has cost headroom, but it exists only under the mature JAEA heat-price
anchor. If that price is not applicable to the installed configuration, the
central conditional-pass claim is not independently established.

**Required correction**
Reconcile the deployment economic boundary with primary JAEA evidence.
Acceptable approaches include:
- model the actual four-unit mature-design basis and allocate all products/costs;
- derive a one-module mature cogeneration heat cost from source-backed
  GTHTR300/GTHTR300C capital/O&M/fuel/finance data;
- or explicitly treat 0.7 JPY/MJ as a project sensitivity assumption rather
  than a source-equivalent one-module case and demonstrate the pass over a
  defensible source-supported one-module heat-cost range.

**Acceptance criterion**
The passing deployment's nuclear heat cost is traceable to an economic boundary
consistent with its number of modules, cogeneration equipment, availability,
finance and O&M/fuel treatment. Recalculated forward abatement cost remains
<S$100/t without leaving any reactor cost economically unassigned.

**Recommended verification test**
Independently reconstruct annual full-reactor cost and product allocations;
hydrogen heat cost + cogenerated-product cost/revenue allocation must recover
the complete annual reactor economic burden within declared tolerance.

## Major findings

### R5-M01 — Cost years are not normalized before modern SGD comparison

Historical JAEA/JAERI and IEAGHG costs are converted by fixed FX assumptions but
not escalated to a common cost year. Modern INL values are 2020s-era estimates.

This does not by itself prove the mature pass fails: current JPY/SGD is lower
than the project's 0.0087 assumption, and the heat-price break-even has ~61%
headroom. But a present-Singapore S$/t claim should not mix historical nominal
costs without an explicit cost-year convention.

**Required correction**
Choose and state a common real/nominal cost year. Escalate source costs with a
defensible index or explicitly retain historical-real scenarios and prevent
cross-year interpretation.

**Acceptance criterion**
Every monetary input in each deployment case has source currency/year,
escalation convention, FX date/basis and final SGD cost year; S$/t is reported
on one internally consistent basis.

**Recommended verification test**
Machine-readable currency/cost-year ledger with independently checked
representative conversions.

---

### R5-M02 — Cogeneration is physically plausible but its economic offtake is assumed rather than closed

JAEA provides a real cogeneration architecture and Singapore has a power system
large enough to absorb order-hundreds of MWe physically. The current model,
however, does not calculate the remaining power output, electricity-generation
efficiency after the chosen 141.7 MW heat extraction, power-cycle/interconnection
costs or a Singapore sale/value stream.

The manuscript correctly says the remaining output "must" have a useful
customer and calls the method a capacity-allocation screen. That is good claim
discipline, but the economic burden is not closed in a forward cogeneration
business case.

**Required correction**
Quantify the surplus product for the passing case and either:
- model its source-supported generation cost and value/revenue; or
- formulate an explicit break-even offtake value required for the hydrogen
  allocation to remain valid.

Include power-cycle equipment already embedded in the chosen JAEA cost basis
once, not twice.

**Acceptance criterion**
No reactor capacity is economically free: the full annual reactor burden is
allocated between hydrogen and a quantified cogenerated product, and the
required offtake/value is explicitly shown to be a scenario condition.

**Recommended verification test**
Set cogeneration value to zero and recover the hydrogen-only failure; then solve
the minimum surplus-output value needed for S$100/t and compare it with an
appropriate Singapore market/value range.

---

### R5-M03 — The 1.30 deployment has only 1.27% annual-abatement headroom

The strict annual threshold is passed by 3,186.7 t/y only. Small deterioration
in specific lifecycle abatement, availability/annual service, or scale can
erase the pass.

This does not invalidate the scenario but must be visible alongside the much
larger cost headroom.

**Required correction**
Report explicit annual-abatement margin and minimum required service/availability
sensitivity rather than presenting 1.30 as generically above threshold.

**Acceptance criterion**
The manuscript/result table reports the 1.27% abatement margin and at least one
sensitivity showing the minimum H2 service required as specific abatement
changes.

**Recommended verification test**
Perturb specific abatement by +/-2% and verify threshold classification changes
where mathematically expected.

## Minor findings

### R5-m01 — JAEA source categories should be separated more finely

"JAEA mature design" currently combines the 2014 revised heat-cost screen,
earlier GTHTR300 capital targets and GTHTR300C engineering precedent. They are
related but not one single source case. Label them as separate economic,
capital-target and engineering anchors.

### R5-m02 — Current JPY FX assumption is not dated

S$0.0087/JPY is conservative relative to late-September-2026 spot values near
S$0.0081/JPY, but the chosen FX date/basis should be stated.

### R5-m03 — Deployment PDF is readable

No deployment figure/table clipping or broken references were observed in the
20-page artifact. Some tables are dense but usable.

## Oral-defence vulnerabilities

1. Why does scale help? — Defensible: it multiplies positive specific abatement
   to cross an absolute annual threshold; it does not improve specific LCA.
2. Why 1.30? — Defensible arithmetically, but only 1.27% abatement headroom.
3. Why one 600 MWth reactor? — Duty fits, but temperature does not fit the cited
   900 C secondary-helium design (B01).
4. What happens to unused output? — Physically JAEA sends it to electricity;
   economic offtake remains assumed (M02).
5. Who buys it? — Not established; Singapore grid absorption is physically
   plausible but commercial value is not demonstrated.
6. Why is JAEA cost relevant? — Future-commercial design-study anchor only,
   not Singapore quote; one-module transfer is unresolved (B02).
7. Is it FOAK/NOAK? — Mature/future-commercial design study with learning
   assumptions, not observed commercial FOAK.
8. More expensive Singapore finance? — Modern/adverse screens fail; JAEA heat
   price embeds 3% source finance.
9. No cogeneration customer? — Hydrogen-only case fails (~S$331/t).
10. Why S$47.6? — Reproduced from ~S$12.1m incremental /253,187 t avoided.
11. CCS cost? — Low pass uses S$15/t stored scenario; no Singapore tariff exists.
12. Storage available today? — No; cross-border CCS remains under development.
13. Commercial feasibility? — Not demonstrated.
14. Why not conventional SMR+CCS? — Case 1A remains a serious comparator; matched
   Singapore economics are unresolved.
15. Easiest pass destroyers? — Current physical temperature mismatch already
   blocks verification; economically, mature heat-cost transfer/cogeneration
   allocation and T&S are key; abatement scale has only 1.27% headroom.

## Final assessment

The deployment extension contains useful and mostly reproducible work. The scale
calculation, CCS arithmetic, CRF, hydrogen-only falsification, modern/adverse
cost screens and S$47.6 arithmetic all survive review.

The reported conditional joint pass does not yet survive independent
verification for two root reasons:

1. the cited GTHTR300C 170 MW heat branch supplies ~900 C secondary helium while
   the verified process model requires 920 C secondary helium for its 900 C
   process hot end; and
2. the only passing heat-price anchor is transferred from a four-unit future
   commercial JAEA economic basis to a one-module deployment without closing
   source-equivalent one-module/cogeneration economics.

These are not cosmetic uncertainties. They affect the physical and economic
premises of the only passing scenario.

## Main Research Handoff

1. **R5-B01 — reconcile the high-temperature interface.**
   Acceptance: source-supported secondary helium >= required process hot end +
   minimum approach at ~142 MW duty, or a re-solved lower-temperature process
   case; both CN4252 thresholds must still pass after recomputation.

2. **R5-B02 — rebuild/reconcile mature nuclear economics on the actual module and
   cogeneration boundary.**
   Acceptance: full annual reactor cost is traceable and allocated across
   hydrogen and cogenerated output with no free capacity; the resulting
   forward cost remains <S$100/t.

3. **R5-M01 — normalize monetary inputs to an explicit common cost-year/FX basis.**
   Acceptance: source year/currency -> escalation -> FX -> final SGD year is
   traceable for every material cost.

4. **R5-M02 — quantify the cogenerated product and required offtake/value.**
   Acceptance: surplus output, conversion efficiency, annual energy and
   minimum economic value are explicit; zero-value surplus recovers the
   hydrogen-only failure.

5. **R5-M03 — expose the narrow annual-abatement margin.**
   Acceptance: 1.27% margin and service/specific-abatement sensitivity are
   reported and threshold classification is regression-tested.
