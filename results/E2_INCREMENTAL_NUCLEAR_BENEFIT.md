# E2 Incremental Nuclear-Benefit Attribution

## 1. Question and result

E2 asks: **what does nuclear process heat add beyond conventional SMR-H2 + CCS?**

The strongest source-backed non-nuclear comparator is IEAGHG 2017-02 Case 1A (shifted-syngas MDEA capture). It is normalized to the project's common 130 MMSCFD / 29,000 lb/h source service and 85% annual availability.

The central finding is deliberately two-part:

1. **CCS alone already clears the CN4252 0.25 MtCO2e/y abatement threshold** on the E2 matched-service lifecycle screen: A→B = approximately **461,084 tCO2e/y**.
2. The numerical B→C difference is approximately **350,488 tCO2e/y**, but it cannot be attributed solely to nuclear heat because Case B (IEAGHG) and Case C (INL) are different source flowsheets with different capture topology, capture fraction and gas accounting. E2 therefore calls this the **B→C bundled cross-source difference**, not a pure nuclear-abatement result.

This limitation is scientifically material. E2 does not manufacture a pure nuclear percentage or pure nuclear S$/tCO2e value.

## 2. Common H2 normalization

IEAGHG's base and Case 1A both produce:
- 100,000 Nm3 H2/h;
- 8.994 t H2/h.

Therefore the source conversion is:
[
0.08994 {m kg H_2/Nm^3}.
]

The project source H2 rate is 29,000 lb/h = 13,154.18 kg/h. The equivalent IEAGHG volumetric service is:
[
146,255.0 {m Nm^3/h}.
]

At 85% availability:
[
m_{H2}=97,946.0 {m t/y},
]
identical to the canonical project annual H2 result.

All IEAGHG A/B mass-emission terms are scaled by this common H2 service.

## 3. Physical cases

### Case A — unabated conventional SMR

Source: IEAGHG base case.

At the common service:
- source NG energy input: 394.77 MW LHV before H2 scaling;
- direct CO2: **881,122 t/y**;
- upstream NG lifecycle burden: **121,694 tCO2e/y** using the project's common 11.5 kgCO2e/GJ proxy;
- no captured-CO2 T&S burden;
- E2 lifecycle total: **1,002,816 tCO2e/y**.

The represented annual cost is held at the E1 baseline **S$269.115m/y** so the economic comparison remains on the E1 Singapore screening boundary. This is not a bottom-up IEAGHG plant-cost reconstruction.

### Case B — conventional SMR + CCS, no nuclear heat

Source: IEAGHG Case 1A.

At the common service:
- source NG energy input: 407.68 MW LHV before H2 scaling;
- direct CO2: **403,371 t/y**;
- captured CO2: **507,481 t/y**;
- upstream NG: **125,673 tCO2e/y**;
- T&S lifecycle proxy: **12,687 tCO2e/y**;
- lifecycle total: **541,731 tCO2e/y**.

Case 1A consumes more NG than the no-capture base because CCS imposes an energy penalty. E2 does **not** give it the nuclear-assisted Case-6 34 MMSCFD gas rate.

For cost, E2 uses the repository's existing Singapore Case-1A screen:
- S$77.7/t direct CO2 avoided non-T&S;
- plus the E1 S$15/t captured T&S assumption.

This produces:
- A→B incremental represented cost: **S$44.733m/y**;
- represented Case-B annual cost: **S$313.849m/y**.

This is a screening harmonisation, not a full matched Singapore COM/TCI estimate.

### Case C — HTGR-assisted SMR + CCS

Source: current INL Case-6 project candidate plus E1 central economics.

At 85% availability:
- 34.0 MMSCFD natural gas;
- 176.8 MWth nuclear process heat;
- direct CO2: **39,966 t/y**;
- upstream NG: **133,618 tCO2e/y**;
- nuclear lifecycle burden: **3,649 tCO2e/y**;
- incremental auxiliary lifecycle burden: **450 tCO2e/y**;
- T&S lifecycle burden: **13,559 tCO2e/y**;
- lifecycle total: **191,243 tCO2e/y**;
- represented annual E1 central cost: **S$410.393m/y**.

E1 economics are unchanged.

## 4. Emissions attribution

The E2 normalized lifecycle ledger gives:

| Quantity | tCO2e/y |
|---|---:|
| A lifecycle | 1,002,816 |
| B lifecycle | 541,731 |
| C lifecycle | 191,243 |
| A→B abatement | **461,084** |
| B→C bundled difference | **350,488** |
| A→C cross-source total | **811,573** |

The algebraic identity closes exactly:
[
(A-B)+(B-C)=A-C.
]

### Descriptive shares

Relative to this E2 cross-source A→C total:
- A→B CCS step = **56.8%**;
- B→C bundled difference = **43.2%**.

**43.2% is not a pure nuclear share.** It bundles:
- nuclear heat substitution;
- a different source process model;
- a much higher capture fraction/topology;
- different natural-gas accounting;
- different residual direct-emissions state.

The project's earlier canonical A→C avoided value (917,139 tCO2e/y) used an INL-family baseline. The E2 cross-source total is lower (811,573 tCO2e/y) because E2 uses the IEAGHG conventional baseline so that A and B are truly matched. This is a comparator-boundary change, not an emissions-model regression.

## 5. Natural-gas attribution cannot be made as a single percentage

IEAGHG A/B report NG on an LHV energy basis. The current INL candidate is preserved as a volumetric 34 MMSCFD source value and the project conversion uses an HHV screening basis. Directly subtracting the displayed MW-equivalent values would mix LHV and HHV conventions.

Therefore E2 does **not** report a false-precision percentage NG reduction attributable to nuclear. It can state only:
- A→B: IEAGHG CCS increases source NG input from 394.77 to 407.68 MW LHV;
- C: INL Case 6 reports 34 MMSCFD NG with nuclear heat;
- a pure B→C NG saving requires a common gas-composition/heating-value process basis not currently available.

## 6. Cost attribution

### A→B CCS step

- incremental annual represented cost: **S$44.733m/y**;
- lifecycle abatement: **461,084 tCO2e/y**;
- screening incremental cost: **S$97.02/tCO2e**.

Thus, under this particular E2 harmonisation, Case B:
- passes the >0.25 Mt/y abatement requirement;
- sits just below S$100/t on the lifecycle denominator.

This differs from the repository's earlier S$146–180/t **direct-avoidance** Singapore Case-1A screen because E2 uses the broader lifecycle-abatement denominator and the E1 S$15/t T&S assumption rather than the earlier Group-A USD50–75/t T&S range. Both records are retained; they answer different bounded questions.

### B→C bundled cost difference

- represented annual cost difference: **+S$96.544m/y**;
- bundled lifecycle difference: **350,488 tCO2e/y**;
- arithmetic ratio: **S$275.46/tCO2e**.

This **must not be called the pure nuclear-step cost**. It includes the cross-source process/capture differences described above.

A pure incremental nuclear S$/tCO2e cannot be identified from the current source evidence without a conventional non-nuclear comparator using the same INL Case-6 chemistry/capture topology and differing only in heat source.

## 7. Nuclear-value falsification

### Does CCS alone exceed 0.25 MtCO2e/y?
**YES.** E2 A→B lifecycle abatement is approximately 0.461 MtCO2e/y.

### Is nuclear necessary for the abatement threshold?
**NO**, based on the strongest matched non-nuclear comparator currently available.

### Is nuclear necessary for the cost threshold?
**NO evidence establishes that it is.** Under the E2 S$15/t T&S harmonisation, Case B is approximately S$97/tCO2e and therefore already below the threshold, while the E1 central nuclear-assisted total is S$154/tCO2e against its own verified INL baseline.

Under the earlier higher Singapore Group-A T&S screen, conventional CCS was above S$100/t. But adding the E1 central nuclear burden does not demonstrate that nuclear rescues that case.

### Does nuclear improve cost effectiveness?
The available matched-service evidence does **not** support that claim. The B→C bundled cost rises much faster than the bundled abatement, and the E1 central nuclear-assisted result fails S$100/t.

### What additional benefit remains potentially relevant?
Evidence supports nuclear heat as a way to displace the fired high-temperature heat service and reduce dependence on combustion for reformer heat. However, current sources do not isolate that benefit from the simultaneous capture/process changes strongly enough to assign a pure lifecycle-tonnage percentage. Strategic energy-security benefits are not monetized or claimed in E2.

## 8. Direct-emissions attribution

A→B direct reduction:
**477,751 tCO2/y**.

B→C direct difference:
**363,405 tCO2/y**.

Again, the B→C direct difference is not purely nuclear: most of the difference reflects the much lower residual direct-emission state of the INL nuclear-assisted capture configuration, not simply removal of furnace combustion.

## 9. Matching limitations

1. **Different source families:** A/B are IEAGHG; C is INL.
2. **Capture topology:** Case 1A captures shifted-syngas CO2 at ~55.7% overall capture; C uses the INL Case-6 capture configuration with much lower residual direct emissions.
3. **Gas basis:** IEAGHG provides LHV energy; INL project gas is MMSCFD with a project HHV conversion assumption.
4. **Electricity:** IEAGHG reports lost net export; C reports process electricity. No common Singapore marginal-grid lifecycle factor is adopted, so E2 does not invent an electricity displacement credit/penalty.
5. **Economics:** B uses the strongest existing Case-1A screening harmonisation; C uses the stronger E1 central model. They are not one vendor-grade common cost estimate.
6. **Lifecycle proxies:** upstream gas and T&S lifecycle factors are common project proxies, not route-specific LCAs.

These limitations prevent a pure nuclear-only attribution, but they do not affect the robust finding that the matched IEAGHG CCS step alone exceeds 0.25 Mt/y.

## 10. Alternative pathways

E2 does not rank electrolysis or electrified SMR. The repository lacks sufficiently matched Singapore lifecycle/economic evidence to compare those pathways on the same basis. The final manuscript must therefore avoid implying nuclear is automatically preferred merely because the integrated configuration has high total abatement.

## 11. Reproducibility

Canonical E2 implementation:
- `model/src/e2_attribution.rs`.

Generator:
- `model/src/bin/e2_incremental_nuclear_benefit.rs`.

Machine-readable outputs:
- `results/e2_attribution/e2_input_provenance.csv`;
- `results/e2_attribution/e2_comparator.csv`;
- `results/e2_attribution/e2_attribution.csv`.

Tests enforce:
- common H2 normalization;
- exact attribution closure;
- CCS-alone abatement threshold;
- no capture double counting;
- preservation of E1 candidate cost;
- no claim that the B→C bundled difference is the majority of the cross-source total.

## 12. E2 conclusion

The project can now answer the first governing question strongly:

**Conventional SMR+CCS alone already supplies more than the required 0.25 MtCO2e/y abatement at the common H2 service. Nuclear heat is therefore not necessary to meet the CN4252 abatement threshold.**

The second question has a bounded rather than falsely precise answer:

**The current source evidence does not isolate a pure nuclear-only abatement or pure nuclear-only S$/tCO2e because the non-nuclear and nuclear cases are different source flowsheets. The observed B→C bundled difference is 0.350 MtCO2e/y at +S$96.5m/y represented cost, or an arithmetic S$275/tCO2e, but that number must not be presented as a pure nuclear-step metric.**

The strongest economic evidence therefore does not establish that nuclear integration improves the assignment's cost effectiveness. Its justification, if any, must rest on a future truly matched process comparison and/or additional benefits demonstrated in later engineering phases—not on crediting nuclear with all CCS abatement.
