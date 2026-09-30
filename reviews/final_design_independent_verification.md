# Final Design Independent Verification

## Scope and frozen state

This is the requested independent verification of the **current submission-facing final design only**. It is not Review 6 and does not reopen Reviews 1–5.

- Closure commit reviewed: `d553a3214927bdbb2ef499ed9872d76370b29aa7`
- Frozen scientific/design commit: `12625aed52d52670b4f87de7511e8a3dd1f103d5`
- Research CI `36662636613`: PASS
- Paper/reproducibility CI `36662636515`: PASS
- PDF artifact `11074623593`: 11 pages; prior closure records zero undefined citations/references and visual inspection PASS.

No model or manuscript file was modified during this verification.

## 1. Primary-source verification

### INL process basis

Primary sources independently checked:

- INL TEV-953 Rev. 0, *HTGR-Integrated Hydrogen Production via Steam Methane Reforming (SMR) Process Analysis*.
- INL TEV-961 Rev. 0, *Sensitivity of Hydrogen Production via Steam Methane Reforming to High Temperature Gas-Cooled Reactor Outlet Temperature Process Analysis*.

TEV-953 establishes the common world-scale hydrogen service as **130 MMSCFD = 29,000 lb/h H2**, with natural-gas feed adjusted to preserve that net H2 capacity. It selects **871 C (1600 F)** reformer outlet, **S/C = 3.0**, predicts **78.1% methane conversion** for the specified conventional reformer conditions, and specifies **88% PSA H2 recovery**. Its conventional **without-CC** Table-2 case gives **52.5 MMSCFD natural gas** and **3,205 short ton/day CO2 emitted**.

TEV-961 is a follow-up temperature-sensitivity study whose six compared cases all include carbon capture. It retains the 130 MMSCFD H2 service and the 871 C reformer target. Its Case 6 is **925 C reactor outlet -> 900 C supplied process heat** and states that this supplies the entire reforming heat duty in one reformer. Table 2 gives for Case 6:

- NG feed: **34.0 MMSCFD**
- H2: **130 MMSCFD**
- total process electricity: **17.3 MWe**
- process heat: **176.8 MWth**
- captured CO2: **1,927 short ton/day**
- emitted CO2: **142 short ton/day**.

**Boundary finding:** the repository correctly combines the TEV-953 conventional **without-CC** same-output baseline with the TEV-961 high-temperature **with-CC** Case 6 candidate. It does not confuse TEV-953's lower-temperature HTGR-with-CC case with Case 6. This is a cross-report comparator, but both reports use the same 130 MMSCFD service basis and TEV-961 explicitly follows TEV-953.

### JAEA / GTHTR300C basis

Primary GTHTR300C literature supports a **600 MWth**, **950 C** reactor with a **170 MWth** reference IHX and cogenerated electricity. Nishihara et al. (ICONE15, 2007) reports the mature one-module economic basis of **59.7 bn JPY**, **0.52 JPY/MJ heat**, **4.9 JPY/kWh electricity**, and **85% availability**; if the IHX and secondary-helium loop cost doubles, total plant cost becomes **70.9 bn JPY**, heat **0.57 JPY/MJ**, and electricity **5.5 JPY/kWh**.

The final design correctly uses 950 C as a JAEA hardware envelope while retaining the specific INL Case-6 ROT of 925 C.

## 2. Independent annualization

Declared availability:
[
h_y=0.85(8760)=7446 {m h/y}.
]

Using the source H2 rate of 29,000 lb/h:
[
m_{H2}=29000(0.45359237)(7446)/1000
=mathbf{97,946.015 t/y}.
]

Using 1 short ton = 0.90718474 metric tonne:

[
m_{CO2,captured}=1927(0.90718474)(365)(0.85)
=mathbf{542,361.984 t/y}.
]

[
m_{CO2,candidate}=142(0.90718474)(365)(0.85)
=mathbf{39,966.477 t/y}.
]

The annualization is internally consistent: the same 85% availability is applied to H2 and daily source carbon quantities.

## 3. Direct abatement

TEV-953 no-CC baseline:
[
E_{base,direct}=3205(0.90718474)(365)(0.85)
=mathbf{902,060.280 tCO2/y}.
]

TEV-961 Case-6 candidate:
[
E_{cand,direct}=mathbf{39,966.477 tCO2/y}.
]

Therefore:
[
A_{direct}=902060.280-39966.477
=mathbf{862,093.803 tCO2/y}.
]

Both source cases deliver 130 MMSCFD H2. Captured CO2 is not added to the avoided-emissions numerator; avoided direct emissions are calculated strictly as baseline emitted minus candidate emitted.

## 4. Lifecycle-abatement reconstruction

The frozen implementation is:
[
A_{LCA}=A_{direct}+
(E_{NG,up,base}-E_{NG,up,cand})
-E_{nuclear}-E_{aux}-E_{T&S}.
]

Natural-gas HHV conversion used by the model:
[
1 {m scf}=1044 {m Btu},qquad
1 {m Btu}=1.05505585262	imes10^{-6} {m GJ}.
]

With the declared IEA global-average gas-supply proxy 11.5 gCO2e/MJ:

- baseline upstream NG = **206,321.689 tCO2e/y**
- candidate upstream NG = **133,617.855 tCO2e/y**
- upstream reduction = **72,703.833 tCO2e/y**.

Other candidate burdens independently reproduced:

- nuclear process-heat lifecycle proxy = **3,649.207 tCO2e/y**
- incremental process-electricity lifecycle burden = **450.483 tCO2e/y**
- CCS transport/storage proxy = 2.5% x captured = **13,559.050 tCO2e/y**.

Thus:
[
A_{LCA}=862093.803+72703.833-3649.207-450.483-13559.050
=mathbf{917,138.896 tCO2e/y}.
]

This reproduces the reported approximately **917,139 tCO2e/y**.

The fact that lifecycle avoided emissions exceed direct avoided emissions is not a sign error: the candidate consumes substantially less natural gas (34.0 vs 52.5 MMSCFD), so its avoided upstream gas burden (**72.704 kt/y**) exceeds the added nuclear + auxiliary + T&S lifecycle burdens (**17.659 kt/y**). Net lifecycle increment above direct abatement is approximately **55.045 kt/y**.

### Lifecycle proxy qualification

The 11.5 gCO2e/MJ gas factor is a global IEA supply-chain anchor, not a Singapore route-specific inventory. The 5.5 gCO2e/kWh nuclear factor is a UNECE global-average electricity LCA. For direct heat the repository transparently derives a thermal proxy by multiplying the electricity factor by a 0.504 net-electric efficiency, giving about 2.77 gCO2e/kWh-th. This is explicitly documented in `equations/singapore_lifecycle_sensitivity.md` as a derived allocation proxy rather than a published heat-LCA value.

## 5. Heat/reactor compatibility

[
176.8/600=mathbf{0.29467}=29.47%.
]

Therefore one 600 MWth module has ample aggregate thermal capacity for the process duty.

However:
[
176.8-170=mathbf{6.8 MWth}
]
so the final duty is **4.0% above** the published 170 MWth reference IHX duty.

Using the JAEA doubled-IHX/secondary-loop **economic sensitivity** is defensible as a conservative screening cost response. It is not evidence that the reference 170 MWth exchanger itself is adequate, nor is the doubled-cost sensitivity an engineered 176.8 MWth exchanger design. The manuscript correctly states this limitation and does not claim completed exchanger engineering.

No aggregate reactor-thermal contradiction was found.

## 6. Final economic reconstruction

The final mature baseline uses the doubled-IHX JAEA source sensitivity and the repository's declared 2025-price/dated-FX normalization.

Reproduced converted screening values:

- JAEA doubled heat cost: **S$5.21577/GJ**
- JAEA doubled electricity cost: **S$50.3276/MWh**
- normalized doubled-IHX plant cost: **S$648.769 million**
- scaled CCS capital: **S$114.036 million**.

Annual cost components reproduced:

- nuclear heat burden: **S$24.719 million/y**
- nuclear electricity burden (202 MWe gross at source cost): **S$75.697 million/y**
- total reactor economic burden: **S$100.416 million/y**
- annualized CCS capital: **S$10.683 million/y**
- integration allowance annualization: **S$3.300 million/y**
- T&S at S$15/t captured: **S$8.135 million/y**
- baseline NG: **S$269.115 million/y**
- candidate NG: **S$174.284 million/y**.

The model therefore charges the full source-priced cogeneration burden rather than treating unused reactor capacity as free.

## 7. Zero-value cogeneration case

Set electricity value to exactly zero.

[
C_{inc,0}=
C_{NG,cand}+C_{reactor}+C_{CCS}+C_{integration}+C_{T&S}
-C_{NG,base}
]

[
=mathbf{S$27.7033 million/y}.
]

No electricity revenue/credit is present.

[
C_{abatement,0}
=27.7033{m M}/917138.896
=mathbf{S$30.206/tCO2e}.
]

Therefore the zero-value case independently satisfies:
[
30.206 < 100 {m S$/tCO2e}.
]

## 8. Central S$150/MWh case

Gross source cogeneration = 202 MWe.

Final process demand = 17.3 MWe.

[
P_{export}=202-17.3=mathbf{184.7 MWe}.
]

[
E_{export}=184.7(7446)=mathbf{1,375,276.2 MWh/y}.
]

At S$150/MWh:
[
V_{power}=mathbf{S$206.29143 million/y}.
]

This credit is applied once to the zero-value annual incremental cost:
[
C_{inc,150}=27.70325-206.29143
=mathbf{-S$178.58818 million/y}.
]

[
C_{abatement,150}
=-178.58818{m M}/917138.896
=mathbf{-S$194.723/tCO2e}.
]

The negative screening cost means only that candidate annual net cost is below baseline annual cost under the declared mature-design and electricity-value assumptions. It is not evidence of guaranteed project profit.

## 9. CN4252 threshold margins

Required:
- annual avoided emissions > 250,000 tCO2e/y
- abatement cost < S$100/tCO2e.

Final lifecycle result:
[
917138.896-250000
=mathbf{667,138.896 tCO2e/y}
]
above the abatement threshold.

Zero-value cost margin:
[
100-30.206
=mathbf{69.794 S$/tCO2e}
]
below the maximum allowed cost.

Central S$150/MWh case is still further below the cost threshold.

**The current final design therefore satisfies both CN4252 thresholds under its declared model assumptions, including in the zero-value cogeneration robustness case.**

## 10. Claim-strength assessment

PASS.

The active manuscript explicitly describes a passing result as a **CONDITIONAL MODEL RESULT** and states that it is not evidence of a presently existing Singapore commercial project. It identifies the major conditions: mature GTHTR300C design-study economics, enlarged IHX/secondary-loop treatment, cross-border CCS, lifecycle proxies, T&S assumptions and electricity-value assumptions.

It does not claim guaranteed profitability, demonstrated commercial feasibility, or proven technology preference.

## 11. Current-versus-historical check

PASS.

The active manuscript foregrounds the INL/NGNP 871 C / 925 C ROT / 900 C process-heat final design.

The historical 64-case / 0-joint-pass study and the Review-5 600 C state are explicitly identified as superseded historical research and retained as repository audit evidence. They are not presented as the operative final design.

## 12. Findings

### MINOR — direct-heat nuclear LCA remains a derived allocation proxy

The 5.5 gCO2e/kWh UNECE value is an electricity lifecycle result, not a direct-process-heat LCA. The repository converts it to approximately 2.77 gCO2e/kWh-th using a 0.504 electric-efficiency allocation. This is transparent and already labelled a proxy, but it is not a primary-source HTGR process-heat LCA.

This does **not** materially affect the headline threshold result. Even replacing the thermal proxy with the full 5.5 g/kWh applied directly per thermal kWh would add only about 3.6 ktCO2e/y relative to the present calculation, far smaller than the 667 kt/y abatement margin.

### MINOR — doubled-IHX source sensitivity is economic, not a completed capacity design

The JAEA source sensitivity doubles IHX/secondary-loop cost; it does not demonstrate a detailed 176.8 MWth exchanger design. The final duty exceeds the 170 MWth reference by 6.8 MWth. The repository appropriately treats this as a conservative screening economic response and explicitly disclaims completed exchanger engineering.

No BLOCKER or MAJOR finding was identified.

## 13. Independent verification conclusion

The primary-source process state, annualization, direct abatement, lifecycle reconstruction, reactor-duty screen, zero-value economic case, central S$150/MWh case, threshold margins and manuscript claim strength were independently checked rather than accepted from Rust test status.

The headline quantitative result is reproduced within rounding:
- annual H2: **97,946.015 t/y**
- direct avoided: **862,093.803 tCO2/y**
- lifecycle avoided: **917,138.896 tCO2e/y**
- zero-value incremental cost: **S$27.7033 million/y**
- zero-value abatement cost: **S$30.206/tCO2e**
- S$150/MWh incremental cost: **-S$178.5882 million/y**
- S$150/MWh abatement cost: **-S$194.723/tCO2e**.

The two MINOR qualifications are already substantially disclosed by the manuscript/repository and do not materially change the CN4252 joint-pass conclusion.

FINAL DESIGN VERIFIED — READY FOR FINAL SUBMISSION QA
