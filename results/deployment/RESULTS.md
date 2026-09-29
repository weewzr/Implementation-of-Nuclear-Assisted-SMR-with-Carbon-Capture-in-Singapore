# Singapore deployment-scale techno-economic extension — results

## Separation from original Gate 5
Original canonical result remains unchanged: approximately 74.85 ktH2/y, 64 cases, 0 joint CN4252 passes; conservative case fails both thresholds.

## Minimum scale
Using the verified best-positive-abatement physical corner, the minimum annual H2 service required to reach 0.25 MtCO2e/y is 96,077.794 tH2/y, 1.283638 times the original annual H2 output. The extension evaluates economic scenarios at 1.30 annual scale (97,302.488 tH2/y), giving 253,186.727 tCO2e/y avoided.

## Reactor/process scale
At 1.30 annual service:
- JAEA mature / FOAK 80% availability: 141.692 MW process-heat capacity;
- modern-central 93% availability: 121.885 MW process-heat capacity;
- one 600 MWth GTHTR300-class module is sufficient;
- JAEA mature thermal-service utilisation is about 23.6%;
- CO2 stored/transported screen: 653,718.615 t/y.

## Forward results — cogeneration allocation
| Scenario | Annual avoided (tCO2e/y) | S$/tCO2e | Joint status |
|---|---:|---:|---|
| JAEA mature design | 253,186.727 | 47.637 | CONDITIONAL SCENARIO PASS |
| Modern central (INL moderate) | 253,186.727 | 174.401 | COST FAIL |
| FOAK/adverse | 253,186.727 | 372.814 | COST FAIL |

The JAEA hydrogen-only allocation fails the cost threshold (about S$331/tCO2e by independent reconstruction), because the full 600 MWth module cannot assign unused capacity zero cost.

## Interpretation
A literature-supported conditional passing region exists, but it is narrow in economic assumptions. It requires:
- larger annual H2 deployment than the original Gate-5 basis;
- JAEA mature-design heat economics (0.7 JPY/MJ source value; S$6.09/GJ project conversion);
- useful cogeneration/capacity allocation for the remaining reactor output;
- low-end S$15/t captured CCS T&S scenario;
- 10% integration/site allowance screen.

The modern independent HTGR central and adverse cost regimes do not pass S$100/t at the same positive-abatement deployment scale. Therefore the extension does not establish commercial feasibility or a preferred Singapore technology.

## Break-even
For the mature low-T&S structure, the S$100/t boundary corresponds to approximately S$9.80/GJ delivered nuclear heat. The JAEA converted mature-design value lies below this boundary; the modern central/adverse effective cost structures do not.

## Reproducibility
- implementation: `model/src/deployment.rs`;
- generated data: `results/generated/deployment_*.csv`;
- literature basis: `results/deployment/LITERATURE_BASIS.md`;
- independent arithmetic: `results/deployment/SANITY_CHECKS.md`;
- manuscript: Section 11, `paper/sections/10a_deployment_extension.tex`.
