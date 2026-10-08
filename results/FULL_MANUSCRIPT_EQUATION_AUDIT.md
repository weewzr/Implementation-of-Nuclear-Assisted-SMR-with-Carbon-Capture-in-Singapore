# Full manuscript equation audit — current active compilation

## Scope and evidence discipline
Active order follows `paper/main.tex`; nested active inputs are `07_lifecycle_formulation.tex`, `09_economic_formulation.tex`, `11_limitations.tex`, `05_nuclear_feasibility_safety.tex`, and `10_singapore.tex`. Dormant `.tex` files are not treated as active claims. **This is a grouped equation ledger, not a new independently calibrated plant simulation.** Literature process states and project screening assumptions remain external inputs.

| Active section / labels | Principle, variables and dimensional check | Numerical reconciliation | Verdict / limitation |
|---|---|---|---|
| 01 Introduction | CN4252 targets compare annual tCO2e/y and S$/tCO2e separately | 0.25 Mt/y; S$100/t; not a licensing standard | PASS: screening targets only |
| 02 Background | SMR and WGS reaction atom balances CH4+H2O -> CO+3H2; CO+H2O -> CO2+H2 | C/H/O atoms conserved in both idealized reactions | PASS: equilibrium/kinetics not independently solved |
| 03 Design basis, two-train derivation | Q_total=NQ_train; mass and standard volume scale with identical trains; ratios dimensionless | 2*176.8=353.6 MWth; 2*97946=195892 t/y; 2*34=68 MMSCFD; 2*542362≈1.085 Mt/y; 353.6/370=0.955676; /600=0.589333 | PASS algebra; **linear identical-train duplication is a project assumption** |
| 04 Model formulation | INL source state at 871/900/925 C, S/C=3, PSA recovery=88% | Source inputs not first-principles predictions | TRACE ONLY; no kinetic/flowsheet re-solution |
| 05 Heat integration: reported temperature differences | ΔT=Thot−Tcold, Celsius differences equal K | 900−871=29 K; 925−900=25 K | PASS; neither is minimum local pinch |
| 05: helium heat balance | Qdot=mdot cp ΔT: kg/s * kJ/kg/K * K = kJ/s=kW | 176800/(78.49*(900−466))=5.19013 kJ/kg/K; cp=5.2 gives mdot≈78.34 kg/s | PASS source-state consistency; not independent IHX qualification |
| 05: reactor/branch fractions, remaining thermal | dimensionless Q/Q; Qreactor−Qprocess in MWth | Historical 176.8/600=0.294667; 600−176.8=423.2; 176.8/370=0.477838; preferred 353.6/600=0.589333 and 600−353.6=246.4 | PASS identities; **423.2 is historical one-train; 246.4 is conditional full-power envelope, not cooling/decay heat** |
| 05: physical IHX duty gap | MWth subtraction and dimensionless ratio | 176.8−170=6.8 MWth; 176.8/170=1.04 | PASS; reference 170 MWth is not 370 MWth source branch; design qualification absent |
| 06 Hydrogen: annualization | lb/h * kg/lb * h/y * t/kg = t/y | 97,946 t/y per train at 85% and 195,892 t/y for two identical trains | PASS against source-derived project model; actual annual availability unproven |
| 07 Carbon: annual short-ton conversion | short ton/d * 0.90718474 t/short ton * 365 d/y * 0.85 = t/y | Baseline 902060.28; candidate 39966.48; direct avoided 862093.80; captured 542361.98 t/y per train | PASS arithmetic; capture throughput ≠ avoided lifecycle emissions |
| 07: NG HHV and upstream emissions | MMSCFD * 1e6 scf/MMSCF * Btu/scf * GJ/Btu * d/y * kg/GJ /1000 | HHV 1044 Btu/scf and upstream 11.5 kgCO2e/GJ are declared screening inputs; baseline ~206322 and candidate ~133618 t/y | PASS dimensional form; source-region methane uncertainty not closed |
| 07: nuclear heat, auxiliary, T&S LCA | MWth *1000 kW/MW * h/y * kg/MWh-equivalent /1e6; captured t * tCO2e/t | 3649, 450, 13559 tCO2e/y per train; 862093.8+72703.8−3649.2−450.5−13559.0=917138.9 | PASS ledger to rounding; nuclear heat allocation 0.504 and T&S 0.025 are **project proxies**, not route-specific observations |
| 07: lifecycle intensity | tCO2e/y divided by tH2/y gives t/t=kg/kg | 191243.07/97946≈1.9525 kgCO2e/kgH2 | PASS per-train boundary; avoid double-counting captured CO2 |
| 08 Energy performance | thermal allocation only | preferred 600−353.6=246.4 MWth conditional envelope | CORRECTED earlier; no net electricity/exergy/UHS claim |
| 09: NG cost | GJ/d * d/y * S$/GJ = S$/y | 52.5 MMSCFD baseline ~S$269.115m/y; 34 candidate ~S$174.284m/y; difference ~S$94.831m/y | PASS historical one-train screen; gas tariff S$15/GJ assumption |
| 09: currency normalization | JPY/MJ * index * SGD/JPY * MJ/GJ = SGD/GJ; analogous kWh/MWh | 0.52*1.12732*.008117*1000≈4.758 SGD/GJ; 4.9*1.12732*.008117*1000≈44.837 SGD/MWh | PASS algebra; broad deflator/FX inputs are screening proxies, not nuclear cost index |
| 09: source-product cost | MW*h/y*GJ/MWh*S$/GJ = S$/y; MWe*h/y*S$/MWh likewise | historical 370 MWth and 88 MWe recover a published economic burden; **no electricity revenue** | PASS dimensions; historical bridge superseded by E1/E2B |
| 09: CCS CAPEX scaling | source EUR * dimensionless deflator * SGD/EUR * (t/y divided by t/y)^alpha | linear alpha=1 gives ~S$114.036m one-train CAPEX | PASS model substitution; no equipment scale-law validation |
| 09: CRF and annualization | CRF=i(1+i)^n/[(1+i)^n−1], with i per year, n years; annual factor 1/y | CRF(8%,25)=0.093678779; S$114.036m*CRF≈S$10.683m/y | PASS; discount/lifetime assumptions explicit |
| 09: integration and T&S | capital*CRF and t/y*S$/t | historical ~S$2.857m/y integration and ~S$8.135m/y T&S | PASS historical screening; not contracted Singapore prices |
| 09: historical abatement ratio | Δannual cost / lifecycle avoidance = S$/tCO2e | historical S$3.416m/y / 0.917139 Mt/y ≈ S$3.725/t | ARITHMETIC PASS; **superseded** by full-burden E1/E2B |
| 09: learning equation | C_N=C_1(1−LR)^(log2 N), cost units retained | 2184*(0.9)^log2(10)=1539.04 USD/kWt | PASS algebra; 10% learning is external scenario, not observed deployment |
| 09: mature cost numerator/denominator | (S$m/y)/(Mt/y)=S$/t | unrounded E2B ΔC≈78.583m/y /1.834278 Mt/y≈42.8414; displayed 616.814−538.230=78.584 due to rounded operands | **CORRECTED** displayed equality to approximate; retain unrounded model result |
| 10 Sensitivities | availability and captured-stream treatment as parameterized scenarios | historical 0.917 Mt/y, 23.2% threshold, 0.778 Mt/y and 48.35/t backup values belong to **one-train** model | CORRECTED historical label; do not transfer directly to preferred two-train |
| 11 CN4252 | 2*917139≈1.834278 Mt/y; S$/t compare threshold | FOAK 137.74 FAIL; BOAK 74.14 projected PASS; 10-OAK 42.84 projected PASS | PASS stage distinction; E2 0.461 comparator is different boundary |
| 12 Deployment | arrows are logical dependency, not numerical differential equations | no dates, risk probabilities or guaranteed maturity | PASS conceptual; site/licensing/CCS gates unresolved |
| 13 Conclusions | controlling numbers match E2B/E6 | 353.6 MWth; 195892 t/y; 1.834 Mt/y; 137.74/74.14/42.84 | PASS as **modelled/projected**, not observed |
| A/B Appendices | references, build traceability, no new physical equations | provenance paths and CN4252 targets traced | PASS conceptual; verify compiled references via CI |

## Carbon and energy accounting cautions
Captured CO2 (1.085 Mt/y at two trains) is a material throughput, **not** lifecycle abatement (1.834 MtCO2e/y). Baseline-minus-candidate stack emissions and upstream gas savings determine avoidance; capture is not added again. The 370/371 MWth branch is a source-level allocation, whereas ~170 MWth is a reference physical IHX rating. Neither 423.2 nor 246.4 MWth is an established ultimate-heat-sink design duty. No project EPZ radius is available.

## Verification status
The explicit arithmetic checks above use stated source inputs and declared project assumptions; they do not validate missing source physical properties, component design, dynamic safety, site risk, contracted costs or commercial availability. Rust CI and manuscript CI must be checked at the final commit; PDF page-by-page visual inspection requires the actual generated artifact.
