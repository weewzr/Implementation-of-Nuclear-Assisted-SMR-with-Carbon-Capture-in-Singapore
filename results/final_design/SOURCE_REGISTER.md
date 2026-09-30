# FINAL DESIGN SOURCE REGISTER

This evidence map keeps the INL process design, Nishihara/JAEA hardware economics and project-derived integration calculation separate.

| Parameter | Final value | Unit | Source | Classification | Transformation / use |
|---|---:|---|---|---|---|
| Reformer outlet | 871 | C | INL TEV-953/961 | SOURCE-BACKED | Selected INL process target |
| Steam/carbon | 3.0 | mol/mol | INL TEV-953 | SOURCE-BACKED | Process assumption |
| Reformer pressure | ~31.7 | bar | INL TEV-953 | SOURCE-BACKED | 460 psia feed condition |
| Methane conversion | 78.1 | % | INL TEV-953 | SOURCE-BACKED MODEL RESULT | Gibbs-minimization result |
| PSA recovery | 88 | % | INL TEV-953 | SOURCE-BACKED | Process specification |
| H2 product | 130 / 29,000 | MMSCFD / lb/h | INL TEV-953/961 | SOURCE-BACKED | Common output basis |
| INL HTGR outlet | 925 | C | INL TEV-961 Case 6 | SOURCE-BACKED | Final chemical-process case |
| Process helium supply / return | 900 / ~466 | C | INL TEV-961 Case 6 | SOURCE-BACKED MODEL RESULT | Process-side helium state |
| INL helium flow | 78.49 | kg/s | INL TEV-961 Case 6 | SOURCE-BACKED MODEL RESULT | Verification anchor |
| Nuclear process heat | 176.8 | MWth | INL TEV-961 Case 6 | SOURCE-BACKED MODEL RESULT | Final chemical-process duty |
| Process electricity | 17.3 | MWe | INL TEV-961 Case 6 | SOURCE-BACKED MODEL RESULT | Internal process demand |
| NG feed final / baseline | 34.0 / 52.5 | MMSCFD | INL TEV-961 / TEV-953 | SOURCE-BACKED MODEL RESULTS | Same H2 service |
| Captured / candidate emitted / baseline emitted CO2 | 1927 / 142 / 3205 | short t/day | INL TEV-961 / TEV-953 | SOURCE-BACKED MODEL RESULTS | Carbon basis |
| Nishihara reactor | 600 | MWth | Nishihara et al. 2007 | SOURCE-BACKED DESIGN | Mature economic architecture |
| Nishihara source hydrogen/IHX branch | 370 | MWth | Nishihara et al. 2007 | SOURCE-BACKED DESIGN | Economic-source configuration |
| Nishihara source power branch | 230 | MWth | 600-370 | SOURCE-DERIVED IDENTITY | Completes source thermal split |
| Nishihara source gross electricity | 88 | MWe | Nishihara et al. 2007 | SOURCE-BACKED DESIGN | At source 370/230 split |
| Source power conversion | 88/230 = 0.3826 | MWe/MWth | Nishihara source split | SOURCE-DERIVED | Used for project part-load mapping |
| Project process draw | 176.8 | MWth | INL duty mapped to JAEA reactor | PROJECT-DERIVED INTEGRATION | Below source 370 MWth heat branch |
| Project residual power branch | 423.2 | MWth | 600-176.8 | PROJECT-DERIVED | Not a published JAEA state |
| Project gross electricity | 161.92 | MWe | 423.2*(88/230) | PROJECT-DERIVED | Screening part-load mapping |
| Project net export | 144.62 | MWe | 161.92-17.3 | PROJECT-DERIVED | Before market-value scenario |
| Reference plant cost | 59.7 | bn JPY (2007) | Nishihara et al. 2007 | SOURCE-BACKED DESIGN STUDY | Principal mature case |
| Reference heat / electricity cost | 0.52 / 4.9 | JPY/MJ; JPY/kWh | Nishihara et al. 2007 | SOURCE-BACKED DESIGN STUDY | Principal mature case |
| Doubled loop-cost plant cost | 70.9 | bn JPY (2007) | Nishihara et al. 2007 | SOURCE-BACKED COST SENSITIVITY | Adverse cost sensitivity only |
| Doubled loop-cost heat / electricity | 0.57 / 5.5 | JPY/MJ; JPY/kWh | Nishihara et al. 2007 | SOURCE-BACKED COST SENSITIVITY | Not a capacity claim |
| Availability | 85 | % | Nishihara et al. 2007 | SOURCE-BACKED DESIGN STUDY | Mature economic assumption |
| Upstream NG lifecycle | 11.5 | gCO2e/MJ | IEA LNG assessment | SOURCE-BACKED GLOBAL PROXY | Not Singapore measurement |
| Nuclear lifecycle proxy | 5.5 | gCO2e/kWh | UNECE LCA | SOURCE-BACKED GLOBAL PROXY | Screening allocation |
| CCS capital | IEAGHG Case-1A incremental | EUR2014 | IEAGHG 2017-02 | SOURCE-BACKED | Scaled with captured throughput |
| T&S | 15 | S$/t captured | project scenario | PROJECT SCREENING ASSUMPTION | No authoritative Singapore tariff |
| Electricity value | 0; 100/150/200 | S$/MWh | zero-value test + EMA context | PROJECT SCENARIOS | Not guaranteed offtake |
